use thiserror::Error;
use uuid::Uuid;

use crate::use_cases::notes::create_note::{CreateNoteError, CreateNoteInput, CreateNoteInterface};

pub struct CreateNoteControllerInputDTO {
    pub user_id: Uuid,
    pub title: String,
    pub summary: Option<String>,
    pub content: String,
}

pub struct CreateNoteControllerOutputDTO {
    pub id: Uuid,
    pub title: String,
    pub summary: Option<String>,
    pub content: String,
}

#[derive(Debug, Error)]
pub enum CreateNoteControllerError {
    #[error("Invalid note: {detail}")]
    InvalidNote { detail: String },

    #[error("Note already exists")]
    NoteAlreadyExist,

    #[error("Unknown error")]
    UnknownError,
}

pub struct CreateNoteController<U> {
    pub use_case: U,
}

impl<U: CreateNoteInterface> CreateNoteController<U> {
    pub async fn execute(
        &self,
        input: CreateNoteControllerInputDTO,
    ) -> Result<CreateNoteControllerOutputDTO, CreateNoteControllerError> {
        let note = self
            .use_case
            .execute(CreateNoteInput {
                user_id: input.user_id,
                title: input.title,
                summary: input.summary,
                content: input.content,
            })
            .await
            .map_err(|e| match e {
                CreateNoteError::EmptyTitle => CreateNoteControllerError::InvalidNote {
                    detail: "Empty title".to_string(),
                },
                CreateNoteError::EmptyContent => CreateNoteControllerError::InvalidNote {
                    detail: "Empty content".to_string(),
                },
                CreateNoteError::NoteAlreadyExist => CreateNoteControllerError::NoteAlreadyExist,
                CreateNoteError::UnknownError => CreateNoteControllerError::UnknownError,
            })?;

        Ok(CreateNoteControllerOutputDTO {
            id: note.id,
            title: note.title,
            summary: note.summary,
            content: note.content,
        })
    }
}
