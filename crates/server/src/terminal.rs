use async_trait::async_trait;

#[async_trait]
pub trait VirtualTerminal {
    async fn on_input(&mut self, keycode: u8);
}

#[derive(Debug, Clone, Default)]
pub struct VoidStringInputTerminal {
    input_buffer: Vec<u8>,
}

#[async_trait]
impl VirtualTerminal for VoidStringInputTerminal {
    async fn on_input(&mut self, keycode: u8) {
        if keycode.is_ascii() {
            self.input_buffer.push(keycode)
        }
    }
}
