use crate::domain::entities::note::Note;
use uuid::Uuid;
use async_trait::async_trait;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum NoteRepositoryError {
    #[error("Note not found")]
    NoteNotFound,

    #[error("Note with this ID already exists")]
    NoteAlreadyExist,
}


#[async_trait]
pub trait NoteRepository {
    async fn create(&self, note: Note) -> Result<(), NoteRepositoryError>;
    async fn get_by_id(&self, id: Uuid) -> Result<Note, NoteRepositoryError>;
    async fn get_by_owner(&self, owner_id: Uuid) -> Result<Vec<Note>, NoteRepositoryError>;
    async fn update(&self, note: Note) -> Result<(), NoteRepositoryError>;
    async fn delete(&self, id: Uuid) -> Result<(), NoteRepositoryError>;
}

