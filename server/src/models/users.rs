use sqlx::PgPool;

#[derive(sqlx::FromRow, Clone)]
pub struct User {
    pub id: i32,
}

impl User {
    pub async fn add_user(pool: &PgPool) -> Result<Self, sqlx::Error> {
        sqlx::query_as!(
            Self,
            "INSERT INTO users (id) 
                    VALUES (DEFAULT)
                    RETURNING id"
        )
        .fetch_one(pool)
        .await
        .map_err(|e| {
            log::error!("Error adding user: {}", e);
            e
        })
    }
}
