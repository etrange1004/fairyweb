use std::{collections::HashMap, io};
use axum::{
    body::Bytes,
    BoxError,
    extract::{Extension, Form, Query, Multipart},
    http::StatusCode,
    response::{Html, IntoResponse, Redirect},
    Router,
    routing::{get, post},
};
use tower_cookies::{Cookie, Cookies};
use build_html::{*, Html as BuildHtml};
use tokio::{io::BufWriter, fs::File};
use tokio_util::io::StreamReader;
use futures::{Stream, TryStreamExt};
use lettre::{
    message::{Message, MultiPart as le_Multipart},
    transport::smtp::{
        authentication::{Credentials, Mechanism},
        PoolConfig, response::Response,
    }, 
    SmtpTransport, Transport,
};

use crate::server::{errors, ApiContext, style, script};
use super::{models::{LoginUser, UpdateUser, UpdatePassword}, security::{hashed_password, verify_password}};

static UPLOAD_FILE_DIR: &str = "uploads";

pub fn router() -> Router {
    Router::new()
        .route("/login", get(login_form).post(login))
        .route("/signin", get(signin_form).post(signin))
        .route("/user", get(edit_user_form).post(edit_user))
        .route("/forget", get(forget_form).post(send_password))
        .route("/verifypw", post(verify_email))
}
async fn login_form(ctx: Extension<ApiContext>) -> impl IntoResponse {
    let mut login_form_str = String::new();
    login_form_str.push_str(&format!(
        "<div class=\"login-form\">
            <form method=\"post\" action=\"/login\">
                <input type=\"email\" class=\"text-field\" placeholder=\"E-Mail ID\" name=\"username\" required autocomplete=\"off\">          
                <input type=\"password\" class=\"text-field\" placeholder=\"Password\" name=\"password\" required>
                <input type=\"submit\" class=\"submit-btn\" value=\"login\"><div class=\"board_page\"><hr></div>
                <input type=\"button\" class=\"submit-btn\" onClick=\"location.href='HOME_URL/signin'\" value=\"Sign in\">
            </form>
            <div class=\"links\"><a href=\"HOME_URL/forget\">Do you forget your password?</a></div>
        </div>"
    ).replace("HOME_URL", ctx.config.home_url.as_str()));
    let resp_page = HtmlPage::new().with_style(style::LOGIN_CSS.to_string()).with_style(style::BOARD_CSS.to_string())
        .with_container(Container::default().with_raw(login_form_str)).to_html_string();
    (StatusCode::OK, Html(resp_page))
}
async fn login(ctx: Extension<ApiContext>, Form(input): Form<LoginUser>, cookies: Cookies) -> Result<Redirect, errors::CustomError> {
    let username = input.username.unwrap(); let password = input.password.unwrap();
    let mut transaction = ctx.db.begin().await.unwrap();
    let user = sqlx::query!("select id, name, email, pw from user where email = ?", username).fetch_one(&mut transaction).await;
    transaction.commit().await.unwrap();
    match user {
        Ok(record) => {
            // response and give jwt, cookie ..................
            if verify_password(password, record.pw).await {
                match cookies.get("userid") {
                    Some(_) => {
                        cookies.list().clear();
                    }
                    None => {}
                }
                cookies.add(Cookie::build("userid", record.id).http_only(true).expires(None).path("/").finish());
                cookies.add(Cookie::build("username", record.name).http_only(true).expires(None).path("/").finish());

                Ok(Redirect::to(&format!("{}/list?page_no=1&per_page=5", ctx.config.home_url)))
            } else {
                return Err(errors::CustomError::InvalidPasswordError);
            }
        },
        Err(_) => return Err(errors::CustomError::UserNotExistError(username)),
    }
}
async fn signin_form(ctx: Extension<ApiContext>) -> impl IntoResponse {
    let mut signin_form_str = String::new();
    signin_form_str.push_str(&format!(
        "<form action=\"HOME_URL/signin\" method=\"post\" name=\"signin\" enctype=\"multipart/form-data\">
            <div class=\"board_write\">
                <div class=\"title\"><dl><dt>EMail ID</dt>
                    <dd><input type=\"text\" name=\"email\" placeholder=\"EMail ID 입력\" required autocomplete=\"off\"></dd></dl></div>
                <div class=\"info\"><dl><dt>Name</dt>
                    <dd><input type=\"text\" name=\"name\" placeholder=\"이름 입력\" required autocomplete=\"off\"></dd></dl>
                <dl><dt>Password</dt>
                    <dd><input type=\"password\" name=\"password\" placeholder=\"비밀번호 입력\" required autocomplete=\"off\"></dd></dl></div>
                <div class=\"cont\"><textarea name=\"bio\" placeholder=\"자기소개 입력\" required autocomplete=\"off\"></textarea></div>
                <div class=\"title\"><dl><dt>Profile Image</dt>
                <dd><input type=\"file\" name=\"pimg\" placeholder=\"프로필사진\" required autocomplete=\"off\"></dd></dl></div>
            </div>
            <div class=\"bt_wrap\">
                <a href=\"javascript:signin_submit();\" class=\"on\">Sign In</a>
                <a href=\"HOME_URL/login\">Log In</a>
            </div>
        </form>"
    ).replace("HOME_URL", ctx.config.home_url.as_str()));
    let container = Container::default().with_attributes([("class", "board_wrap")])
        .with_container(Container::default().with_attributes([("class", "board_title")])
            .with_raw(r#"<strong>요정의 가입 신청서</strong>"#)
            .with_paragraph("요정님, EMail ID, 이름, 패스워드, 자기소개 등을 정확하게 등록해주세요!")
        )
        .with_container(Container::default().with_attributes([("class", "board_write_wrap")])
            .with_raw(signin_form_str)
        );
    let resp_page = HtmlPage::new().with_style(style::BOARD_CSS.to_string())
        .with_script_literal(script::SIGNIN_FORM_SCRIPT.to_string()).with_container(container).to_html_string();
    (StatusCode::OK, Html(resp_page))
}
async fn signin(ctx: Extension<ApiContext>, mut multipart: Multipart) -> Result<Redirect, errors::CustomError> {
    let mut form_data: HashMap<String, String> = HashMap::new();
    let mut filename = String::new();
    // email field should be first than file fields.
    while let Some(field) = multipart.next_field().await.unwrap() {    
        if let Some(file_name) = field.file_name() {
            form_data.insert(field.name().unwrap().to_string(), file_name.to_string());
            filename.push_str(file_name);
            stream_to_file(filename.as_str(), field).await.map_err(|_| errors::CustomError::FileUploadError).unwrap();
            filename.clear(); // 여러파일 쌉가능.... =ㅅ=;;
        } else {
            form_data.insert(field.name().unwrap().to_string(), field.text().await.unwrap());
            if form_data.get::<String>(&"email".to_string()).is_some() {
                let flag = form_data.get::<String>(&"email".to_string()).unwrap();
                match sqlx::query!("select name, email from user where email = ?", flag).fetch_one(&ctx.db).await {
                    Ok(rec) => return Err(errors::CustomError::UserExistsError(rec.email)),
                    Err(_) => {}
                }
            }            
            continue;
        }        
    }
    let email = form_data.get::<String>(&"email".to_string()).unwrap();    
    let password = hashed_password(form_data.get::<String>(&"password".to_string()).unwrap().to_string()).await;
    let mut transaction = ctx.db.begin().await.unwrap();
    let _ = sqlx::query!("insert into user(id, name, email, pw, permit, bio, pimg) values(uuid(), ?, ?, ?, 0, ?, ?)",
        form_data.get::<String>(&"name".to_string()).unwrap(),
        email,
        password,
        form_data.get::<String>(&"bio".to_string()).unwrap(),
        form_data.get::<String>(&"pimg".to_string()).unwrap()).execute(&mut transaction).await.unwrap();
    let record = sqlx::query!("select id from user where email = ?", email).fetch_one(&mut transaction).await.unwrap();
    transaction.commit().await.unwrap();
    Ok(Redirect::to(&format!("{}/user?id={}", ctx.config.home_url, record.id)))
}
async fn edit_user_form(ctx: Extension<ApiContext>, updateinfo: Option<Query<UpdateUser>>, cookies: Cookies) -> Result<impl IntoResponse, errors::CustomError> {
    let Query(updateinfo) = updateinfo.unwrap();
    let userid = match updateinfo.id {
        Some(id) => id,
        None => {
            cookies.get("userid").and_then(|c| c.value().parse::<String>().ok()).unwrap_or("3a9f71ec-414b-11ed-9da1-b42e99c05629".to_string())
        }
    };
    let mut transaction = ctx.db.begin().await.unwrap();
    let user = sqlx::query!("select name, email, pw, bio, pimg from user where id = ?", userid).fetch_one(&mut transaction).await;
    transaction.commit().await.unwrap();
    match user {
        Ok(record) => {
            let mut edit_user_form_str = String::new();
            edit_user_form_str.push_str(&format!(
                "<form action=\"HOME_URL/user\" method=\"post\" name=\"edituser\" enctype=\"multipart/form-data\">
                    <input type=\"hidden\" name=\"id\" value=\"{}\">
                    <div class=\"board_write\">
                        <div class=\"title\"><dl><dt>EMail ID</dt>
                            <dd><input type=\"text\" name=\"email\" placeholder=\"EMail ID 입력\" value=\"{}\" required autocomplete=\"off\"></dd></dl></div>
                        <div class=\"info\"><dl><dt>Name</dt>
                            <dd><input type=\"text\" name=\"name\" placeholder=\"이름 입력\" value=\"{}\" required autocomplete=\"off\"></dd></dl>
                        <dl><dt>Password</dt>
                            <dd><input type=\"password\" name=\"password\" placeholder=\"비밀번호 입력\" required autocomplete=\"off\"></dd></dl></div>
                        <div class=\"cont\"><textarea name=\"bio\" placeholder=\"자기소개 입력\" required autocomplete=\"off\">{}</textarea></div>
                        <div class=\"cont\"><img src=\"./uploads/{}\"></div>
                        <div class=\"title\"><dl><dt>Profile Image</dt>
                        <dd><input type=\"file\" name=\"pimg\" placeholder=\"프로필사진\" required autocomplete=\"off\"></dd></dl></div>
                    </div>
                    <div class=\"bt_wrap\">
                        <a href=\"javascript:edituser_submit();\" class=\"on\">Update Info</a>
                        <a href=\"HOME_URL/login\">Log In</a>
                        <a href=\"HOME_URL/list?id={}&name={}\">Fairy's Quest</a>
                        
                    </div>
                </form>", userid, record.email, record.name, record.bio.unwrap(), record.pimg.unwrap(), userid, record.name,
            ).replace("HOME_URL", ctx.config.home_url.as_str()));
            let container = Container::default().with_attributes([("class", "board_wrap")])
                .with_container(Container::default().with_attributes([("class", "board_title")])
                    .with_raw(r#"<strong>요정의 정보수정</strong>"#)
                    .with_paragraph("요정님, EMail ID, 이름, 패스워드, 자기소개 등을 수정해주세요!")
                )
                .with_container(Container::default().with_attributes([("class", "board_write_wrap")])
                    .with_raw(edit_user_form_str)
                );
            let resp_page = HtmlPage::new().with_style(style::BOARD_CSS.to_string())
                .with_script_literal(script::EDIT_USER_FORM_SCRIPT.to_string()).with_container(container).to_html_string();
            //let mut file = std::fs::File::create("edituser.html").unwrap();
            //std::io::copy(&mut resp_page.as_bytes(), &mut file).unwrap();
            //file.write_all(resp_page.as_bytes()).await.unwrap(); 
            Ok((StatusCode::OK, Html(resp_page)))// include_str!("../../edituser.html"))))
        }
        Err(_) => return Err(errors::CustomError::UserNotExistError("user".to_string())),
    }
}
async fn edit_user(ctx: Extension<ApiContext>, cookies: Cookies, mut multipart: Multipart) -> Result<Redirect, errors::CustomError> {
    let mut form_data: HashMap<String, String> = HashMap::new();
    let mut filename = String::new();
    // userid(email) field should be first than file fields.
    while let Some(field) = multipart.next_field().await.unwrap() {    
        if let Some(file_name) = field.file_name() {
            filename.clear(); // 여러파일 쌉가능.... =ㅅ=;;
            form_data.insert(field.name().unwrap().to_string(), file_name.to_string());
            filename.push_str(file_name);
            stream_to_file(filename.as_str(), field).await.map_err(|_| errors::CustomError::FileUploadError).unwrap();
        } else {
            form_data.insert(field.name().unwrap().to_string(), field.text().await.unwrap());
            if form_data.get::<String>(&"id".to_string()).is_some() && form_data.get::<String>(&"password".to_string()).is_some() {
                let userid = form_data.get::<String>(&"id".to_string()).unwrap();
                let password = form_data.get::<String>(&"password".to_string()).unwrap();
                let record = sqlx::query!("select pw from user where id = ?", userid).fetch_one(&ctx.db).await.unwrap(); 
                tracing::debug!("***********************userid: {}, password:{}, db pw:{} ", userid, password, record.pw);
                if !verify_password(password.to_string(), record.pw).await {
                    return Err(errors::CustomError::InvalidPasswordError);
                }
            }            
            continue;
        }        
    }
    let userid = form_data.get::<String>(&"id".to_string()).unwrap();
    let username = form_data.get::<String>(&"name".to_string()).unwrap();
    let email = form_data.get::<String>(&"email".to_string()).unwrap();
    let bio = form_data.get::<String>(&"bio".to_string()).unwrap();

    let mut tx = ctx.db.begin().await.unwrap();
    let _ = sqlx::query!("update user set email = ?, name = ?, bio = ?, pimg = ? where id = ?", 
        email, username.clone(), bio, filename, userid.clone()).execute(&mut tx).await.unwrap();
    tx.commit().await.unwrap();
    match cookies.get("userid") {
        Some(_) => {
            cookies.list().clear();
        }
        None => {}
    }
    cookies.add(Cookie::build("userid", userid.clone()).http_only(true).expires(None).path("/").finish());
    cookies.add(Cookie::build("username", username.clone()).http_only(true).expires(None).path("/").finish());
    Ok(Redirect::to(&format!("{}/list?page_no=1&per_page=5", ctx.config.home_url)))    
}
async fn forget_form() -> impl IntoResponse {
    let mut forget_form_str = String::new();
    forget_form_str.push_str(&format!(
        "<div class=\"login-form\">
            <form method=\"post\" action=\"/forget\">
                <input type=\"email\" class=\"text-field\" placeholder=\"패스워드를 받을 E-Mail 입력\" name=\"username\" required autocomplete=\"off\">          
                <input type=\"password\" class=\"text-field\" placeholder=\"New Password\" name=\"password\" required>
                <input type=\"submit\" class=\"submit-btn\" value=\"send password to E-mail\"><p></p>
            </form>
        </div>"
    ));
    let resp_page = HtmlPage::new().with_style(style::LOGIN_CSS.to_string()).with_style(style::BOARD_CSS.to_string())
        .with_container(Container::default().with_raw(forget_form_str)).to_html_string();
    (StatusCode::OK, Html(resp_page))    
}
async fn send_password(ctx: Extension<ApiContext>, Form(input): Form<LoginUser>) -> impl IntoResponse {
    let username = input.username.unwrap(); let password = input.password.unwrap();
    let url = ctx.config.home_url.clone();
    match sqlx::query!("select id, name, email, pw from user where email = ?", username).fetch_one(&ctx.db).await {
        Ok(record) => {
            let result = tokio::task::spawn_blocking(move || -> Result<Response, lettre::transport::smtp::Error> {
                // send email....
                let smtp_server = "smtp-relay.gmail.com";
                let smtp_username = "etrange1004@gmail.com";
                let smtp_password = "11111111111111111111";
                //let smtp_port = 587u16;
                let email = Message::builder()
                    .from("천사대장요정 <daijangfairy@fairyholdings.io>".parse().unwrap())
                    .to(username.parse().unwrap())
                    .subject("Change password and verify it after click link below.")
                    .multipart(le_Multipart::alternative_plain_html(
                        format!("To {} : Verify your email address to change password in fairyweb quest board.", record.name),
                        format!(
                            "<form action=\"HOME_URL/verifypw\" name=\"verifypw_form\" method=\"post\">
                                <input type=\"hidden\" name=\"id\" value=\"{}\">
                                <input type=\"hidden\" name=\"password\" value=\"{}\">
                            </form>
                            <p>Click link !!!<a href=\"javascript:verifypw_form.submit();\" target=\"_blank\">Verify^0^</a></p>",
                            record.id, password
                        ).replace("HOME_URL", url.clone().as_str()),
                    )).unwrap();
                let sender = SmtpTransport::starttls_relay(smtp_server)
                    .unwrap()
                    .credentials(Credentials::new(smtp_username.to_string(), smtp_password.to_string()))
                    .authentication(vec![Mechanism::Plain])
                    .pool_config(PoolConfig::new().max_size(20))
                    .build();
                sender.send(&email)
            }).await.map_err(|_| errors::CustomError::SendVerifyEmailError).unwrap();
            match result {
                Ok(_) => return Ok((
                    StatusCode::UNAUTHORIZED, Html(
                    format!("<script>alert(\"password sent to your input email. please login after veifying your email.\");
                        location.href=\"HOME_URL/login\";</script>").replace("HOME_URL", ctx.config.home_url.as_str()))
                )),
                Err(_) => return Err(errors::CustomError::SendVerifyEmailError),
            }
        },
        Err(_) => return Err(errors::CustomError::UserNotExistError(username)),
    }
}
async fn verify_email(ctx: Extension<ApiContext>, Form(input): Form<UpdatePassword>) -> Result<impl IntoResponse, errors::CustomError> {
    let id = input.id.unwrap(); let password = input.password.unwrap();
    let hashed_password = hashed_password(password).await;
    match sqlx::query!("update user set pw = ? where id = ?", hashed_password, id).execute(&ctx.db).await {
        Ok(_) => return Ok((
            StatusCode::OK, Html(
            format!("<script>alert(\"Success to verify email to change password.\");location.href=\"HOME_URL/login\";</script>")
            .replace("HOME_URL", ctx.config.home_url.as_str()))
        )),
        Err(e) => return Err(errors::CustomError::SqlxDatabaseError(e.to_string())),
    }
}
fn path_is_valid(path: &str) -> bool {
    let path = std::path::Path::new(&*path);
    let mut components = path.components().peekable();
    if let Some(first) = components.peek() {
        if !matches!(first, std::path::Component::Normal(_)) {
            return false;
        }
    }
    components.count() == 1
}
async fn stream_to_file<S, E>(path: &str, stream: S) -> Result<(), (StatusCode, String)> 
where S: Stream<Item = Result<Bytes, E>>, E: Into<BoxError> {
    if !path_is_valid(path) {
        return Err((StatusCode::BAD_REQUEST, "Invalid path".to_owned()));
    }
    async {
        let body_with_io_error = stream.map_err(|err| io::Error::new(io::ErrorKind::Other, err));
        let body_reader = StreamReader::new(body_with_io_error);
        futures::pin_mut!(body_reader);

        let path = std::path::Path::new(UPLOAD_FILE_DIR).join(path);
        let mut file = BufWriter::new(File::create(path).await?);
        tokio::io::copy(&mut body_reader, &mut file).await?;
        Ok::<_, io::Error>(())
    }.await.map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))
}