use crate::domain::ticket::TicketStatus;
use chrono::Utc;
use uuid::Uuid;

pub struct Ticket {
    id: Uuid,
    created_by: Uuid,

    title: String,
    description: Option<String>,
    status: TicketStatus,

    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<Utc>,
}

#[derive(Debug, thiserror::Error)]
pub enum TicketError {
    #[error("title can not be empty")]
    EmptyTitle,
    #[error("the id of the user who created this ticket is invalid")]
    InvalidUserId,
}

impl Ticket {
    pub fn new(
        created_by: Uuid,
        title: String,
        description: Option<String>,
    ) -> Result<Ticket, TicketError> {
        if title.is_empty() {
            return Err(TicketError::EmptyTitle);
        }

        if created_by.is_nil() {
            return Err(TicketError::InvalidUserId);
        }

        Ok(Ticket {
            id: Uuid::new_v4(),
            created_by: created_by,
            title,
            description,
            status: Default::default(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn who_created(&self) -> Uuid {
        self.created_by
    }

    pub fn title(&self) -> String {
        self.title.clone()
    }

    pub fn description(&self) -> String {
        match &self.description {
            Some(value) => value.clone(),
            None => "".to_string(),
        }
    }

    pub fn status(&self) -> TicketStatus {
        self.status
    }

    pub fn created_at(&self) -> chrono::DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> chrono::DateTime<Utc> {
        self.updated_at
    }
}
