use std::io::{stdout, Write};
use std::sync::Arc;
use std::time::Duration;

use crossterm::cursor::{Hide, Show};
use crossterm::event::{poll as poll_terminal_event, read as read_terminal_event, Event, KeyCode};
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use server::{VirtualTerminal, VoidStringInputTerminal};
use tokio::sync::mpsc::channel;
use tokio::sync::Mutex;
use tokio::{io, task::yield_now};

pub fn key_event_to_keycode(event: KeyCode) -> u8 {
    use keycodes::*;
    match event {
        KeyCode::Backspace => KEY_BACK_SPACE,
        KeyCode::Enter => KEY_ENTER,
        KeyCode::Left => KEY_LEFT,
        KeyCode::Right => KEY_RIGHT,
        KeyCode::Up => KEY_UP,
        KeyCode::Down => KEY_DOWN,
        KeyCode::Home => KEY_HOME,
        KeyCode::End => KEY_END,
        KeyCode::PageUp => KEY_PAGE_UP,
        KeyCode::PageDown => KEY_PAGE_DOWN,
        KeyCode::Tab => KEY_TAB,
        KeyCode::Delete => KEY_DELETE,
        KeyCode::Insert => KEY_INSERT,
        KeyCode::Char(ch) => ch.try_into().unwrap(),
        // TODO: handle those keys too
        _ => unreachable!(),
    }
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let vt = Arc::new(Mutex::new(VoidStringInputTerminal::default()));

    execute!(stdout(), EnterAlternateScreen, Hide)?;

    std::panic::set_hook(Box::new(|panicinfo| {
        eprint!("{panicinfo}");
        disable_raw_mode().unwrap();
        execute!(stdout(), LeaveAlternateScreen, Show).unwrap();
    }));

    enable_raw_mode()?;

    let (send, mut recv) = channel::<u8>(16);

    tokio::spawn(async move {
        loop {
            match poll_terminal_event(Duration::from_secs(0)) {
                Ok(has_terminal_event) => {
                    if has_terminal_event {
                        let event = read_terminal_event().unwrap();
                        match event {
                            Event::Key(key) => {
                                let keycode = key_event_to_keycode(key.code);
                                send.send(keycode).await.unwrap();
                            }
                            _ => {}
                        }
                    }
                }
                Err(_err) => break,
            }
            yield_now().await
        }
        unreachable!()
    });

    loop {
        match recv.recv().await {
            None => break,
            Some(keycode) => {
                let mut vt = vt.lock().await;
                vt.on_input(keycode).await;
                let rendered = vt.render().await;
                stdout().write(&rendered)?;
                stdout().flush()?;
            }
        }
    }

    disable_raw_mode()?;
    execute!(stdout(), LeaveAlternateScreen, Show)?;

    Ok(())
}
