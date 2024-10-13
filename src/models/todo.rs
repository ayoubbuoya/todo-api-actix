use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

// Struct for creating a new todo, without `id` field.
#[derive(Deserialize, ToSchema)]
pub struct TodoCreateRequest {
    #[schema(example = "Read 20 pages of Python book")]
    pub title: String,
}

#[derive(Serialize, Deserialize, Clone, ToSchema, PartialEq, Eq)]
pub struct Todo {
    pub title: String,
    pub completed: bool,
}
