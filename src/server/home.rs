use axum::{
    handler::Handler,
    http::StatusCode,
    response::{Html, IntoResponse},
    Router, 
    routing::get,
};

pub fn router() -> Router {
    Router::new()
        .fallback(fallback.into_service())
        .route("/", get(root))
}
async fn fallback(uri: axum::http::Uri) -> impl IntoResponse {
    (StatusCode::NOT_FOUND, format!("No route {}", uri))
}

async fn root() -> Html<&'static str> {
    Html(
        "<div align=\"center\">
            <h1>상도동아저씨요정의 퀘스트 카페방</h1>
            <h3>착하고 귀여운 요정들은 로그인해주세요, 아저씨가 쿠키를 줍니다.</h3>
            <a href=\"/login\">요정이들 입장</a>
        </div>"
    )
}