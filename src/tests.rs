use super::EventsManager;

#[test]
fn connecting() {
    let manager = EventsManager::default();
    manager.unwrap();
}