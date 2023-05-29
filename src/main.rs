use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/", web::get().to(index)))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

async fn index(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().body("Hello, World!")
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::body::to_bytes;
    use actix_web::{http, test};

    #[actix_web::test]
    async fn test_index_ok() {
        let req = test::TestRequest::default().to_http_request();
        let resp = index(req).await;

        // Check status code.
        assert_eq!(resp.status(), http::StatusCode::OK);

        // Check body.
        let bytes = to_bytes(resp.into_body()).await.unwrap();
        assert_eq!(std::str::from_utf8(&bytes).unwrap(), "Hello, World!");
    }
}
