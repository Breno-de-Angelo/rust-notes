use crate::entities::note::Note;
use crate::entities::note::NoteError;
use crate::use_cases::repositories::note_repository::{NoteRepository, NoteRepositoryError};
use async_trait::async_trait;
use thiserror::Error;
use uuid::Uuid;

pub struct CreateNoteInput {
    pub user_id: Uuid,
    pub title: String,
    pub summary: Option<String>,
    pub content: String,
}

pub struct CreateNoteOutput {
    pub id: Uuid,
    pub title: String,
    pub summary: Option<String>,
    pub content: String,
}

#[async_trait]
pub trait CreateNoteInterface {
    async fn execute(&self, input: CreateNoteInput) -> Result<CreateNoteOutput, CreateNoteError>;
}

#[derive(Debug, Error)]
pub enum CreateNoteError {
    #[error("Title cannot be empty")]
    EmptyTitle,

    #[error("Content cannot be empty")]
    EmptyContent,

    #[error("Note with this ID already exists")]
    NoteAlreadyExist,

    #[error("Unknown error")]
    UnknownError,
}

pub struct CreateNoteInteractor<R: NoteRepository> {
    repo: R,
}

#[async_trait]
impl<R: NoteRepository> CreateNoteInterface for CreateNoteInteractor<R> {
    async fn execute(&self, input: CreateNoteInput) -> Result<CreateNoteOutput, CreateNoteError> {
        let note = match Note::new(input.title, input.summary, input.content, input.user_id) {
            Ok(note) => note,
            Err(e) => {
                return Err(match e {
                    NoteError::EmptyTitle => CreateNoteError::EmptyTitle,
                    NoteError::EmptyContent => CreateNoteError::EmptyContent,
                    _ => CreateNoteError::UnknownError,
                });
            }
        };
        if let Err(e) = self.repo.create(&note).await {
            return Err(match e {
                NoteRepositoryError::NoteAlreadyExist => CreateNoteError::NoteAlreadyExist,
                _ => CreateNoteError::UnknownError,
            });
        }
        Ok(CreateNoteOutput {
            id: note.id,
            title: note.title,
            summary: note.summary,
            content: note.content,
        })
    }
}
