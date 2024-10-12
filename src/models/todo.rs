use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub struct TodoItem {
    #[schema(example = "c5aaefd0-8f5d-41d9-9210-935fa2aabefa")]
    pub id: String,
    #[schema(example = "Buy groceries")]
    pub title: String,
    #[schema(example = false)]
    pub completed: bool,
}

// Struct for creating a new todo, without `id` field.
#[derive(Deserialize, ToSchema)]
pub struct TodoCreateRequest {
    #[schema(example = "Buy groceries")]
    pub title: String,
    #[schema(example = false)]
    pub completed: bool,
}
