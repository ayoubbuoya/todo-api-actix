use actix_web::{
    get,
    web::{self, Json, Path},
    App, HttpResponse, HttpServer, Responder,
};
use dotenv::dotenv;
use futures::stream::TryStreamExt;
use models::todo::{Todo, TodoCreateRequest};
use mongodb::{
    bson::{doc, oid::ObjectId},
    Client, Collection,
};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod models;

const DB_NAME: &str = "rust-todo";
const COLL_NAME: &str = "todos";

#[utoipa::path(
    get,
    path = "/health",
    responses((status = 200, description = "Return API is running!", body = String))
)]
#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().json("API is running!")
}

// Create a to-do item
#[utoipa::path(
    post,
    path = "/todos",
    request_body = TodoCreateRequest,
    responses(
        (status = 201, description = "Todo created successfully", body = Todo),
        (status = 400, description = "Invalid input")
    )
)]
async fn create_todo(
    client: web::Data<Client>,
    todo_request: web::Json<TodoCreateRequest>,
) -> impl Responder {
    let collection = client.database(DB_NAME).collection(COLL_NAME);

    // Create a new `Todo` and assign a UUID.
    let new_todo = Todo {
        title: todo_request.title.clone(),
        completed: false,
    };

    let result = collection.insert_one(new_todo.clone()).await;

    match result {
        Ok(_) => HttpResponse::Ok().json(new_todo),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

// Retrieve all to-do items
#[utoipa::path(
    get,
    path = "/todos",
    responses((status = 200, description = "List of to-do items", body = [Todo]))
)]
async fn get_todos(client: web::Data<Client>) -> impl Responder {
    let collection: Collection<Todo> = client.database(DB_NAME).collection(COLL_NAME);
    match collection.find(doc! {}).await {
        Ok(cursor) => {
            let todos: Vec<Todo> = cursor.try_collect().await.unwrap_or_else(|_| vec![]);
            if todos.is_empty() {
                HttpResponse::NotFound().body(format!("No users found"))
            } else {
                HttpResponse::Ok().json(todos)
            }
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

// Retrieve a to-do item by ID
#[utoipa::path(
    get,
    path = "/todos/{id}",
    responses(
        (status = 200, description = "Todo item found", body = Todo),
        (status = 404, description = "Todo item not found")
    ),
    params(("id" = Uuid, description = "Todo item ID"))
)]
async fn get_todo(client: web::Data<Client>, todo_id: Path<String>) -> impl Responder {
    let collection: Collection<Todo> = client.database(DB_NAME).collection(COLL_NAME);
    let object_id = match ObjectId::parse_str(&todo_id.into_inner()) {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().body("Invalid ID format"),
    };

    let filter = doc! { "_id": object_id };

    match collection.find_one(filter).await {
        Ok(Some(todo)) => HttpResponse::Ok().json(todo),
        Ok(None) => HttpResponse::NotFound().body("Item not found"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

// Update a to-do item
#[utoipa::path(
    put,
    path = "/todos/{id}",
    request_body = Todo,
    responses(
        (status = 200, description = "Todo updated successfully", body = Todo),
        (status = 404, description = "Todo item not found")
    ),
    params(("id" = Uuid, description = "Todo item ID"))
)]
async fn update_todo(
    client: web::Data<Client>,
    todo_id: Path<String>,
    updated_todo: Json<Todo>,
) -> impl Responder {
    let collection: Collection<Todo> = client.database(DB_NAME).collection(COLL_NAME);
    let object_id = match ObjectId::parse_str(&todo_id.into_inner()) {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().body("Invalid ID format"),
    };

    let filter = doc! { "_id": object_id };
    let update = doc! { "$set": { "title": updated_todo.title.clone(), "completed": updated_todo.completed } };

    match collection.update_one(filter, update).await {
        Ok(result) => {
            if result.matched_count > 0 {
                HttpResponse::Ok().json(updated_todo.into_inner())
            } else {
                HttpResponse::NotFound().body("Item not found")
            }
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

// Delete a to-do item
#[utoipa::path(
    delete,
    path = "/todos/{id}",
    responses(
        (status = 204, description = "Todo deleted successfully"),
        (status = 404, description = "Todo item not found")
    ),
    params(("id" = Uuid, description = "Todo item ID"))
)]
async fn delete_todo(client: web::Data<Client>, todo_id: Path<String>) -> impl Responder {
    let collection: Collection<Todo> = client.database(DB_NAME).collection(COLL_NAME);
    let object_id = match ObjectId::parse_str(&todo_id.into_inner()) {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().body("Invalid ID format"),
    };

    let filter = doc! { "_id": object_id };

    match collection.delete_one(filter).await {
        Ok(result) => {
            if result.deleted_count > 0 {
                HttpResponse::NoContent().finish()
            } else {
                HttpResponse::NotFound().body("Item not found")
            }
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    /// Define OpenAPI documentation using Utoipa
    #[derive(OpenApi)]
    #[openapi(
        paths(health, create_todo, get_todos, update_todo, delete_todo, get_todo),
        components(schemas(Todo, TodoCreateRequest))
    )]
    struct ApiDoc;

    // Retrieve the MongoDB URI or panic if it doesn't exist
    let database_url = std::env::var("MONGODB_URI").expect("MONGODB_URI must be set");

    println!("Connecting to MongoDB at {}...", database_url);

    // Attempt to connect to MongoDB and handle errors
    let client = match Client::with_uri_str(&database_url).await {
        Ok(client) => client,
        Err(err) => {
            eprintln!("Error connecting to MongoDB: {}", err);
            std::process::exit(1); // Exit the program with a non-zero status code
        }
    };

    println!("Connected to MongoDB successfully!");

    // Make instance variable of ApiDoc so all worker threads gets the same instance.
    let openapi = ApiDoc::openapi();

    println!("starting HTTP server at http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(client.clone()))
            .service(
                SwaggerUi::new("/swagger/{_:.*}").url("/api-docs/openapi.json", openapi.clone()),
            )
            .service(
                web::resource("/todos")
                    .route(web::get().to(get_todos))
                    .route(web::post().to(create_todo)),
            )
            .service(
                web::resource("/todos/{id}")
                    .route(web::get().to(get_todo))
                    .route(web::put().to(update_todo))
                    .route(web::delete().to(delete_todo)),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
