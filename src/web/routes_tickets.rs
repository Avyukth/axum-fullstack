use crate::error::Result;
use crate::model::{ModelController, Ticket, TicketForCreate};
use axum::Router;
use axum::extract::Path;
use axum::routing::{post, delete};
use axum::{extract::State, Json};

pub fn routes(mc: ModelController) -> Router {
    Router::new()
        .route("/tickets", post(create_ticket).get(list_tickets))
        .route("/tickets/:id", delete(delete_ticket))
        .with_state(mc)
}
async fn create_ticket(
    State(mc): State<ModelController>,
    Json(ticket_fc): Json<TicketForCreate>,
) -> Result<Json<Ticket>> {
    println!("--->> {:<12} -  create_ticket", "API_TICKET_HANDLER");
    let ticket = mc.create_ticket(ticket_fc).await?;

    Ok(Json(ticket))
}

async fn list_tickets(mc: State<ModelController>) -> Result<Json<Vec<Ticket>>> {
    println!("--->> {:<12} -  list_ticket", "API_TICKET_HANDLER");

    let tickets = mc.list_tickets().await?;
    Ok(Json(tickets))
}

async fn delete_ticket(mc: State<ModelController>, Path(id): Path<u64>) -> Result<Json<Ticket>> {
    println!("--->> {:<12} -  delete_ticket", "API_TICKET_HANDLER");

    let ticket = mc.delete_ticket(id).await?;
    Ok(Json(ticket))
}
