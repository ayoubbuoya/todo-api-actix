use actix_web::{get,  web, App, HttpResponse, HttpServer, Responder};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use std::sync::Mutex;
use models::todo::{TodoCreateRequest, TodoItem};

mod models;


// App state to hold the to-do list
struct AppState {
    todos: Vec<TodoItem>,
}

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
    data: web::Data<AppState>,
    todo_request: web::Json<TodoCreateRequest>,
) -> impl Responder {
    let mut todos = data.todos.clone();
    
    // Create a new `TodoItem` and assign a UUID.
    let new_todo = TodoItem {
        id: "4".to_string(),
        title: todo_request.title.clone(),
        completed: todo_request.completed,
    };
    
    todos.push(new_todo.clone());

    HttpResponse::Created().json(new_todo)
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
    params(
        ("id" = Uuid, description = "Todo item ID")
    )
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
    params(
        ("id" = Uuid, description = "Todo item ID")
    )
)]
async fn delete_todo(
    data: web::Data<AppState>,
    todo_id: web::Path<String>,
) -> impl Responder {
    let mut todos = data.todos.clone();
    if todos.iter().position(|item| item.id == *todo_id).is_some() {
        todos.retain(|item| item.id != *todo_id);
        HttpResponse::NoContent().finish()
    } else {
        HttpResponse::NotFound().body("Item not found")
    }
}


// Retrieve all to-do items
#[utoipa::path(
    get,
    path = "/todos",
    responses(
        (status = 200, description = "List of to-do items", body = [TodoItem])
    )
)]
async fn get_todos(data: web::Data<AppState>) -> impl Responder {
    let todos = data.todos.clone();
    HttpResponse::Ok().json(&*todos)
}

// Retrieve a to-do item by ID
#[utoipa::path(
    get,
    path = "/todos/{id}",
    responses(
        (status = 200, description = "Todo item found", body = TodoItem),
        (status = 404, description = "Todo item not found")
    ),
    params(
        ("id" = Uuid, description = "Todo item ID")
    )
)]
async fn get_todo(
    data: web::Data<AppState>,
    todo_id: web::Path<String>,
) -> impl Responder {
    let todos = data.todos.clone();
    match todos.iter().find(|item| item.id == *todo_id) {
        Some(todo) => HttpResponse::Ok().json(todo),
        None => HttpResponse::NotFound().body("Item not found"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    /// Define OpenAPI documentation using Utoipa
    #[derive(OpenApi)]
    #[openapi(
        paths(health,  
            create_todo,
            get_todos, 
            update_todo,
            delete_todo, 
            get_todo
        ), 
        components(schemas(TodoItem, TodoCreateRequest)),
    )]
    struct ApiDoc;

    // Make instance variable of ApiDoc so all worker threads gets the same instance.
    let openapi = ApiDoc::openapi();


    println!("starting HTTP server at http://localhost:8080");


    HttpServer::new(move || {
        App::new()
        .app_data(web::Data::new(AppState {
            todos: Vec::new(),
        }))
        .service(
            SwaggerUi::new("/swagger/{_:.*}").url("/api-docs/openapi.json", openapi.clone()),
        )
         .service(web::resource("/todos")
                .route(web::get().to(get_todos))
                .route(web::post().to(create_todo))
            )
            .service(web::resource("/todos/{id}")
                .route(web::get().to(get_todo))
                .route(web::put().to(update_todo))
                .route(web::delete().to(delete_todo))
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await

    


}
