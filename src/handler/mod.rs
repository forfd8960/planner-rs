use crate::model::TodoStore;

pub mod create;
pub mod delete;
pub mod list;
pub mod update;

pub fn save_changes(todo_store: &mut TodoStore) -> Result<(), String> {
    todo_store.save()
}
