use sea_orm::{DatabaseConnection, EntityTrait, ActiveModelTrait, Set};
use crate::entity::tasks;

pub async fn add_task(db: &DatabaseConnection, title: &str, description: Option<&str>) -> Result<(), sea_orm::DbErr> {
    let task = tasks::ActiveModel {
        title: Set(title.to_string()),
        description: Set(description.map(|s| s.to_string())),
        status: Set("open".to_string()),
        ..Default::default()
    };
    task.insert(db).await?;
    Ok(())
}

pub async fn list_tasks(db: &DatabaseConnection) -> Result<Vec<tasks::Model>, sea_orm::DbErr> {
    tasks::Entity::find().all(db).await
}

pub async fn complete_task(db: &DatabaseConnection, task_id: i32) -> Result<(), sea_orm::DbErr> {
    if let Some(task) = tasks::Entity::find_by_id(task_id).one(db).await? {
        let mut active: tasks::ActiveModel = task.into();
        active.status = Set("done".to_string());
        active.update(db).await?;
    }
    Ok(())
}

pub async fn delete_task(db: &DatabaseConnection, task_id: i32) -> Result<(), sea_orm::DbErr> {
    if let Some(task) = tasks::Entity::find_by_id(task_id).one(db).await? {
        let active: tasks::ActiveModel = task.into();
        active.delete(db).await?;
    }
    Ok(())
}
