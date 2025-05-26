use uuid::Uuid;
use chrono::{DateTime,Utc};

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

impl Note {
    pub fn new(title: String, summary: Option<String>, content: String, owner_id: Uuid) -> Self {
        let now = Utc::now();
        let id = Uuid::now_v7(); 
        Note { id, title, summary, content, owner_id, created_at: now, updated_at: now }
    }

    pub fn edit(&mut self, content: String) {
        self.content = content;
        self.updated_at = Utc::now();
    }

    pub fn update_summary(&mut self, summary: String) {
        self.summary = Some(summary);
        self.updated_at = Utc::now();
    }

    pub fn rename(&mut self, title: String) {
        self.title = title;
        self.updated_at = Utc::now();
    }
}
