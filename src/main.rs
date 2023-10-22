use std::{io::Result, process, time::Duration};

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    terminal,
};
use errno::errno;

fn check_event() -> bool {
    let timeout = Duration::from_millis(100);

    if let Ok(b) = event::poll(timeout) {
        return b;
    } else {
        die("Poll error");
    }

    false
}

fn main() -> Result<()> {
    terminal::enable_raw_mode()?;

    loop {
        let mut c = None;

        if check_event() {
            if let Ok(ev) = event::read() {
                if let Event::Key(key_event) = ev {
                    if key_event.kind == KeyEventKind::Release {
                        c = Some(key_event)
                    }
                }
            } else {
                die("Read error");
            }
        }

        if let Some(c) = c {
            if c.code == KeyCode::Char('c') && c.modifiers == KeyModifiers::CONTROL {
                break;
            } else {
                println!("{c:?}\r");
            }
        }
    }

    terminal::disable_raw_mode()?;

    Ok(())
}

fn die<S: Into<String>>(msg: S) {
    let _ = terminal::disable_raw_mode();

    eprintln!("{}: {}", msg.into(), errno());

    process::exit(1);
}
