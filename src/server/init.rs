use axum::{
    extract::{Extension, Form},
    http::StatusCode,
    response::{Html, IntoResponse, Redirect},
    Router,
    routing::get,
};
use build_html::{*, Html as BuildHtml};
use crate::server::{errors, ApiContext, style};
use crate::server::models::LoginUser;

pub fn router() -> Router {
    Router::new()
        .route("/init", get(admin_init_form).post(init_setting))
}
async fn admin_init_form() -> impl IntoResponse {
    let mut login_form_str = String::new();
    login_form_str.push_str(
        "<div class=\"login-form\">
            <form method=\"post\" action=\"/init\">
                <input type=\"text\" class=\"text-field\" placeholder=\"ADMIN ID\" name=\"username\" required autocomplete=\"off\">          
                <input type=\"password\" class=\"text-field\" placeholder=\"Password\" name=\"password\" required>
                <input type=\"submit\" class=\"submit-btn\" value=\"admin login and database setting\">
            </form>
        </div>"
    );
    let resp_page = HtmlPage::new().with_style(style::LOGIN_CSS.to_string())
        .with_container(Container::default().with_raw(login_form_str)).to_html_string();
    (StatusCode::OK, Html(resp_page))
}
async fn init_setting(ctx: Extension<ApiContext>, Form(input) : Form<LoginUser>) -> Result<Redirect, errors::CustomError> {
    let id = input.username.unwrap();
    let pw = input.password.unwrap();
    if id == "admin".to_string() && pw == "0000".to_string() {
        sqlx::migrate!("./migrations")
        .run(&ctx.db)
        .await
        .map_err(|err| errors::CustomError::SqlxDatabaseError(err.to_string()))
        .unwrap();
        Ok(Redirect::to(&format!("{}/signin", ctx.config.home_url)))
    } else {
        return Err(errors::CustomError::AdminLoginError)
    }   
}