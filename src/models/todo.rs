use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;


#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub struct TodoItem {
    id: Uuid,
    title: String,
    completed: bool,
}
