use crate::domain::repositories::note_repository::NoteRepository;
use crate::application::notes::errors::NoteApplicationError;
use crate::domain::entities::note::Note;
use uuid::Uuid;

pub struct CreateNoteInput {
    pub title: String,
    pub summary: Option<String>,
    pub content: String,
}

pub struct CreateNoteUseCase<R: NoteRepository> {
    pub repo: R,
}

impl<R: NoteRepository> CreateNoteUseCase<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, input: CreateNoteInput, user_id: Uuid) -> Result<(), NoteApplicationError> {
        let note = Note::new(input.title, input.summary, input.content, user_id)?;
        self.repo.create(note).await?;
        Ok(())
    }
}
