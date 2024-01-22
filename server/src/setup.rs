use sea_orm::*;


pub(super) async fn set_up_db(database_url: &str, database_name: &str) -> Result<DatabaseConnection, DbErr> {
    let db = Database::connect(database_url).await?;

    let db = match db.get_database_backend() {
        DbBackend::MySql => {
            db.execute(Statement::from_string(
                db.get_database_backend(),
                format!("CREATE DATABASE IF NOT EXISTS `{}`;", database_name),
            ))
            .await?;

            let url = format!("{}/{}", database_url, database_name);
            Database::connect(&url).await?
        }
        DbBackend::Postgres => {
            let find_database_result: Vec<QueryResult> = db.query_all(Statement::from_string(
                db.get_database_backend(),
                format!("SELECT FROM pg_database WHERE datname = '{}';", database_name),
            ))
            .await?;

            if find_database_result.is_empty() {
                db.execute(Statement::from_string(
                    db.get_database_backend(),
                    format!("CREATE DATABASE \"{}\";", database_name),
                ))
                .await?;
            }

            let url = format!("{}/{}", database_url, database_name);
            Database::connect(&url).await?
        }
        DbBackend::Sqlite => db,
    };

    Ok(db)
}