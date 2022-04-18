use rdev::{listen, Event, Key, EventType};

fn main() {
    let mut cmd = false;

    let callback = move |event: Event| {
        //println!("My callback {:?}", event);

        if event.event_type == EventType::KeyPress(Key::MetaLeft) {
            cmd = true;
        } else if event.event_type == EventType::KeyRelease(Key::MetaLeft) {
            cmd = false;
        } else if event.event_type == EventType::KeyPress(Key::KeyC) {
            if cmd {
                println!("Command-C detected!");
            }
        }
    };

    if let Err(error) = listen(callback) {
        println!("Error: {:?}", error)
    }
}


