use crate::domain::repositories::note_repository::{NoteRepository, NoteRepositoryError};
use crate::domain::entities::note::NoteError;
use crate::domain::entities::note::Note;
use uuid::Uuid;
use thiserror::Error;

pub struct CreateNoteInput {
    pub title: String,
    pub summary: Option<String>,
    pub content: String,
}

pub struct CreateNoteUseCase<R: NoteRepository> {
    pub repo: R,
}

#[derive(Debug, Error)]
pub enum CreateNoteUseCaseError {
    #[error("Title cannot be empty")]
    EmptyTitle,

    #[error("Content cannot be empty")]
    EmptyContent,

    #[error("Note with this ID already exists")]
    NoteAlreadyExist,

    #[error("Unknown error")]
    UnknownError,
}

impl<R: NoteRepository> CreateNoteUseCase<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, input: CreateNoteInput, user_id: Uuid) -> Result<(), CreateNoteUseCaseError> {
        let note = match Note::new(input.title, input.summary, input.content, user_id) {
            Ok(note) => note,
            Err(e) => {
                return Err(match e {
                    NoteError::EmptyTitle => CreateNoteUseCaseError::EmptyTitle,
                    NoteError::EmptyContent => CreateNoteUseCaseError::EmptyContent,
                    _ => CreateNoteUseCaseError::UnknownError,
                });
            }
        };
        if let Err(e) = self.repo.create(note).await {
            return Err(match e {
                NoteRepositoryError::NoteAlreadyExist => CreateNoteUseCaseError::NoteAlreadyExist,
                _ => CreateNoteUseCaseError::UnknownError,
            });
        }
        Ok(())
    }
}
