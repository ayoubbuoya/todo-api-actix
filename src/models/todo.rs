use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub struct TodoItem {
    #[schema(example = "c5aaefd0-8f5d-41d9-9210-935fa2aabefa")]
    pub id: String,
    #[schema(example = "Read 15 pages of Rust book")]
    pub title: String,
    pub completed: bool,
}

// Struct for creating a new todo, without `id` field.
#[derive(Deserialize, ToSchema)]
pub struct TodoCreateRequest {
    #[schema(example = "Read 20 pages of Python book")]
    pub title: String,
}

#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub struct Todo {
    pub title: String,
    pub completed: bool,
}
