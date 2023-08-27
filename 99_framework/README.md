# 1. 시작

프로젝트 시작

```shell
➜  99_framework git:(main) cargo new framwork
     Created binary (application) `framwork` package
```

package 설치 - https://github.com/actix/actix-web

Cargo.toml 에 아래 추가

```toml
Dependencies:

[dependencies]
actix-web = "4"
```

framework folder 로 이동 후 추가한 dependence 설치

```shell
cargo build
```

이제 이 문서 토대로 진행

- https://actix.rs/docs/
- https://actix.rs/docs/getting-started

# 2. actix-web 시작

1. src/main.rs 에 아래 코드 추가

```rust
   use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
```

2. 서버 실행

```shell
cargo run
```

3. 서버 접속

새 터미널을 열어서 curl 요청 해보자

```shell
➜  study-rust git:(main) ✗ curl http://127.0.0.1:8080/
Hello world!
```

성공했다!

4. test 어떻게 함?

- https://actix.rs/docs/testing#unit-tests
- test code 에서 hello 함수 어떻게 호출함? - 처음부터 다시봐야겠다
