use uuid::Uuid;
use chrono::{DateTime,Utc};
use thiserror::Error;

#[derive(Debug)]
pub struct Note {
    pub id: Uuid,
    pub title: String,
    pub summary: Option<String>,
    pub content: String,
    pub owner_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Error)]
pub enum NoteError {
    #[error("Title cannot be empty")]
    EmptyTitle,

    #[error("Content cannot be empty")]
    EmptyContent,
}

impl Note {
    pub fn new(title: String, summary: Option<String>, content: String, owner_id: Uuid) -> Result<Self, NoteError> {
        let now = Utc::now();
        let id = Uuid::now_v7(); 
        if title.trim().len() == 0 {
            return Err(NoteError::EmptyTitle);
        }
        if content.trim().len() == 0 {
            return Err(NoteError::EmptyContent);
        }
        Ok(Note { id, title, summary, content, owner_id, created_at: now, updated_at: now })
    }

    pub fn edit(&mut self, content: String) -> Result<(), NoteError> {
        if content.trim().len() == 0 {
            return Err(NoteError::EmptyContent);
        }
        self.content = content;
        self.updated_at = Utc::now();
        return Ok(());
    }

    pub fn update_summary(&mut self, summary: String) {
        self.summary = Some(summary);
        self.updated_at = Utc::now();
    }

    pub fn rename(&mut self, title: String) -> Result<(), NoteError> {
        if title.trim().len() == 0 {
            return Err(NoteError::EmptyTitle);
        }
        self.title = title;
        self.updated_at = Utc::now();
        return Ok(());
    }
}
