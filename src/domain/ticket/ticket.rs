use uuid::Uuid;

pub struct Ticket {
    id: Uuid,
    created_by: Uuid,

    title: String,
    description: Option<String>,
    status: String,
}

#[derive(Debug, thiserror::Error)]
pub enum TicketError {
    #[error("title can not be empty")]
    EmptyTitle,
    #[error("the id of the user who created this ticket is invalid")]
    InvalidUserId,
}

impl Ticket {
    fn new(
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
            status: "pending".to_string(),
        })
    }
}
