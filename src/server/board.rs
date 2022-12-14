use std::collections::HashMap;
use axum::{
    extract::{Extension, Form, Query, Multipart},
    http::StatusCode,
    response::{Html, IntoResponse, Redirect},
    Router,
    routing::{get, post},
};
use tower_cookies::Cookies;
use build_html::{*, Html as OtherHtml};

use crate::server::{ApiContext, style, errors};

use super::script::{EDIT_FORM_SCRIPT, COMMENT_FORM_SCRIPT, WRITE_FORM_SCRIPT};

pub fn router() -> Router {
    Router::new()
        .route("/list", get(board_list_from_get))
        .route("/view", get(board_view_from_get).post(board_view_from_post))
        .route("/edit", get(board_edit_from_get).post(board_edit_from_post))
        .route("/write", get(board_write_from_get).post(board_write_from_post))
        .route("/search", post(board_search_from_post))
}

#[derive(serde::Deserialize, Debug, Default)]
pub struct Pagination {
    pub page_no: Option<u32>,
    pub per_page: Option<u32>,
}
#[derive(serde::Serialize, Debug)]
struct ContentRecord {
    number: u32,
    title: String,
    content: String,
    writer_id: String,
    writer_name: String,
    date: String,
    hit: u32,
}
#[derive(serde::Deserialize, Debug)]
struct ContentEdited {
    number: Option<u32>,
    page_no: Option<u32>,
    id: Option<String>,
}
#[derive(serde::Deserialize, Debug)]
struct SearchInfo {
    page_no: Option<u32>,
    per_page: Option<u32>,
    keyword: Option<String>,
}
#[derive(serde::Deserialize, Debug)]
struct ContentInfo {
    number: Option<u32>,
    page_no: Option<u32>,
}
#[derive(serde::Deserialize, Debug)]
struct CommentInfo {
    parent: Option<u32>,
    id: Option<String>,
    name: Option<String>,
    content: Option<String>,
    page_no: Option<u32>,
}

async fn board_list_from_get(ctx: Extension<ApiContext>, pagination: Option<Query<Pagination>>) -> impl IntoResponse {
    let Query(pagination) = pagination.unwrap_or_default();
    let mut current_page = pagination.page_no.unwrap_or(1); if current_page <= 0 { current_page = 1; }
    let mut per_page = pagination.per_page.unwrap_or(5); if per_page > 100 { per_page = 100; }
    let rows = sqlx::query!("select number, title, id, name, date, hit from board order by date desc limit ?, ?",
        (current_page - 1) * per_page, per_page).fetch_all(&ctx.db).await.unwrap();
    let mut raw_str_dbrec = String::new();
    for row in rows.iter() {
        raw_str_dbrec.push_str(&format!(
            "<div>
                <div class=\"num\">{}</div>
                <div class=\"title\"><a href=\"HOME_URL/view?number={}&page_no={}\">{}</a></div>
                <div class=\"writer\">{}</div>
                <div class=\"date\">{}</div>
                <div class=\"count\">{}</div>
            </div>\n",
            row.number, row.number, current_page, row.title, row.name, row.date.to_string(), row.hit            
        ).replace("HOME_URL", ctx.config.home_url.as_str()));
    }
    let rec_count = u32::try_from(sqlx::query!("select count(*) as rec_count from board").fetch_one(&ctx.db).await.unwrap().rec_count).unwrap();
    let page_number = if rec_count % per_page == 0 { rec_count / per_page } else { rec_count / per_page + 1 };
    let prev_page = if current_page <= 1 { current_page } else { current_page - 1 };
    let next_page = if current_page + 1 > page_number { page_number } else { current_page + 1 };
    let mut raw_str_pagectrl = String::new();
    raw_str_pagectrl.push_str(&format!(
        "<div class=\"board_page\">
            <a href=\"HOME_URL/list?page_no=1&per_page={}\" class=\"bt first\"><<<</a>\n
            <a href=\"HOME_URL/list?page_no={}&per_page={}\" class=\"bt prev\"><<</a>\n
            <a href=\"#\" class=\"num on\">{} page</a>\n
            <a href=\"HOME_URL/list?page_no={}&per_page={}\" class=\"bt next\">>></a>\n
            <a href=\"HOME_URL/list?page_no={}&per_page={}\" class=\"bt last\">>>></a>\n
            <form action=\"HOME_URL/list\" method=\"get\" name=\"select_page_form\">
                <select id=\"select_per_page\" name=\"per_page\" onchange=\"javascript:select_page_form.submit();\">
                    <option value=\"5\">5</option><option value=\"10\">10</option>
                    <option value=\"15\">15</option><option value=\"20\">20</option>
                    <option value=\"50\">50</option><option value=\"100\">100</option>
                </select>
                <input type=\"hidden\" name=\"page_no\" value=\"{}\">                
            </form>
        </div>\n
        <div class=\"bt_wrap\">
            <a href=\"HOME_URL/write\" class=\"on\">????????????</a>
        </div>
        <script>
            var sel_per_page=document.getElementById(\"select_per_page\");
            sel_per_page.value=\"SEL_VAL\";
        </script>", 
        per_page, prev_page, per_page, current_page, next_page, per_page, page_number, per_page, current_page)
        .replace("HOME_URL", ctx.config.home_url.as_str()).replace("SEL_VAL", per_page.to_string().as_str()));

    let container = Container::default().with_attributes([("class", "board_wrap")])
        .with_container(Container::default().with_attributes([("class", "board_title")])
            .with_raw(r#"<strong>????????? ????????? ????????? ?????????</strong>"#)
            .with_paragraph("???????????? ????????? ???????????? ????????? ???????????? ?????????????????????.")
            .with_raw(&format!(
                "<form action=\"HOME_URL/search\" method=\"post\" name=\"search_form\">
                    <div class=\"board_write\">
                        <div class=\"title\"><dl><dt>?????? ?????? ??????</dt>  
                            <dd><input type=\"text\" name=\"keyword\" placeholder=\"????????? ??????\" required autocomplete=\"off\"
                                onKeyup=\"if(window.event.keyCode==13){{search_form.submit();}}\"></dd></dl>
                            <input type=\"hidden\" name=\"page_no\" value=\"{}\">
                            <input type=\"hidden\" name=\"per_page\" value=\"{}\">
                        </div>
                    </div>                   
                </form>", current_page, per_page
            ).replace("HOME_URL", ctx.config.home_url.as_str()))
        )
        .with_container(Container::default().with_attributes([("class", "board_list_wrap")])
            .with_container(Container::default().with_attributes([("class", "board_list")])
                .with_container(Container::default().with_attributes([("class", "top")])
                    .with_container(Container::default().with_attributes([("class", "num")]).with_raw(r#"??????"#))
                    .with_container(Container::default().with_attributes([("class", "title")]).with_raw(r#"??????"#))
                    .with_container(Container::default().with_attributes([("class", "writer")]).with_raw(r#"?????????"#))
                    .with_container(Container::default().with_attributes([("class", "count")]).with_raw(r#"?????????"#))
                    .with_container(Container::default().with_attributes([("class", "date")]).with_raw(r#"?????????"#)))
                .with_raw(raw_str_dbrec).with_raw(raw_str_pagectrl))
        );
    let resp_page = HtmlPage::new()
        .with_style(style::BOARD_CSS.to_string()).with_container(container).to_html_string();

    (StatusCode::OK, Html(resp_page))
}

async fn board_write_from_get(ctx: Extension<ApiContext>, cookies: Cookies) -> impl IntoResponse {
    let id = cookies.get("userid").and_then(|c| c.value().parse::<String>().ok()).unwrap_or("3a9f71ec-414b-11ed-9da1-b42e99c05629".to_string());
    let name = cookies.get("username").and_then(|c| c.value().parse::<String>().ok()).unwrap_or("????????????".to_string());
    let mut raw_str_wform = String::new();
    raw_str_wform.push_str(&format!(
        "\n<form action=\"HOME_URL/write\" method=\"post\" name=\"write_quest\" enctype=\"multipart/form-data\">
            <div class=\"board_write\">
                <div class=\"title\"><dl><dt>??????</dt>
                    <dd><input type=\"text\" name=\"title\" placeholder=\"?????? ??????\" required autocomplete=\"off\"></dd></dl></div>
                <div class=\"info\"><dl><dt>?????????</dt>
                    <dd><input type=\"text\" name=\"name\" placeholder=\"????????? ??????\" value=\"{}\" required autocomplete=\"off\"></dd></dl>
                <dl><dt>????????????</dt>
                    <dd><input type=\"password\" name=\"password\" placeholder=\"???????????? ??????\" required autocomplete=\"off\"></dd></dl></div>
                <div class=\"cont\"><textarea name=\"content\" placeholder=\"?????? ??????\" required autocomplete=\"off\"></textarea></div>
            </div>
            <div class=\"bt_wrap\">
                <a href=\"javascript:write_quest_submit();\" class=\"on\">??????</a>
                <a href=\"HOME_URL/list\">??????</a>
                <input type=\"hidden\" name=\"id\" value=\"{}\">
            </div>
        </form>", name, id
    ).replace("HOME_URL", ctx.config.home_url.as_str()));
    let container = Container::default().with_attributes([("class", "board_wrap")])
        .with_container(Container::default().with_attributes([("class", "board_title")])
            .with_raw(r#"<strong>????????? ????????? ?????????</strong>"#)
            .with_paragraph("?????????, ????????? ???????????? ????????? ???????????? ??????????????????!")
        )
        .with_container(Container::default().with_attributes([("class", "board_write_wrap")])
            .with_raw(raw_str_wform)
        );
    let resp_page = HtmlPage::new().with_style(style::BOARD_CSS.to_string())
        .with_script_literal(WRITE_FORM_SCRIPT.to_string()).with_container(container).to_html_string();

    (StatusCode::OK, Html(resp_page))    
}
async fn board_write_from_post(ctx: Extension<ApiContext>, mut multipart: Multipart) -> Result<Redirect, (StatusCode, String)> {
    let mut form_data: HashMap<String, String> = HashMap::new();
    let mut filename = String::new();
    // password and number field should be first than file fields.
    while let Some(field) = multipart.next_field().await.unwrap() {    
        if let Some(file_name) = field.file_name() {
            filename.clear(); // ???????????? ?????????.... =???=;;
            form_data.insert(field.name().unwrap().to_string(), file_name.to_string());
            filename.push_str(file_name);
            //stream_to_file(filename.as_str(), field).await.map_err(|_| errors::CustomError::FileUploadError).unwrap();
        } else {
            form_data.insert(field.name().unwrap().to_string(), field.text().await.unwrap());            
            continue;
        }        
    }
    let mut tx = ctx.db.begin().await.unwrap();   
    sqlx::query!("insert into board (title, content, id, name, password) values(?, ?, ?, ?, ?)",
        form_data.get::<String>(&"title".to_string()).unwrap().replace("<", "[").replace(">", "]"), 
        form_data.get::<String>(&"content".to_string()).unwrap().replace("<", "[").replace(">", "]").replace("\r\n", "<br>"), 
        form_data.get::<String>(&"id".to_string()).unwrap(), 
        form_data.get::<String>(&"name".to_string()).unwrap(), 
        form_data.get::<String>(&"password".to_string()).unwrap()).execute(&mut tx).await.unwrap();
    tx.commit().await.unwrap();
    Ok(Redirect::to(&format!("{}/list?page_no=1&per_page=5", ctx.config.home_url)))
}
async fn board_view_from_get(ctx: Extension<ApiContext>, cookies: Cookies, contentinfo: Option<Query<ContentInfo>>) -> impl IntoResponse {
    let Query(contentinfo) = contentinfo.unwrap();
    let number = contentinfo.number.unwrap();
    let current_page = contentinfo.page_no.unwrap();
    let id = cookies.get("userid").and_then(|c| c.value().parse::<String>().ok()).unwrap_or("3a9f71ec-414b-11ed-9da1-b42e99c05629".to_string());
    let name = cookies.get("username").and_then(|c| c.value().parse::<String>().ok()).unwrap_or("????????????".to_string());
    let row = sqlx::query!("select title, content, id, name, date, hit from board where number = ?", number)
        .fetch_one(&ctx.db)
        .await
        .unwrap();
    let mut raw_str_content = String::new();
    raw_str_content.push_str(&format!(
        "\n<div class=\"board_view_wrap\">
            <div class=\"board_view\">
                <div class=\"title\">{}</div>
                <div class=\"info\">
                    <dl><dt>??????</dt><dd>{}</dd></dl>
                    <dl><dt>?????????</dt><dd>{}</dd></dl>
                    <dl><dt>?????????</dt><dd>{}</dd></dl>
                    <dl><dt>??????</dt><dd>{}</dd></dl>
                </div>
                <div class=\"cont\">{}</div>
            </div>\n",  
        row.title, number, row.name, row.date, row.hit, row.content
    ));
    let comment_recs = sqlx::query!("select name, content, date from comment where parent = ? order by date asc", number)
        .fetch_all(&ctx.db)
        .await
        .unwrap();

    for rec in comment_recs.iter() {
        raw_str_content.push_str(&format!(
            "<div class=\"board_view\">
                <div class=\"info\"><dl><dt>?????????</dt><dd>{}</dd></dl><dl><dt>?????????</dt><dd>{}</dd></dl></div>
                <div class=\"cont\">{}</div>
            </div>\n", rec.name, rec.date, rec.content
        ));
    }
    raw_str_content.push_str(&format!(
        "<form action=\"HOME_URL/view\" method=\"post\" name=\"write_comment\">
            <div class=\"comment_write\">
                <div class=\"comment\"><textarea name=\"content\" placeholder=\"?????? ??????\" required autocomplete=\"off\"></textarea></div>
                <div class=\"info\">
                    <dl><dt><div class=\"comment_bt_wrap\"><a href=\"javascript:write_comment_submit();\" class=\"on\">??????</a></div></dt></dl>
                </div>                                
            </div>
            <input type=\"hidden\" name=\"parent\" value=\"{}\"><input type=\"hidden\" name=\"page_no\" value=\"{}\">
            <input type=\"hidden\" name=\"id\" value=\"{}\">
            <input type=\"hidden\" name=\"name\" value=\"{}\">
        </form>
            <div class=\"bt_wrap\">
                <a href=\"HOME_URL/list?page_no={}&per_page=5\" class=\"on\">??????</a>",
        number, current_page, id, name, current_page).replace("HOME_URL", ctx.config.home_url.as_str()));
    if row.id == id {
        raw_str_content.push_str(&format!(
                "<a href=\"HOME_URL/edit?number={}&id={}&page_no={}\">??????</a>", number, row.id, current_page)
                .replace("HOME_URL", ctx.config.home_url.as_str()));
    }
    raw_str_content.push_str(                
            "</div>
        </div>"
    );   
    let container = Container::default().with_attributes([("class", "board_wrap")])
        .with_container(Container::default().with_attributes([("class", "board_title")])
            .with_raw(r#"<strong>????????? ????????? ?????????</strong>"#)
            .with_paragraph("????????? ??????????????????. ????????? ??? ???????????????!")
        )
        .with_container(Container::default().with_attributes([("class", "board_view_wrap")])
            .with_raw(raw_str_content)
        );
    let resp_page = HtmlPage::new().with_style(style::BOARD_CSS.to_string())
        .with_script_literal(COMMENT_FORM_SCRIPT.to_string()).with_container(container).to_html_string();
    let mut tx = ctx.db.begin().await.unwrap();
    sqlx::query!("update board set hit = ? where number = ?", row.hit + 1, number).execute(&mut tx).await.unwrap();
    tx.commit().await.unwrap();
    (StatusCode::OK, Html(resp_page))    
}
async fn board_edit_from_get(ctx: Extension<ApiContext>, contentinfo: Option<Query<ContentEdited>>) -> impl IntoResponse {
    let Query(contentinfo) = contentinfo.unwrap();
    let number = contentinfo.number.unwrap();
    let current_page = contentinfo.page_no.unwrap_or(1);
    let id = contentinfo.id.unwrap();
    let row = sqlx::query!("select title, content, name, password from board where number = ?", number).fetch_one(&ctx.db).await.unwrap();
    let mut raw_str_eform = String::new();
    raw_str_eform.push_str(&format!(
        "\n<form action=\"HOME_URL/edit\" method=\"post\" name=\"edit_quest\" enctype=\"multipart/form-data\">
            <input type=\"hidden\" name=\"id\" value=\"{}\">
            <input type=\"hidden\" name=\"number\" value=\"{}\">
            <input type=\"hidden\" name=\"page_no\" value=\"{}\">
            <div class=\"board_write\">
                <div class=\"title\"><dl><dt>??????</dt>
                    <dd><input type=\"text\" name=\"title\" placeholder=\"?????? ??????\" value=\"{}\" required autocomplete=\"off\"></dd></dl></div>
                <div class=\"info\"><dl><dt>?????????</dt>
                    <dd>{}</dd></dl>
                <dl><dt>????????????</dt>
                    <dd><input type=\"password\" name=\"password\" placeholder=\"???????????? ??????\" required autocomplete=\"off\"></dd></dl></div>
                <div class=\"cont\"><textarea name=\"content\" placeholder=\"?????? ??????\" required autocomplete=\"off\">{}</textarea></div>
            </div>
            <div class=\"bt_wrap\">
                <a href=\"javascript:edit_quest_submit();\" class=\"on\">??????</a>
                <a href=\"HOME_URL/view?number={}&page_no={}\">??????</a>
                
            </div>
        </form>", id, number, current_page, row.title, row.name, row.content.replace("<br>", "\r\n"), number, current_page 
    ).replace("HOME_URL", ctx.config.home_url.as_str()));
    let container = Container::default().with_attributes([("class", "board_wrap")])
        .with_container(Container::default().with_attributes([("class", "board_title")])
            .with_raw(r#"<strong>????????? ????????? ?????? ?????????</strong>"#)
            .with_paragraph("?????????, ????????? ???????????? ????????? ???????????? ??????????????????!")
        )
        .with_container(Container::default().with_attributes([("class", "board_write_wrap")])
            .with_raw(raw_str_eform)
        );
    let resp_page = HtmlPage::new().with_style(style::BOARD_CSS.to_string())
        .with_script_literal(EDIT_FORM_SCRIPT).with_container(container).to_html_string();

    (StatusCode::OK, Html(resp_page))    
}
async fn board_edit_from_post(ctx: Extension<ApiContext>, mut multipart: Multipart) -> Result<Redirect, errors::CustomError> {
    let mut form_data: HashMap<String, String> = HashMap::new();
    let mut filename = String::new();
    // password and number field should be first than file fields.
    while let Some(field) = multipart.next_field().await.unwrap() {    
        if let Some(file_name) = field.file_name() {
            filename.clear(); // ???????????? ?????????.... =???=;;
            form_data.insert(field.name().unwrap().to_string(), file_name.to_string());
            filename.push_str(file_name);
            //stream_to_file(filename.as_str(), field).await.map_err(|_| errors::CustomError::FileUploadError).unwrap();
        } else {
            form_data.insert(field.name().unwrap().to_string(), field.text().await.unwrap());
            if form_data.get::<String>(&"number".to_string()).is_some() && form_data.get::<String>(&"password".to_string()).is_some() {
                let no = form_data.get::<String>(&"number".to_string()).unwrap().to_string();
                let password = form_data.get::<String>(&"password".to_string()).unwrap().to_string();
                let record = sqlx::query!("select id, password from board where number = ?", no).fetch_one(&ctx.db).await.unwrap(); 
                tracing::debug!("****************************************number: {}, password:{}, db pw:{} ", no, password, record.password);
                if record.password != password {
                    return Err(errors::CustomError::EditInvalidPasswordError((no, record.id)));
                }
            }            
            continue;
        }        
    }
    let mut tx = ctx.db.begin().await.unwrap();
    sqlx::query!("update board set title = ?, content = ? where number = ?",
        form_data.get::<String>(&"title".to_string()).unwrap().replace("<", "[").replace(">", "]"),
        form_data.get::<String>(&"content".to_string()).unwrap().replace("<", "[").replace(">", "]").replace("\r\n", "<br>"),
        form_data.get::<String>(&"number".to_string()).unwrap()).execute(&mut tx).await.unwrap();
    tx.commit().await.unwrap();    
    Ok(Redirect::to(&format!(
        "HOME_URL/view?number={}&id={}&page_no={}",
        form_data.get::<String>(&"number".to_string()).unwrap(),
        form_data.get::<String>(&"id".to_string()).unwrap(),
        form_data.get::<String>(&"page_no".to_string()).unwrap()).replace("HOME_URL", ctx.config.home_url.as_str())))
}
async fn board_view_from_post(ctx: Extension<ApiContext>, Form(input): Form<CommentInfo>) -> Result<Redirect, (StatusCode, String)> {
    let parent = input.parent.unwrap();
    let id = input.id.unwrap();
    let name = input.name.unwrap();
    let content = input.content.unwrap().replace("<", "[").replace(">", "]").replace("\r\n", "<br>");
    let page_no = input.page_no.unwrap();
    let mut tx = ctx.db.begin().await.unwrap();
    sqlx::query!("insert into comment(parent, id, name, content) values(?, ?, ?, ?)", parent, id, name, content)
        .execute(&mut tx).await.unwrap();
    tx.commit().await.unwrap();
    Ok(Redirect::to(&format!("HOME_URL/view?number={}&page_no={}", parent, page_no).replace("HOME_URL", ctx.config.home_url.as_str())))
}
async fn board_search_from_post(ctx: Extension<ApiContext>, Form(input): Form<SearchInfo>) -> impl IntoResponse {
    let mut current_page = input.page_no.unwrap_or(1); if current_page <= 0 { current_page = 1; }
    let mut per_page = input.per_page.unwrap_or(5); if per_page > 100 { per_page = 100; }
    let keyword = format!("%{}%", input.keyword.unwrap());
    tracing::debug!("select number, title, id, name, date, hit from board where title like {} or content like {} 
    order by date desc limit {}, {}", keyword, keyword, (current_page - 1) * per_page, per_page);
    let rows = sqlx::query!(
        "select number, title, id, name, date, hit from board where title like ? or content like ? 
        order by date desc limit ?, ?", keyword, keyword, (current_page - 1) * per_page, per_page)
        .fetch_all(&ctx.db).await.unwrap();
    let mut raw_str_dbrec = String::new();
    for row in rows.iter() {
        raw_str_dbrec.push_str(&format!(
            "<div>
                <div class=\"num\">{}</div>
                <div class=\"title\"><a href=\"HOME_URL/view?number={}&page_no={}\">{}</a></div>
                <div class=\"writer\">{}</div>
                <div class=\"date\">{}</div>
                <div class=\"count\">{}</div>
            </div>\n",
            row.number, row.number, current_page, row.title, row.name, row.date.to_string(), row.hit            
        ).replace("HOME_URL", ctx.config.home_url.as_str()));
    }
    let rec_count = u32::try_from(sqlx::query!("select count(*) as rec_count from board where title like ? or content like ?",
        keyword, keyword).fetch_one(&ctx.db).await.unwrap().rec_count).unwrap();
    let page_number = if rec_count % per_page == 0 { rec_count / per_page } else { rec_count / per_page + 1 };
    let prev_page = if current_page <= 1 { current_page } else { current_page - 1 };
    let next_page = if current_page + 1 > page_number { page_number } else { current_page + 1 };
    let mut raw_str_pagectrl = String::new();
    raw_str_pagectrl.push_str(&format!(
        "<div class=\"board_page\">
            <form method=\"post\" name=\"select_page_no\" action=\"HOME_URL/search\">
                <input id=\"page_no\" type=\"hidden\" name=\"page_no\">
                <input id=\"per_page\" type=\"hidden\" name=\"per_page\" value=\"{}\">
                <input id=\"keyword\" type=\"hidden\" name=\"keyword\" value=\"{}\">
            </form>
            <a href=\"javascript:document.select_page_no.page_no.value=1;select_page_no.submit();\" class=\"bt first\"><<<</a>\n
            <a href=\"javascript:document.select_page_no.page_no.value={};select_page_no.submit();\" class=\"bt prev\"><<</a>\n
            <a href=\"#\" class=\"num on\">{} page</a>\n
            <a href=\"javascript:document.select_page_no.page_no.value={};select_page_no.submit();\" class=\"bt next\">>></a>\n
            <a href=\"javascript:document.select_page_no.page_no.value={};select_page_no.submit();\" class=\"bt last\">>>></a>\n
            <form action=\"HOME_URL/search\" method=\"post\" name=\"select_per_page\">
                <select name=\"per_page\" onchange=\"javascript:select_per_page.submit();\">
                <option value=\"5\">5</option><option value=\"10\">10</option>
                <option value=\"15\">15</option><option value=\"20\">20</option>
                <option value=\"50\">50</option><option value=\"100\">100</option>
                </select>
                <input type=\"hidden\" name=\"page_no\" value=\"{}\">
                <input type=\"hidden\" name=\"keyword\" value=\"{}\">                
            </form>
        </div>\n
        <div class=\"bt_wrap\">
            <a href=\"HOME_URL/write\" class=\"on\">????????????</a>
            <a href=\"HOME_URL/list\" class=\"on\">????????????</a>
        </div>
        <script>
            var sel_per_page=document.getElementById(\"select_per_page\");
            sel_per_page.value=\"SEL_VAL\";
        </script>", 
        per_page, keyword, prev_page, current_page, next_page, page_number, current_page, keyword)
        .replace("HOME_URL", ctx.config.home_url.as_str()).replace("SEL_VAL", per_page.to_string().as_str()));

    let container = Container::default().with_attributes([("class", "board_wrap")])
        .with_container(Container::default().with_attributes([("class", "board_title")])
            .with_raw(r#"<strong>????????? ????????? ????????? ?????????</strong>"#)
            .with_paragraph("???????????? ????????? ???????????? ????????? ???????????? ?????????????????????.")
            .with_raw(&format!(
                "<form action=\"HOME_URL/search\" method=\"post\" name=\"search_form\">
                    <div class=\"board_write\">
                        <div class=\"title\"><dl><dt>?????? ?????? ??????</dt>  
                            <dd><input type=\"text\" name=\"keyword\" placeholder=\"????????? ??????\" required autocomplete=\"off\"
                                onKeyup=\"if(window.event.keyCode==13){{search_form.submit();}}\"></dd></dl>
                            <input type=\"hidden\" name=\"page_no\" value=\"{}\">
                            <input type=\"hidden\" name=\"per_page\" value=\"{}\">
                        </div>
                    </div>                   
                </form>", current_page, per_page
            ).replace("HOME_URL", ctx.config.home_url.as_str()))
        )
        .with_container(Container::default().with_attributes([("class", "board_list_wrap")])
            .with_container(Container::default().with_attributes([("class", "board_list")])
                .with_container(Container::default().with_attributes([("class", "top")])
                    .with_container(Container::default().with_attributes([("class", "num")]).with_raw(r#"??????"#))
                    .with_container(Container::default().with_attributes([("class", "title")]).with_raw(r#"??????"#))
                    .with_container(Container::default().with_attributes([("class", "writer")]).with_raw(r#"?????????"#))
                    .with_container(Container::default().with_attributes([("class", "count")]).with_raw(r#"?????????"#))
                    .with_container(Container::default().with_attributes([("class", "date")]).with_raw(r#"?????????"#)))
                .with_raw(raw_str_dbrec).with_raw(raw_str_pagectrl))
        );
    let resp_page = HtmlPage::new()
        .with_style(style::BOARD_CSS.to_string()).with_container(container).to_html_string();

    (StatusCode::OK, Html(resp_page))   
}