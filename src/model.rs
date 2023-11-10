use crate::error::{  Error,Result };

#[derive(Clone, Debug, Serialize)]
pub struct Ticket{
    pub id: u64,
    pub title: String,

}

pub struct TicketForCreate{
    pub title: String,
}

#[derive(Clone)]
pub struct ModelController{
    tickets_store: Arc<Mutex<Vec<Option<Ticket>>>>,
}


impl ModelController{
    pub async fn new() -> Result<Self> {
        Ok(ModelController{
            tickets_store: Arc::default(),
        })
    }
}


impl ModelController {
    pub async fn create_ticket(&self, ticket_fc: TicketForCreate) -> Result<Ticket> {
        let mut store = self.tickets_store.lock().unwrap;
        let id = store.len() as u64 ;
        let ticket = Ticket{
            id: id,
            title: ticket_fc.title,
        };
        store.push(Some(ticket));
        Ok(Ticket)
    }
}
