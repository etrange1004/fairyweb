use std::sync::Arc;

use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    http::StatusCode,
    response::{Html, IntoResponse}, 
    Router,
    routing::{get}, Extension,
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
            let _ = tx.send(format!("{} : {}", name, text));
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
                <div class=\"cont\"><textarea id=\"chat\" placeholder=\"채팅창\"></textarea></div>
                <div class=\"info\">
                    <dl><dt>대화명</dt>
                    <dd><input type=\"text\" id=\"username\" placeholder=\"입력후 버튼 클릭\" required autocomplete=\"off\"></dd></dl>
                    <dl><dt>채팅 참여</dt>
                    <dd><button type=\"button\" id=\"join-chat\">Join Chat</button></dd></dl>
                </div>
                <div class=\"title\"><dl><dt>메시지</dt>
                    <dd><input type=\"text\" id=\"input\" placeholder=\"채팅 메시지 입력\" required autocomplete=\"off\"></dd></dl>
                </div>
            </div>
        </div>
        <script>
            const username = document.querySelector(\"#username\");
            const join_btn = document.querySelector(\"#join-chat\");
            const textarea = document.querySelector(\"#chat\");
            const input = document.querySelector(\"#input\");
            
            join_btn.addEventListener(\"click\", function(e) {
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
                    textarea.value += e.data + \"\\r\\n\";
                }
                input.onkeydown = function(e) {
                    if ( e.key == \"Enter\" ) {
                        websocket.send(input.value);
                        input.value = \"\";
                    }
                }
            });
        </script>"
    );
    let container = Container::default().with_attributes([("class", "board_wrap")])
        .with_container(Container::default().with_attributes([("class", "board_title")])
            .with_raw(r#"<strong>요정들의 채팅방</strong>"#)
            .with_paragraph("요정님, 즐거운 채팅 시간 되세요!")
        )
        .with_raw(chatroom_rawstr.replace("HOME_URL", ctx.config.home_url.replace("http://", "ws://").as_str()));
    let resp_page = HtmlPage::new()
        .with_style(style::BOARD_CSS.to_string()).with_container(container).to_html_string();

    (StatusCode::OK, Html(resp_page))
}
