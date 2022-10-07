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
        /*
        sqlx::query!("
            create database fairydb;
            use fairydb;
            create table user (
                id varchar(36) not null primary key,
                name varchar(30) not null,
                email varchar(100) not null,
                pw varchar(256) not null,
                date datetime not null default now(),
                permit tinyint unsigned default 0,
                bio text,
                pimg text
            );
            create table comment (
                number int unsigned not null auto_increment primary key,
                parent int unsigned not null,
                id varchar(36) not null,
                name varchar(30) not null,
                content text not null,
                date datetime not null default now()
            );
            create table board (
                number int unsigned not null auto_increment primary key,
                title varchar(150) not null,
                content text not null,
                id varchar(36) not null,
                name varchar(30) not null,
                password varchar(20) not null,
                date datetime not null default now(),
                hit int unsigned not null default 0
            );
        ")
        .execute(&ctx.db)
        .await
        .unwrap();
        */
        Ok(Redirect::to(&format!("{}/signin", ctx.config.home_url)))
    } else {
        return Err(errors::CustomError::AdminLoginError)
    }   
}