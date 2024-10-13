use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use futures::stream::TryStreamExt;
use models::todo::{Todo, TodoCreateRequest, TodoItem};
use mongodb::{bson::doc, options::IndexOptions, Client, Collection, IndexModel};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod models;

const DB_NAME: &str = "rust-todo";
const COLL_NAME: &str = "todos";

// App state to hold the to-do list
struct AppState {
    todos: Vec<TodoItem>,
}

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
        (status = 201, description = "Todo created successfully", body = TodoItem),
        (status = 400, description = "Invalid input")
    )
)]
async fn create_todo(
    client: web::Data<Client>,
    todo_request: web::Json<TodoCreateRequest>,
) -> impl Responder {
    let collection = client.database(DB_NAME).collection(COLL_NAME);

    // Create a new `TodoItem` and assign a UUID.
    let new_todo = Todo {
        title: todo_request.title.clone(),
        completed: false,
    };

    let result = collection.insert_one(new_todo.clone()).await;

    match result {
        Ok(_) => HttpResponse::Ok().body("user added"),
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

// Update a to-do item
#[utoipa::path(
    put,
    path = "/todos/{id}",
    request_body = TodoItem,
    responses(
        (status = 200, description = "Todo updated successfully", body = TodoItem),
        (status = 404, description = "Todo item not found")
    ),
    params(("id" = Uuid, description = "Todo item ID"))
)]
async fn update_todo(
    data: web::Data<AppState>,
    todo_id: web::Path<String>,
    updated_todo: web::Json<TodoItem>,
) -> impl Responder {
    let mut todos = data.todos.clone();
    if let Some(todo) = todos.iter_mut().find(|item| item.id == *todo_id) {
        todo.title = updated_todo.title.clone();
        todo.completed = updated_todo.completed;
        HttpResponse::Ok().json(todo)
    } else {
        HttpResponse::NotFound().body("Item not found")
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
async fn delete_todo(data: web::Data<AppState>, todo_id: web::Path<String>) -> impl Responder {
    let mut todos = data.todos.clone();
    if todos.iter().position(|item| item.id == *todo_id).is_some() {
        todos.retain(|item| item.id != *todo_id);
        HttpResponse::NoContent().finish()
    } else {
        HttpResponse::NotFound().body("Item not found")
    }
}

// Retrieve a to-do item by ID
#[utoipa::path(
    get,
    path = "/todos/{id}",
    responses(
        (status = 200, description = "Todo item found", body = TodoItem),
        (status = 404, description = "Todo item not found")
    ),
    params(("id" = Uuid, description = "Todo item ID"))
)]
async fn get_todo(data: web::Data<AppState>, todo_id: web::Path<String>) -> impl Responder {
    let todos = data.todos.clone();
    match todos.iter().find(|item| item.id == *todo_id) {
        Some(todo) => HttpResponse::Ok().json(todo),
        None => HttpResponse::NotFound().body("Item not found"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    /// Define OpenAPI documentation using Utoipa
    #[derive(OpenApi)]
    #[openapi(
        paths(health, create_todo, get_todos, update_todo, delete_todo, get_todo),
        components(schemas(TodoItem, TodoCreateRequest))
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
