use tokio::sync::{mpsc, oneshot};

pub enum FleetMessage {
    Test {
        respond_to: oneshot::Sender<bool>,
    },
    AddShip {
        respond_to: oneshot::Sender<bool>,
        user_discord: String,
        manufacturer: String,
        model: String,
        name: Option<String>,
    },
    RemoveShip {
        respond_to: oneshot::Sender<bool>,
        user_discord: String,
        ship_id: usize,
    },
    NameShip {
        respond_to: oneshot::Sender<bool>,
        user_discord: String,
        ship_id: usize,
        new_name: String,
    },
}

pub struct FleetActor {
    receiver: mpsc::Receiver<FleetMessage>,
}

impl FleetActor {
    pub fn new(receiver: mpsc::Receiver<FleetMessage>) -> Self {
        FleetActor { receiver }
    }

    pub async fn handle_message(&mut self, msg: FleetMessage) {
        match msg {
            FleetMessage::AddShip {
                respond_to,
                user_discord,
                manufacturer,
                model,
                name,
            } => todo!(),
            FleetMessage::RemoveShip {
                respond_to,
                user_discord,
                ship_id,
            } => todo!(),
            FleetMessage::NameShip {
                respond_to,
                user_discord,
                ship_id,
                new_name,
            } => todo!(),
            FleetMessage::Test { respond_to } => {
                respond_to.send(true);
            }
        }
    }
}

async fn run_fleet_actor(mut actor: FleetActor) {
    while let Some(msg) = actor.receiver.recv().await {
        actor.handle_message(msg).await;
    }
}

#[derive(Clone)]
pub struct FleetActorHandle {
    sender: mpsc::Sender<FleetMessage>,
}

impl FleetActorHandle {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel(8);
        let actor = FleetActor::new(receiver);

        tokio::spawn(run_fleet_actor(actor));

        Self { sender }
    }

    pub async fn test(&mut self) -> bool {
        let (send, receive) = oneshot::channel();
        let msg = FleetMessage::Test { respond_to: send };

        let _ = self.sender.send(msg).await;
        receive.await.expect("Actor task has been killed")
    }
}

impl Default for FleetActorHandle {
    fn default() -> Self {
        Self::new()
    }
}
