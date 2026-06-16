pub enum StateError {
    AlreadyAssign,
    AlreadyClosed,
    AlreadyProcessing,
    AlreadyCancelled,
    InUnassigned,
}

pub trait State {
    fn assign(self: Box<Self>) -> Result<Box<dyn State>, StateError>;
    fn process(self: Box<Self>) -> Result<Box<dyn State>, StateError>;
    fn transfer(self: Box<Self>) -> Result<Box<dyn State>, StateError>;
    fn resolve(self: Box<Self>) -> Result<Box<dyn State>, StateError>;
    fn close(self: Box<Self>) -> Result<Box<dyn State>, StateError>;
    fn cancel(self: Box<Self>) -> Result<Box<dyn State>, StateError>;
}

pub struct TicketCreated;
pub struct TicketPending;
pub struct TicketAssigned;
pub struct TicketProcessing;
pub struct TicketResolved;
pub struct TicketClosed;
pub struct TicketCancelled;

impl State for TicketCreated {
    fn assign(self: Box<Self>) -> Result<Box<dyn State>, StateError> {
        Ok(Box::new(TicketAssigned))
    }

    fn process(self: Box<Self>) -> Result<Box<dyn State>, StateError> {
        Err(StateError::InUnassigned)
    }

    fn transfer(self: Box<Self>) -> Result<Box<dyn State>, StateError> {
        todo!()
    }

    fn resolve(self: Box<Self>) -> Result<Box<dyn State>, StateError> {
        todo!()
    }

    fn close(self: Box<Self>) -> Result<Box<dyn State>, StateError> {
        todo!()
    }

    fn cancel(self: Box<Self>) -> Result<Box<dyn State>, StateError> {
        todo!()
    }
}

impl State for TicketAssigned {
    fn assign(self: Box<Self>) -> Result<Box<dyn State>, StateError> {
        todo!()
    }

    fn process(self: Box<Self>) -> Result<Box<dyn State>, StateError> {
        todo!()
    }

    fn transfer(self: Box<Self>) -> Result<Box<dyn State>, StateError> {
        todo!()
    }

    fn resolve(self: Box<Self>) -> Result<Box<dyn State>, StateError> {
        todo!()
    }

    fn close(self: Box<Self>) -> Result<Box<dyn State>, StateError> {
        todo!()
    }

    fn cancel(self: Box<Self>) -> Result<Box<dyn State>, StateError> {
        todo!()
    }
}

impl State for TicketPending {
    fn assign(self: Box<Self>) -> Result<Box<dyn State>, StateError> {
        todo!()
    }

    fn process(self: Box<Self>) -> Result<Box<dyn State>, StateError> {
        todo!()
    }

    fn transfer(self: Box<Self>) -> Result<Box<dyn State>, StateError> {
        todo!()
    }

    fn resolve(self: Box<Self>) -> Result<Box<dyn State>, StateError> {
        todo!()
    }

    fn close(self: Box<Self>) -> Result<Box<dyn State>, StateError> {
        todo!()
    }

    fn cancel(self: Box<Self>) -> Result<Box<dyn State>, StateError> {
        todo!()
    }
}

impl State for TicketProcessing {
    fn assign(self: Box<Self>) -> Result<Box<dyn State>, StateError> {
        todo!()
    }

    fn process(self: Box<Self>) -> Result<Box<dyn State>, StateError> {
        todo!()
    }

    fn transfer(self: Box<Self>) -> Result<Box<dyn State>, StateError> {
        todo!()
    }

    fn resolve(self: Box<Self>) -> Result<Box<dyn State>, StateError> {
        todo!()
    }

    fn close(self: Box<Self>) -> Result<Box<dyn State>, StateError> {
        todo!()
    }

    fn cancel(self: Box<Self>) -> Result<Box<dyn State>, StateError> {
        todo!()
    }
}

impl State for TicketResolved {
    fn assign(self: Box<Self>) -> Result<Box<dyn State>, StateError> {
        todo!()
    }

    fn process(self: Box<Self>) -> Result<Box<dyn State>, StateError> {
        todo!()
    }

    fn transfer(self: Box<Self>) -> Result<Box<dyn State>, StateError> {
        todo!()
    }

    fn resolve(self: Box<Self>) -> Result<Box<dyn State>, StateError> {
        todo!()
    }

    fn close(self: Box<Self>) -> Result<Box<dyn State>, StateError> {
        todo!()
    }

    fn cancel(self: Box<Self>) -> Result<Box<dyn State>, StateError> {
        todo!()
    }
}

impl State for TicketClosed {
    fn assign(self: Box<Self>) -> Result<Box<dyn State>, StateError> {
        todo!()
    }

    fn process(self: Box<Self>) -> Result<Box<dyn State>, StateError> {
        todo!()
    }

    fn transfer(self: Box<Self>) -> Result<Box<dyn State>, StateError> {
        todo!()
    }

    fn resolve(self: Box<Self>) -> Result<Box<dyn State>, StateError> {
        todo!()
    }

    fn close(self: Box<Self>) -> Result<Box<dyn State>, StateError> {
        todo!()
    }

    fn cancel(self: Box<Self>) -> Result<Box<dyn State>, StateError> {
        todo!()
    }
}

impl State for TicketCancelled {
    fn assign(self: Box<Self>) -> Result<Box<dyn State>, StateError> {
        todo!()
    }

    fn process(self: Box<Self>) -> Result<Box<dyn State>, StateError> {
        todo!()
    }

    fn transfer(self: Box<Self>) -> Result<Box<dyn State>, StateError> {
        todo!()
    }

    fn resolve(self: Box<Self>) -> Result<Box<dyn State>, StateError> {
        todo!()
    }

    fn close(self: Box<Self>) -> Result<Box<dyn State>, StateError> {
        todo!()
    }

    fn cancel(self: Box<Self>) -> Result<Box<dyn State>, StateError> {
        todo!()
    }
}
