use std::{thread, time::Duration, fs::File};
use rdev::{listen, Event, Key, EventType};
use arboard::Clipboard;
use daemonize::Daemonize;

fn main() {
    let stdout = File::create("/tmp/pez.out").unwrap();
    let stderr = File::create("/tmp/pez.err").unwrap();

    let daemonize = Daemonize::new()
        .pid_file("/tmp/pez.pid")
        .working_directory("/tmp")
        .user("mofei")
        .stdout(stdout)
        .stderr(stderr);

    match daemonize.start() {
        Ok(_) => println!("[pez] launched successfully"),
        Err(e) => eprintln!("error, {}", e),
    }

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
        println!("error: {:?}", error)
    }
}
