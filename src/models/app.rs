use sea_orm::DatabaseConnection;

pub struct _AppState {
    db: DatabaseConnection,
    jwt_secret: String,
}
