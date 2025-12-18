use axum::{http::StatusCode, middleware::Next, response::Response};

pub async fn auth (mut req: Request, next: Next) -> Result<Response, StatusCode> {
    let header = req.headers()
    .get(header::AUTHORIZATION)
    .and_then(|value| value.to_str()
    .ok()).ok_or(StatusCode::UNAUTHORIZED)?;

    let token = header
    .strip_prefix("Bearer ")
    .ok_or(StatusCode::UNAUTHORIZED)?;

    let secret = get_user_secret().map_err(|_| StatusCode::UNAUTHORIZED)?;

    let claims = verify_token(token, secret).map_err(|_| StatusCode::UNAUTHORIZED)?;

    let user_id = claims.sub.parse::<i32>().map_err(|_| StatusCode::UNAUTHORIZED)?;

    req.extensions_mut().insert(user_id);

    Ok(next.run(req).await)
}
        