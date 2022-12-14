use std::sync::Arc;

use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    http::StatusCode,
    response::{Html, IntoResponse}, 
    Router,
    routing::get, Extension,
};
use futures::{StreamExt, SinkExt};
use build_html::{*, Html as OtherHtml};

use super::{ChatState, ApiContext, style};

pub fn router() -> Router {
    Router::new()
        .route("/chat", get(chat_start))
        .route("/chatsocket", get(chat_handler))
}
async fn chat_handler(ws: WebSocketUpgrade, Extension(state): Extension<Arc<ChatState>>) -> impl IntoResponse {
    ws.on_upgrade(|socket| chat_socket(socket, state))
}
async fn chat_socket(stream: WebSocket, state: Arc<ChatState>) {
    let (mut sender, mut receiver) = stream.split();
    let mut username = String::new();
    while let Some(Ok(message)) = receiver.next().await {
        if let Message::Text(name) = message {
            username.push_str(name.as_str());
            if !username.is_empty() {
                state.user_set.lock().unwrap().insert(name.to_owned());
                break;
            } else {
                let _ = sender.send(Message::Text(String::from("username already taken."))).await;
                return;
            }
        }
    }
    let mut rx = state.tx.subscribe();
    let msg = format!("{} joined in chat.", username);
    tracing::debug!("{}", msg);
    let _ = state.tx.send(msg);

    let mut send_task = tokio::spawn(async move {
        while let Ok(message) = rx.recv().await {
            if sender.send(Message::Text(message)).await.is_err() {
                break;
            }
        }
    });

    let tx = state.tx.clone();
    let name = username.clone();

    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(Message::Text(text))) = receiver.next().await {
            if text.contains("/quit") { break; }
            if text.contains(" is typing...") {
                let _ = tx.send(text);
            } else {
                let _ = tx.send(format!("{} : {}", name, text));
            }
        }
    });

    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    }

    let msg = format!("{} left the chat.", username);
    tracing::debug!("{}", msg);
    let _ = state.tx.send(msg);
    state.user_set.lock().unwrap().remove(&username);
}
async fn chat_start(ctx: Extension<ApiContext>) -> impl IntoResponse {
    let mut chatroom_rawstr = String::new();
    chatroom_rawstr.push_str(
        "<div class=\"board_write_wrap\">
            <div class=\"board_write\">
                <div class=\"cont\"><textarea id=\"chat\" placeholder=\"?????????\"></textarea></div>
                <div class=\"info\">
                    <dl><dt>?????????</dt>
                    <dd><input type=\"text\" id=\"username\" placeholder=\"????????? ?????? ??????\" required autocomplete=\"off\"></dd></dl>
                    <dl><dt>?????? ??????</dt>
                    <dd><button type=\"button\" id=\"joinchat\">Join Chat</button></dd></dl>
                </div>
                <div class=\"title\"><dl><dt>?????????</dt>
                    <dd><input type=\"text\" id=\"input\" placeholder=\"?????? ????????? ??????\" required autocomplete=\"off\"></dd></dl>
                </div>
            </div>
        </div>
        <script>
        const username = document.querySelector(\"#username\");
        const joinbtn = document.querySelector(\"#joinchat\");
        const textarea = document.querySelector(\"#chat\");
        const input = document.querySelector(\"#input\");

        var typing = false;
        var timeout = undefined;

        joinbtn.addEventListener(\"click\", function(e) {
            if ( username.value == \"\" ) {
                alert(\"???????????? ???????????????! ^0^\");
                return;
            }
            var chatuser = username.value;
            this.disabled = true;
            const websocket = new WebSocket(\"HOME_URL/chatsocket\");
            websocket.onopen = function() {
                console.log(\"connection opened.\");
                websocket.send(username.value);
            }
            const btn = this;
            websocket.onclose = function() {
                console.log(\"connection closed.\");
                btn.disabled = false;
            }
            websocket.onmessage = function(e) {
                console.log(\"received message: \" + e.data);
                if (e.data.includes(\" is typing...\")) {
                    document.title = e.data;
                } else {
                    textarea.value += e.data + \"\\r\\n\";
                }
            }
            input.onkeydown = function(e) {
                if ( e.key == \"Enter\" ) {
                    clearTimeout(timeout);
                    typingTimeout();
                    websocket.send(input.value);
                    input.value = \"\";
                }
                else {
                    typing = true;
                    websocket.send(chatuser + \" is typing...\");
                    clearTimeout(timeout);
                    timeout = setTimeout(typingTimeout, 3000);
                }
            }
        });
        </script>"
    );
    let container = Container::default().with_attributes([("class", "board_wrap")])
        .with_container(Container::default().with_attributes([("class", "board_title")])
            .with_raw(r#"<strong>???????????? ?????????</strong>"#)
            .with_paragraph("?????????, ????????? ?????? ?????? ?????????!")
        )
        .with_raw(chatroom_rawstr.replace("HOME_URL", ctx.config.home_url.replace("https://", "ws://").as_str()));
    let resp_page = HtmlPage::new().with_style(style::BOARD_CSS.to_string()).with_container(container).to_html_string();

    (StatusCode::OK, Html(resp_page))
}
