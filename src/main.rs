use std::{thread, time::Duration};
use rdev::{listen, Event, Key, EventType};
use arboard::Clipboard;

fn main() {
    let mut clipboard = Clipboard::new().unwrap();
    let mut clip: Vec<String> = Vec::new();

    let mut prev_copy = String::new();
    let mut cmd = false;

    let callback = move |event: Event| {
        //println!("My callback {:?}", event);

        if event.event_type == EventType::KeyPress(Key::MetaLeft) {
            cmd = true;
        } else if event.event_type == EventType::KeyRelease(Key::MetaLeft) {
            cmd = false;
            prev_copy = clipboard.get_text().unwrap();

            //println!("{:?}", clip);
        } else if event.event_type == EventType::KeyPress(Key::KeyC) {
            if cmd {
                thread::sleep(Duration::from_millis(10));

                let copy = clipboard.get_text().unwrap();

                if copy != *prev_copy {
                    clip.push(copy);
                }
            }
        } else if event.event_type == EventType::KeyPress(Key::KeyV) {
            if cmd {
                if clip.len() > 0 {
                    let paste = clip.remove(0);
                    clipboard.set_text(paste.into()).unwrap();
                }
            }
        }
    };

    if let Err(error) = listen(callback) {
        println!("Error: {:?}", error)
    }
}
