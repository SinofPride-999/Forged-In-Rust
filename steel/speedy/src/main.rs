use std::env;
use std::error::Error;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Instant;

use crossbeam_channel::{bounded, unbounded};
use ctrlc;
use indicatif::{ProgressBar, ProgressStyle};
use notify_rust::Notification;
use rayon::prelude::*;
use walkdir::WalkDir;
use num_cpus;

#[derive(Debug)]
enum SpeedyError {
    Io(io::Error),
    Parse(String),
    Argument(String),
    WalkDir(walkdir::Error),
    ThreadPoolBuild(rayon::ThreadPoolBuildError),
    Notification(notify_rust::error::Error),
    Ctrlc(ctrlc::Error),
    Template(String),
}

impl std::fmt::Display for SpeedyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SpeedyError::Io(e) => write!(f, "IO error: {}", e),
            SpeedyError::Parse(s) => write!(f, "Parse error: {}", s),
            SpeedyError::Argument(s) => write!(f, "Argument error: {}", s),
            SpeedyError::WalkDir(e) => write!(f, "Directory walk error: {}", e),
            SpeedyError::ThreadPoolBuild(e) => write!(f, "Thread pool error: {}", e),
            SpeedyError::Notification(e) => write!(f, "Notification error: {}", e),
            SpeedyError::Ctrlc(e) => write!(f, "Ctrl-C handler error: {}", e),
            SpeedyError::Template(e) => write!(f, "Template error: {}", e),
        }
    }
}

impl Error for SpeedyError {}

impl From<io::Error> for SpeedyError {
    fn from(e: io::Error) -> Self {
        SpeedyError::Io(e)
    }
}

impl From<walkdir::Error> for SpeedyError {
    fn from(e: walkdir::Error) -> Self {
        SpeedyError::WalkDir(e)
    }
}

impl From<rayon::ThreadPoolBuildError> for SpeedyError {
    fn from(e: rayon::ThreadPoolBuildError) -> Self {
        SpeedyError::ThreadPoolBuild(e)
    }
}

impl From<notify_rust::error::Error> for SpeedyError {
    fn from(e: notify_rust::error::Error) -> Self {
        SpeedyError::Notification(e)
    }
}

impl From<ctrlc::Error> for SpeedyError {
    fn from(e: ctrlc::Error) -> Self {
        SpeedyError::Ctrlc(e)
    }
}

fn main() -> Result<(), SpeedyError> {
    let start_time = Instant::now();

    let args: Vec<String> = env::args().collect();

    if args.len() == 1 || args[1] == "--help" {
        print_help();
        return Ok(());
    }

    if args.len() < 3 {
        println!("Usage:");
        println!("  speedy search:file <name> [--global]");
        println!("  speedy search:folder <name> [--global]");
        println!("  speedy search:file <name> [--path <custom_path>]");
        println!("Options:");
        println!("  --verbose       Show all warnings");
        println!("  --quiet         Suppress non-essential output");
        println!("  --depth <num>   Limit search depth (default: unlimited)");
        println!("  --notify        Show desktop notification when found");
        println!("  --threads <num> Set number of threads (default: CPU cores)");
        println!();
        println!("For more information, try 'speedy --help'");
        return Ok(());
    }

    let search_type = args[1].clone();
    let target = args[2].clone();
    let mut search_path = None;
    let mut is_global = false;
    let mut verbose = false;
    let mut quiet = false;
    let mut max_depth = usize::MAX;
    let mut notify = false;
    let mut num_threads = num_cpus::get();
    let mut stop_after_match = false;

    let mut i = 3;
    while i < args.len() {
        match args[i].as_str() {
            "--global" => {
                is_global = true;
                i += 1;
            }
            "--path" => {
                if i + 1 >= args.len() {
                    return Err(SpeedyError::Argument("Missing path after --path".to_string()));
                }
                search_path = Some(PathBuf::from(&args[i + 1]));
                i += 2;
            }
            "--verbose" => {
                verbose = true;
                i += 1;
            }
            "--quiet" => {
                quiet = true;
                i += 1;
            }
            "--depth" => {
                if i + 1 >= args.len() {
                    return Err(SpeedyError::Argument("Missing depth value after --depth".to_string()));
                }
                max_depth = args[i + 1]
                    .parse()
                    .map_err(|_| SpeedyError::Parse("Depth must be a number".to_string()))?;
                i += 2;
            }
            "--notify" => {
                notify = true;
                i += 1;
            }
            "--threads" => {
                if i + 1 >= args.len() {
                    return Err(SpeedyError::Argument("Missing thread count after --threads".to_string()));
                }
                num_threads = args[i + 1]
                    .parse()
                    .map_err(|_| SpeedyError::Parse("Thread count must be a number".to_string()))?;
                i += 2;
            }
            "--stop-after-match" => {
                stop_after_match = true;
                i += 1;
            }
            _ => {
                return Err(SpeedyError::Argument(format!("Unknown argument: {}", args[i])));
            }
        }
    }

    rayon::ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .build_global()?;

    let root_dir = match search_path {
        Some(path) => path,
        None => {
            if is_global {
                Path::new("C:\\").to_path_buf()
            } else {
                env::current_dir()?
            }
        }
    };

    if !root_dir.exists() {
        return Err(SpeedyError::Argument(format!(
            "Path does not exist: {}",
            root_dir.display()
        )));
    }

    if !quiet {
        println!(
            "🔍 Searching for {} \"{}\" in {}...",
            if search_type == "search:file" { "file" } else { "folder" },
            target,
            root_dir.display()
        );
        if max_depth != usize::MAX {
            println!("   (Depth limited to {} levels)", max_depth);
        }
    }

    let progress = if !quiet {
        let pb = ProgressBar::new_spinner();
        pb.set_style(
            ProgressStyle::default_spinner()
                .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
                .template("{spinner} Searching... {msg}")
                .map_err(|e| SpeedyError::Template(e.to_string()))?,
        );
        Some(pb)
    } else {
        None
    };

    let (found_tx, found_rx) = bounded(1);
    let (progress_tx, progress_rx) = unbounded();

    let cancelled = Arc::new(AtomicBool::new(false));
    let c = cancelled.clone();
    ctrlc::set_handler(move || {
        c.store(true, Ordering::SeqCst);
    })?;

    let root_dir_clone = root_dir.clone();
    let cancelled_clone = cancelled.clone();
    let progress_clone = progress.clone();
    let search_type_clone = search_type.clone();
    let target_clone = target.clone();

    let search_thread = std::thread::spawn(move || {
        let found = match search_type_clone.as_str() {
            "search:file" => parallel_search(
                &root_dir_clone,
                &target_clone,
                true,
                verbose,
                max_depth,
                &cancelled_clone,
                &found_tx,
                &progress_tx,
                stop_after_match,
            ),
            "search:folder" => parallel_search(
                &root_dir_clone,
                &target_clone,
                false,
                verbose,
                max_depth,
                &cancelled_clone,
                &found_tx,
                &progress_tx,
                stop_after_match,
            ),
            _ => Ok(false),
        };
        if let Some(pb) = progress_clone {
            pb.finish_and_clear();
        }
        found
    });

    if let Some(pb) = progress {
        while !search_thread.is_finished() {
            if let Ok(count) = progress_rx.try_recv() {
                pb.set_message(format!("Scanned {} locations", count));
            }
            pb.tick();
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    }

    let found = search_thread.join().unwrap()?;
    let elapsed = start_time.elapsed();

    if found {
        if let Ok(path) = found_rx.try_recv() {
            if !quiet {
                println!(
                    "\n🎯 Found matching {} at:",
                    if search_type == "search:file" { "file" } else { "folder" }
                );
                println!("   {}", path.display());
            }
            if notify {
                Notification::new()
                    .summary("Speedy Search")
                    .body(&format!("Found {}: {}", target, path.display()))
                    .show()?;
            }
        }
        if !quiet {
            println!("✅ Found \"{}\" in {:.2?}", target, elapsed);
        }
    } else if cancelled.load(Ordering::SeqCst) {
        if !quiet {
            println!("🛑 Search cancelled by user");
        }
    } else {
        if !quiet {
            println!("❌ Could not find \"{}\" after {:.2?}", target, elapsed);
            if !verbose && is_global {
                println!("ℹ️ Tip: Try with --verbose to see search progress or permission issues");
            }
        }
    }

    Ok(())
}

fn parallel_search(
    root: &Path,
    target: &str,
    search_files: bool,
    verbose: bool,
    max_depth: usize,
    cancelled: &Arc<AtomicBool>,
    found_tx: &crossbeam_channel::Sender<PathBuf>,
    progress_tx: &crossbeam_channel::Sender<usize>,
    stop_after_match: bool,
) -> Result<bool, SpeedyError> {
    let target = target.to_lowercase();
    let scanned = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let found = Arc::new(AtomicBool::new(false));

    let walker = WalkDir::new(root)
        .max_depth(max_depth)
        .follow_links(false)
        .into_iter()
        .filter_entry(|e| !should_skip_directory(e.path()))
        .filter_map(|e| {
            if cancelled.load(Ordering::SeqCst) || (found.load(Ordering::SeqCst) && stop_after_match) {
                return None;
            }

            match e {
                Ok(entry) => {
                    let count = scanned.fetch_add(1, Ordering::Relaxed) + 1;
                    if count % 500 == 0 {
                        let _ = progress_tx.send(count);
                    }
                    Some(entry)
                },
                Err(e) => {
                    if verbose && should_log_error(&e) {
                        eprintln!("⚠️ Could not access directory: {}", e);
                    }
                    None
                }
            }
        });

    let result = walker.par_bridge().find_any(|entry| {
        if cancelled.load(Ordering::SeqCst) || (found.load(Ordering::SeqCst) && stop_after_match) {
            return false;
        }

        let path = entry.path();
        let is_match = path.file_name()
            .and_then(|n| n.to_str())
            .map(|name| name.to_lowercase() == target)
            .unwrap_or(false);

        if is_match {
            if (search_files && path.is_file()) || (!search_files && path.is_dir()) {
                let _ = found_tx.send(path.to_path_buf());
                found.store(true, Ordering::SeqCst);
                true
            } else {
                false
            }
        } else {
            false
        }
    });

    Ok(result.is_some())
}

fn should_log_error(e: &walkdir::Error) -> bool {
    use std::io::ErrorKind;

    match e.io_error().map(|e| e.kind()) {
        Some(ErrorKind::PermissionDenied) => false,
        Some(ErrorKind::NotFound) => false,
        Some(ErrorKind::Interrupted) => false,
        _ => true,
    }
}

fn should_skip_directory(path: &Path) -> bool {
    if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
        let lower = name.to_lowercase();

        let skip_names = [
            "$recycle.bin", "system volume information", "windows", "program files",
            "program files (x86)", "appdata", "temp", "tmp", "node_modules", ".git",
        ];

        if skip_names.contains(&lower.as_str()) {
            return true;
        }
    }

    false
}

fn print_help() {
    println!("Speedy - A fast file and folder search tool");
    println!();
    println!("USAGE:");
    println!("  speedy search:file <name> [options]");
    println!("  speedy search:folder <name> [options]");
    println!();
    println!("OPTIONS:");
    println!("  --global           Search the entire system (default: current directory)");
    println!("  --path <path>      Search in a specific directory");
    println!("  --verbose          Show detailed search information and warnings");
    println!("  --quiet            Suppress non-essential output");
    println!("  --depth <num>      Limit search depth (default: unlimited)");
    println!("  --notify           Show desktop notification when found");
    println!("  --threads <num>    Set number of threads (default: CPU cores)");
    println!("  --stop-after-match Stop searching after first match is found");
    println!("  --help             Show this help message");
    println!();
    println!("EXAMPLES:");
    println!("  speedy search:file document.txt --global");
    println!("  speedy search:folder Projects --path ~/work");
    println!("  speedy search:file config.ini --depth 3 --notify");
    println!();
    println!("PERFORMANCE TIPS:");
    println!("  - Use --global only when necessary");
    println!("  - Limit search depth with --depth for faster results");
    println!("  - For large searches, use --threads to control CPU usage");
    println!("  - Use --stop-after-match when you only need the first result");
}

