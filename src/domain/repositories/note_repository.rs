use uuid::Uuid;
use async_trait::async_trait;
use crate::domain::entities::note::Note;

#[async_trait]
pub trait NoteRepository {
    async fn create(&self, note: Note) -> Result<(), String>;
    async fn get_by_id(&self, id: Uuid) -> Result<Note, String>;
    async fn get_by_owner(&self, owner_id: Uuid) -> Result<Vec<Note>, String>;
    async fn update(&self, note: Note) -> Result<(), String>;
    async fn delete(&self, id: Uuid) -> Result<(), String>;
}

