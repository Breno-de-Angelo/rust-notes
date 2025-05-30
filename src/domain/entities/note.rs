use chrono::{DateTime, Utc};
use thiserror::Error;
use uuid::Uuid;

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
    pub fn new(
        title: String,
        summary: Option<String>,
        content: String,
        owner_id: Uuid,
    ) -> Result<Self, NoteError> {
        let now = Utc::now();
        let id = Uuid::now_v7();
        if title.trim().len() == 0 {
            return Err(NoteError::EmptyTitle);
        }
        if content.trim().len() == 0 {
            return Err(NoteError::EmptyContent);
        }
        Ok(Note {
            id,
            title,
            summary,
            content,
            owner_id,
            created_at: now,
            updated_at: now,
        })
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_note_creation_success() {
        let title = "Testing title".to_string();
        let summary = Some("A short description of the note".to_string());
        let content = "The content of the note to be created.".to_string();
        let owner_id = Uuid::now_v7();

        let note = Note::new(title.clone(), summary.clone(), content.clone(), owner_id)
            .expect("Failed to create note");

        assert_eq!(note.title, title);
        assert_eq!(note.summary, summary);
        assert_eq!(note.content, content);
        assert_eq!(note.owner_id, owner_id);
    }

    #[test]
    fn test_note_creation_empty_title_should_fail() {
        let note = Note::new(
            "".to_string(),
            Some("summary".to_string()),
            "content".to_string(),
            Uuid::now_v7(),
        );
        assert!(matches!(note, Err(NoteError::EmptyTitle)));
    }

    #[test]
    fn test_note_creation_empty_content_should_fail() {
        let note = Note::new(
            "title".to_string(),
            Some("summary".to_string()),
            "".to_string(),
            Uuid::now_v7(),
        );
        assert!(matches!(note, Err(NoteError::EmptyContent)));
    }
}
