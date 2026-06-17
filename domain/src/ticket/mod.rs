use crate::{
    shared::identifier::Identifier,
    ticket::state::{Assigned, Created, TicketState},
};

mod state;

pub struct Ticket {
    ticket_id: Identifier,

    title: String,
    assignee: Option<Identifier>,

    state: Box<dyn TicketState>,
}

impl Ticket {
    fn new(
        ticket_id: Identifier,
        title: String,
        assignee: Option<Identifier>,
        state: Box<dyn TicketState>,
    ) -> Self {
        Self {
            ticket_id,
            title,
            assignee,
            state,
        }
    }

    pub fn create(title: String, assignee: Option<Identifier>) -> Self {
        Self {
            ticket_id: Identifier::generate_v4(),
            title,
            assignee,
            state: Box::new(Created),
        }
    }

    pub fn assign(&mut self, assignee: Identifier) -> Result<(), String> {
        if self.assignee.is_some() {
            return Err("该 ticket 已经分配过了处理人".to_string());
        }

        let transition = self.state.assign().map_err(|err| err.to_string())?;
        self.assignee = Some(assignee);
        self.state = transition.next_state;

        Ok(())
    }

    pub fn in_progress(&mut self) -> Result<(), String> {
        Ok(())
    }
}
