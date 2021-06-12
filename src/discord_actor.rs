use tokio::sync::{mpsc, oneshot};

use crate::{
    database::{models::Manufacturer, DatabaseActorHandle},
    fleet_actor::FleetActorHandle,
};
pub enum DiscordMessage {
    GetManufacturers {
        respond_to: oneshot::Sender<Vec<Manufacturer>>,
    },
}

pub struct DiscordActor {
    receiver: mpsc::Receiver<DiscordMessage>,
    db_handle: DatabaseActorHandle,
    fleet_handle: FleetActorHandle,
}

impl DiscordActor {
    pub fn new(
        receiver: mpsc::Receiver<DiscordMessage>,
        db_handle: DatabaseActorHandle,
        fleet_handle: FleetActorHandle,
    ) -> Self {
        DiscordActor {
            receiver,
            db_handle,
            fleet_handle,
        }
    }

    pub async fn handle_message(&mut self, msg: DiscordMessage) {
        match msg {
            DiscordMessage::GetManufacturers { respond_to } => {
                let result = self.db_handle.get_all_manufacturers().await;
                respond_to.send(result);
            }
        }
    }
}

async fn run_discord_actor(mut actor: DiscordActor) {
    while let Some(msg) = actor.receiver.recv().await {
        actor.handle_message(msg).await;
    }
}

#[derive(Clone)]
pub struct DiscordActorHandle {
    sender: mpsc::Sender<DiscordMessage>,
}

impl DiscordActorHandle {
    pub fn new(db_handle: DatabaseActorHandle, fleet_handle: FleetActorHandle) -> Self {
        let (sender, receiver) = mpsc::channel(8);
        let actor = DiscordActor::new(receiver, db_handle, fleet_handle);

        tokio::spawn(run_discord_actor(actor));

        Self { sender }
    }

    pub async fn do_a_thing(&mut self) -> Vec<Manufacturer> {
        let (send, receive) = oneshot::channel();
        let msg = DiscordMessage::GetManufacturers { respond_to: send };

        let _ = self.sender.send(msg).await;
        receive.await.expect("Actor task has been killed")
    }
}
