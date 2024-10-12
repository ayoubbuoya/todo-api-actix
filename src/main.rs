use actix_web::{get, post,  App, HttpResponse, HttpServer, Responder};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[utoipa::path(
    get, 
    path = "/health",
    responses(
        (status = 200, description = "Return API is running!", body = String),
    )
)]
#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().json("API is running!")
}

#[get("/")]
async fn hello() -> impl Responder {    
    HttpResponse::Ok().body("Hello world!")
}

#[utoipa::path(
    post,
    path = "echo",
    request_body = String,
    responses(
        (status = 201, description = "Todo created successfully", body = String),
        (status = 400, description = "Invalid input")
    )
)]
#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    /// Define OpenAPI documentation using Utoipa
    #[derive(OpenApi)]
    #[openapi(
        paths(health)
    )]
    struct ApiDoc;

    // Make instance variable of ApiDoc so all worker threads gets the same instance.
    let openapi = ApiDoc::openapi();


    println!("starting HTTP server at http://localhost:8080");


    HttpServer::new(move || {
        App::new().service(
            SwaggerUi::new("/swagger/{_:.*}").url("/api-docs/openapi.json", openapi.clone()),
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await

    


}
