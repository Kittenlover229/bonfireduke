use std::process::exit;

use async_trait::async_trait;
use crossterm::ExecutableCommand;

#[async_trait]
pub trait VirtualTerminal {
    async fn on_input(&mut self, keycode: u8);
    async fn resize(&mut self, cols: u16, rows: u16);

    async fn render(&self) -> Vec<u8>;
}

#[derive(Debug, Clone, Default)]
pub struct VoidStringInputTerminal {
    input_buffer: Vec<u8>,

    cols: u16,
    rows: u16,
}

#[async_trait]
impl VirtualTerminal for VoidStringInputTerminal {
    async fn on_input(&mut self, keycode: u8) {
        if keycode == b'q' {
            panic!()
        }

        if keycode.is_ascii() {
            self.input_buffer.push(keycode)
        }
    }

    async fn resize(&mut self, cols: u16, rows: u16) {
        self.cols = cols;
        self.rows = rows;
    }

    async fn render(&self) -> Vec<u8> {
        let mut output = Vec::<u8>::new();

        use crossterm::cursor::MoveTo;
        use crossterm::terminal::Clear;
        use crossterm::terminal::ClearType;

        output
            .execute(MoveTo(0, 0))
            .unwrap()
            .execute(Clear(ClearType::All))
            .unwrap();

        for byte in self.input_buffer.iter() {
            output.push(*byte)
        }

        output
    }
}
