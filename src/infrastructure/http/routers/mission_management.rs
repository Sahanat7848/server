use std::sync::Arc;

use axum::{Json, Router, extract::State, http::StatusCode, response::IntoResponse, routing::post};

use crate::{
    application::use_cases::mission_management::MissionmanagementUseCase,
    domain::repositories::brawlers::BrawlerRepository,
    infrastructure::{database::{postgresql_connection::PgPoolSquad, repositories::brawlers::BrawlerPostgres}, jwt::authentication_model::LoginModel},
};

pub async fn add<T1,T2>(
    State(user_case): State<Arc<MissionmanagementUseCase<T1,T2>>>,
    Extension(user_id): Extension<i32>,
    Json(model): Json<AddMissionModel>,
) -> impl IntoResponse
where
    T1: MissionManagementRepository + Send + Sync,
    T2: MissionViewingRepository + Send + Sync,
{
    match user_case.add(chief_id,user_id,model).await {
        Ok(mission_id) => (StatusCode::CREATED, mission_id.to_string()).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}


pub async fn edit<T1,T2>(
    State(user_case): State<Arc<MissionmanagementUseCase<T1,T2>>>,
    Extension(user_id): Extension<i32>,
    Path(mission_id): Path<i32>,
    Json(model): Json<EditMissionModel>,
) -> impl IntoResponse
where
    T1: MissionManagementRepository + Send + Sync,
    T2: MissionViewingRepository + Send + Sync,
{
    match user_case.edit(chief_id,user_id,model).await {
        Ok(mission_id) => (StatusCode::OK, format!("Edit mission: {} completed!!",mission_id)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn remove<T1,T2>(
    State(user_case): State<Arc<MissionmanagementUseCase<T1,T2>>>,
    Extension(user_id): Extension<i32>,
    Path(mission_id): Path<i32>,
) -> impl IntoResponse
where
    T1: MissionManagementRepository + Send + Sync,
    T2: MissionViewingRepository + Send + Sync,
{
    match user_case.remove(chief_id,user_id,model).await {
        Ok(mission_id) => (StatusCode::OK, format!("Remove mission_id : {} completed!!",mission_id)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router {
    let mission_repository = MissionManagementPostgres::new(Arc::clone(&db_pool));
    let viewing_repository = MissionViewingPostgres::new(Arc::clone(&db_pool));
    let use_case = MissionManagementUseCase::new(Arc::new(mission_repository),Arc::new(viewing_repository));

    Router::new()
        .route("/", post(add))
        .route("/{mission_id}/edit", patch(edit))
        .route("/{mission_id}/remove", delete(remove))
        .route_layer(middleware::from_fn(auth))
        .with_state(Arc::new(use_case))
}
