use tokio::sync::{mpsc, oneshot};

async fn _test() -> anyhow::Result<()> {
    let ds = DatastoreHandle::new();
    let _q = ds
        .add_manufacturer(
            "Origin".to_string(),
            "Fancy ships for fancy people.".to_string(),
        )
        .await?;
    let _r = ds
        .add_ship_model(
            "The Handlebar".to_string(),
            "A moustache-like ship for extremely good-looking men.".to_string(),
            "Origin".to_string(),
        )
        .await?;

    Ok(())
}

#[derive(Debug)]
pub enum DatastoreMessage<E> {
    AddEvent {
        respond_to: oneshot::Sender<anyhow::Result<()>>,
        bucket: String,
        event: E,
    },
}

#[derive(Debug)]
pub enum ShipModelEvent {
    Created {
        name: String,
        description: String,
        manufacturer: String,
    },
    NameSet {
        name: String,
    },
    DescriptionSet {
        description: String,
    },
    ManufacturerSet {
        manufacturer_id: String,
    },
    Removed {
        id: u64,
    },
}

#[derive(Debug)]
pub enum ManufacturerEvent {
    Created { name: String, description: String },
    NameSet { name: String },
    DescriptionSet { description: String },
    Removed { id: u64 },
}

#[derive(Debug)]
pub struct DatastoreHandle {
    ship_sender: mpsc::Sender<DatastoreMessage<ShipModelEvent>>,
    manufacturer_sender: mpsc::Sender<DatastoreMessage<ManufacturerEvent>>,
}

impl DatastoreHandle {
    pub fn new() -> Self {
        let (ship_sender, _ship_model_rx) = mpsc::channel(8);
        let (manufacturer_sender, _manufacturer_rx) = mpsc::channel(8);
        Self {
            ship_sender,
            manufacturer_sender,
        }
    }

    pub async fn add_manufacturer(&self, name: String, description: String) -> anyhow::Result<()> {
        let (tx, rx) = oneshot::channel();
        let evt = ManufacturerEvent::Created { name, description };
        let msg = DatastoreMessage::AddEvent {
            respond_to: tx,
            bucket: "manufacturers".to_string(),
            event: evt,
        };
        let _ = self.manufacturer_sender.send(msg).await?;

        let _ = rx.await?;
        Ok(())
    }

    pub async fn add_ship_model(
        &self,
        name: String,
        description: String,
        manufacturer: String,
    ) -> anyhow::Result<()> {
        let (tx, rx) = oneshot::channel();
        let evt = ShipModelEvent::Created {
            name,
            description,
            manufacturer,
        };

        let msg = DatastoreMessage::AddEvent {
            respond_to: tx,
            bucket: "ship_models".to_string(),
            event: evt,
        };

        let _ = self.ship_sender.send(msg).await?;

        let _ = rx.await?;

        Ok(())
    }
}

impl Default for DatastoreHandle {
    fn default() -> Self {
        Self::new()
    }
}

// pub enum EsDatastoreMessage<E: Event> {
//     AddEvent {
//         respond_to: oneshot::Sender<anyhow::Result<()>>,
//         event: E,
//         bucket: String,
//     },
// }

// #[derive(Debug)]
// pub struct EsDatastoreHandle<E: Event> {
//     receiver: mpsc::Receiver<EsDatastoreMessage<E>>,
// }
// impl<E: Event> EsDatastoreHandle<E> {
//     pub fn new() -> Self {
//         let (tx, rx) = mpsc::channel(8);
//         Self { receiver: rx }
//     }

//     pub fn add_event(event: E) -> anyhow::Result<()> {
//         todo!();
//     }
// }

// impl<E: Event> Default for EsDatastoreHandle<E> {
//     fn default() -> Self {
//         Self::new()
//     }
// }

// pub trait Event {
//     fn event_type(&self) -> String;
//     fn event_version(&self) -> String;
// }

// pub enum SpaceshipEvent {
//     Created { name: String, model: String },
//     Exploded { id: u64 },
// }
// impl Event for SpaceshipEvent {
//     fn event_type(&self) -> String {
//         match self {
//             SpaceshipEvent::Created { .. } => "SpaceshipCreated".to_string(),
//             SpaceshipEvent::Exploded { .. } => "SpaceshipExplored".to_string(),
//         }
//     }

//     fn event_version(&self) -> String {
//         todo!()
//     }
// }
