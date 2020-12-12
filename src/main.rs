use std::boxed::Box;
use std::collections::HashMap;

trait EventListener {
    fn update(&self, event_type: String);
}

#[derive(Debug)]
struct EmailNotificationListener {
    email: String,
}

impl EmailNotificationListener {
    fn new(email: String) -> Self {
        Self { email }
    }
}

impl EventListener for EmailNotificationListener {
    fn update(&self, event_type: String) {
        println!("Email: {} Event Type: {}", self.email, event_type);
    }
}

struct EventManager {
    listeners: HashMap<String, Vec<Box<dyn EventListener>>>,
}

impl EventManager {
    fn new() -> Self {
        Self {
            listeners: HashMap::new(),
        }
    }

    fn insert(&mut self, operation: String) {
        self.listeners.insert(operation, Vec::new());
    }

    fn subscribe(&mut self, event_type: &str, listener: Box<dyn EventListener>) {
        match self.listeners.get_mut(event_type) {
            Some(event) => event.push(listener),
            None => println!("{} not fount event type.", event_type),
        }
    }

    fn unsubscribe(&mut self, event_type: &str) {
        match self.listeners.get_mut(event_type) {
            Some(event) => {
                event.clear();
                self.listeners.remove(event_type);
                println!("Event: {} removido", &event_type);
            }
            None => println!("{} not fount event type.", event_type),
        }
    }

    fn notify(&mut self, event_type: String) {
        match self.listeners.get(&event_type) {
            Some(event) => {
                for e in event.iter() {
                    e.update(event_type.clone());
                }
            }
            None => println!("{} not fount event type.", event_type),
        }
    }
}

struct Car {
    events: EventManager,
}

impl Car {
    fn new() -> Self {
        let mut event = EventManager::new();
        event.insert("MotorLigado".to_string());

        Self { events: event }
    }

    fn ligar(&mut self) {
        self.events.notify("MotorLigado".to_string());
    }
}

fn main() {
    let email: String = String::from("mbsd@teste.com.br");
    let email_safety: String = String::from("safety@teste.com.br");
    let event_type: String = String::from("MotorLigado");

    let mut car: Car = Car::new();
    let email_list = Box::new(EmailNotificationListener::new(email));
    let email_list_safe = Box::new(EmailNotificationListener::new(email_safety));

    car.events.subscribe(&event_type, email_list);
    car.events.subscribe(&event_type, email_list_safe);

    car.ligar();

    car.events.unsubscribe(&event_type);
}
