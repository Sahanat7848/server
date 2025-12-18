use std::result;
use std::sync::Arc;

use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{Router, routing::get};

use crate::application::use_cases;
use crate::domain::repositories::brawlers::BrawlerRepository;
use crate::domain::value_object::brawler_model::RegisterBrawlerModel;
use crate::infrastructure::database::repositories::brawlers::BrawlerPostgres;
use crate::{
    application::use_cases::brawlers::BrawlersUseCase,
    infrastructure::database::postgresql_connection::PgPoolSquad,
};

pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router {
    let brawler_repository = BrawlerPostgres::new(db_pool);
    let use_case = BrawlersUseCase::new(Arc::new(brawler_repository));

    Router::new()
    .route("/register", post(register))
    .with_state(Arc::new(use_case))
}

pub async fn register<T>(
    State(use_case): State<Arc<BrawlersUseCase<T>>>,
    Json(model): Json<RegisterBrawlerModel>,
) -> impl IntoResponse
where
    T: BrawlerRepository + Send + Sync,
{
    match use_case.register(model).await {
        Ok(user_id) => (StatusCode::CREATED, user_id.to_string()).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}
