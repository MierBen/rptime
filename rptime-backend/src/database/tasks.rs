use diesel::RunQueryDsl;
use crate::database::Pool;
use crate::models::InsertTask;
use crate::utils::AppError;

pub fn import_tasks(tasks: Vec<InsertTask>, pool: &Pool) -> Result<usize, AppError> {
    use crate::models::schema::tasks;

    let conn = &pool.get().unwrap();
    info!("Add {} to database", tasks.len());
    diesel::insert_into(tasks::table)
        .values(&tasks)
        .execute(conn)
        .map_err(|err| AppError::ServiceError {
            cause: format!("Error from import_task() - {:?}", err),
        })
}