use std::collections::HashMap;
use std::boxed::Box;

trait EventListener {
    fn update(&self, event_type: String);
}

#[derive(Debug)]
struct EmailNotificationListener {
    email: String,
}

impl EmailNotificationListener {
    fn new(email: String) -> Self {
        Self {email}
    }
}

impl EventListener for EmailNotificationListener {

    fn update(&self, event_type: String) {
        println!("Email: {} Event Type: {}", self.email, event_type);
    }
}

struct EventManager {
    listeners: HashMap<String, Vec<Box<dyn EventListener>>>
}

impl EventManager {
    fn new() -> Self {
        Self{listeners: HashMap::new()}
    }

    fn insert(&mut self, operation: String) {
        self.listeners.insert(operation, Vec::new());
    }

    fn subscribe(&mut self, event_type: String, listener: Box<dyn EventListener>) {
        match self.listeners.get_mut(&event_type) {
            Some(event) => event.push(listener),
            None => println!("{} not fount event type.", event_type)
        }
    }

    fn unsubscribe(&mut self, event_type: String) {
        match self.listeners.remove(&event_type) {
            Some(_) => println!("removido"),
            None => println!("{} not fount event type.", event_type)
        }
    }

    fn notify(&mut self, event_type: String) {
        match self.listeners.get(&event_type) {
            Some(event) => {
                for e in event.iter() {
                    e.update(event_type.clone());
                }
            },
            None => println!("{} not fount event type.", event_type)
        }
    }

}

struct Car {
    events: EventManager
}

impl Car {
    fn new() -> Self {
        let mut event = EventManager::new();
        event.insert("MotorLigado".to_string());

        Self {
            events: event,
        }
    }

    fn ligar(&mut self) {
        self.events.notify("MotorLigado".to_string());
    }
}

fn main() {
    let email: String = "mbsd@teste.com.br".to_string();
    let email_safety: String = "safety@teste.com.br".to_string();

    let mut car: Car = Car::new();
    let event_type: String = "MotorLigado".to_string();
    let event_type_safe = event_type.clone();
    let email_list = Box::new(EmailNotificationListener::new(email));
    let email_list_safe = Box::new(EmailNotificationListener::new(email_safety));

    car.events.subscribe(event_type, email_list);
    car.events.subscribe(event_type_safe, email_list_safe);

    car.ligar();
}
