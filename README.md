# FairyWeb 0.1.0

This is a simple web board and web chat tool written in rust. As Rust libraries, Axum, Sqlx, Tokio, Tower, etc. were used. The html, css, and javascript files do not exist, and they are created in the code without reading the file and output directly to the client browser.

이것은 Rust로 작성된 간단한 웹 보드 및 웹 채팅 도구입니다. Rust 라이브러리로는 Axum, Sqlx, Tokio, Tower 등이 사용되었습니다. html, css, javascript 파일은 존재하지 않으며 파일을 읽지 않고 코드에서 생성되어 클라이언트 브라우저에 직접 출력됩니다.

# Getting started

Compiling and running FairyWeb requires pre-work.

1. Requires installation of MySql 8.0.27 or higher.
2. Create fairydb database and create user ID/password account with chachafairy / 0000. 
   (Please refer to the DATABASE_URL string in the .env file)
3. Execute FairyBoard.sql to create user, comment, and board table.
4. Compile and run the source file with cargo run.
5. Enter http://localhost:8080/ in the browser address bar =ㅅ=

FairyWeb을 컴파일하고 실행하려면 사전작업이 필요합니다.

1.  MySql 8.0.27 이상 버전의 설치가 필요함.
2. fairydb 데이터베이스를 생성하고 chachafairy / 0000로 사용자아이디 / 패스워드 계정을 생성함. (.env 파일의 DATABASE_URL 스트링을 참조하기 바람)
3. FairyBoard.sql 을 실행해 user, comment, board table 을 생성해주세요.
4. cargo run으로 소스파일을 컴파일하고 실행합니다.
5. 크롬이나 파이어폭스 브라우저를 실행하고 주소 입력창에 http://localhost:8080/ 입력합니다 =ㅅ=

## Requirements
MySQL 8.0.27 or higher
 
## Notice

The send_password function, which sends mail to change the user's password in the user.rs file, does not work because it does not receive authentication from smtp_server. If smtp_server authentication is possible, enter information and use it. ^0^n

user.rs 파일의 사용자 패스워드 변경을 위해 메일을 발송하는 send_password 함수는 smtp_server의 인증을 받지못해 작동하지 않기 때문에 smtp_server 인증이 가능하시면 정보를 입력하시고 사용하세요. ^0^n 

## License

This project is licensed under the [MIT license].

[MIT license]: LICENSE