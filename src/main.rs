use borsh::{BorshDeserialize, BorshSerialize};
use crosstown_bus::{CrosstownBus, HandleError, MessageHandler};

#[derive(Debug, Clone, BorshDeserialize, BorshSerialize)]
pub struct UserCreatedEventMessage {
    pub user_id: String,
    pub user_name: String,
}

pub struct UserCreatedHandler;

impl MessageHandler<UserCreatedEventMessage> for UserCreatedHandler {
    fn get_handler_action(&self) -> String {
        String::from("UserCreated")
    }

    // Memperbaiki tipe data Box menjadi Box<UserCreatedEventMessage>ca
    fn handle(&self, message: Box<UserCreatedEventMessage>) -> Result<(), HandleError> {
        println!("Message received on handler 1: {:?}", message);
        Ok(())
    }
}

fn main() {
    let url = "amqps://jilkwcrc:3B3qb2ONEYepKlWPrYqLcOVHrCXrz9S6@armadillo.rmq.cloudamqp.com/jilkwcrc".to_owned();

    let mut p = CrosstownBus::new_queue_publisher(url).unwrap();

    _ = p.publish_event("user_created".to_owned(), UserCreatedEventMessage {
        user_id: "1".to_owned(), 
        user_name: "2406495786-Amir".to_owned() 
    });
    
    _ = p.publish_event("user_created".to_owned(), UserCreatedEventMessage {
        user_id: "2".to_owned(), 
        user_name: "2406495786-Budi".to_owned() 
    });
    
    _ = p.publish_event("user_created".to_owned(), UserCreatedEventMessage {
        user_id: "3".to_owned(), 
        user_name: "2406495786-Cica".to_owned() 
    });
    
    _ = p.publish_event("user_created".to_owned(), UserCreatedEventMessage {
        user_id: "4".to_owned(), 
        user_name: "2406495786-Dira".to_owned() 
    });
    
    _ = p.publish_event("user_created".to_owned(), UserCreatedEventMessage {
        user_id: "5".to_owned(), 
        user_name: "2406495786-Emir".to_owned() 
    });
}