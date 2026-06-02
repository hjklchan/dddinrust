#[derive(Default, Clone, Copy)]
pub enum TicketStatus {
    #[default]
    Pending,
    Processing,
    Testing,
    Completed,
}

impl TicketStatus {
    pub fn to_text(&self) -> &str {
        match self {
            TicketStatus::Pending => "pending",
            TicketStatus::Processing => "processing",
            TicketStatus::Testing => "testing",
            TicketStatus::Completed => "completed",
        }
    }
}
