use std::fmt::Debug;

#[derive(Debug, thiserror::Error)]
pub enum TicketStateError {
    #[error("unknown assignee")]
    UnknownAssignee,
    #[error("disallow reassign")]
    DisallowReassign,
    #[error("already resolved")]
    AlreadyResolved,
    #[error("already in progress")]
    AlreadyInProgress,
}

#[derive(Debug)]
pub struct StateAssigningTransition {
    pub next_state: Box<dyn TicketState>,
}

pub trait TicketState: Debug {
    fn assign(&self) -> Result<StateAssigningTransition, TicketStateError>;
    fn in_progress(self: Box<Self>) -> Result<Box<dyn TicketState>, TicketStateError>;
    fn resolve(self: Box<Self>) -> Result<Box<dyn TicketState>, TicketStateError>;
}

#[derive(Debug)]
pub struct Created;
#[derive(Debug)]
pub struct Assigned;
#[derive(Debug)]
pub struct InProgress;
#[derive(Debug)]
pub struct Resolved;

impl TicketState for Created {
    fn assign(&self) -> Result<StateAssigningTransition, TicketStateError> {
        Ok(StateAssigningTransition {
            next_state: Box::new(Assigned),
        })
    }

    fn in_progress(self: Box<Self>) -> Result<Box<dyn TicketState>, TicketStateError> {
        Err(TicketStateError::UnknownAssignee)
    }

    fn resolve(self: Box<Self>) -> Result<Box<dyn TicketState>, TicketStateError> {
        Ok(Box::new(Resolved))
    }
}

impl TicketState for Assigned {
    fn assign(&self) -> Result<StateAssigningTransition, TicketStateError> {
        Err(TicketStateError::DisallowReassign)
    }

    fn in_progress(self: Box<Self>) -> Result<Box<dyn TicketState>, TicketStateError> {
        Ok(Box::new(InProgress))
    }

    fn resolve(self: Box<Self>) -> Result<Box<dyn TicketState>, TicketStateError> {
        Ok(Box::new(Resolved))
    }
}

impl TicketState for Resolved {
    fn assign(&self) -> Result<StateAssigningTransition, TicketStateError> {
        Err(TicketStateError::AlreadyResolved)
    }

    // 重新处理 Ticket
    fn in_progress(self: Box<Self>) -> Result<Box<dyn TicketState>, TicketStateError> {
        Ok(Box::new(InProgress))
    }

    fn resolve(self: Box<Self>) -> Result<Box<dyn TicketState>, TicketStateError> {
        Err(TicketStateError::AlreadyResolved)
    }
}

impl TicketState for InProgress {
    fn assign(&self) -> Result<StateAssigningTransition, TicketStateError> {
        Ok(StateAssigningTransition {
            next_state: Box::new(InProgress),
        })
    }

    fn in_progress(self: Box<Self>) -> Result<Box<dyn TicketState>, TicketStateError> {
        Err(TicketStateError::AlreadyInProgress)
    }

    fn resolve(self: Box<Self>) -> Result<Box<dyn TicketState>, TicketStateError> {
        Ok(Box::new(Resolved))
    }
}
