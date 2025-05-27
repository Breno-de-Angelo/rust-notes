use crate::domain::entities::note::NoteError;
use crate::domain::repositories::note_repository::NoteRepositoryError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum NoteApplicationError {
    #[error(transparent)]
    ValidationError(#[from] NoteError),

    #[error(transparent)]
    RepositoryError(#[from] NoteRepositoryError),
}

