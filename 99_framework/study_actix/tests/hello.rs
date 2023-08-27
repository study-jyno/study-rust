#[cfg(test)]
use super::*;
use actix_web::{
    http::{self, header::ContentType},
    test,
};

#[actix_web::test]
async fn test_hello_ok() {
    let req = test::TestRequest::default()
        .insert_header(ContentType::plaintext())
        .to_http_request();
    let resp = hello(req).await;
    assert_eq!(resp.status(), http::StatusCode::OK);
    assert_eq!(resp.Result(), "hello world");
}
