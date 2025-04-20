use crate::interface::CursorStyle;
use crate::position::Position;

pub mod command_view;
pub mod text_view;

pub trait View {
    fn set_size(&mut self, p: Position);
    fn get_size(&self) -> Position;
    fn get_view(&self) -> Vec<char>;
    fn get_updates(&mut self) -> Vec<bool>;
    fn get_cursor_pos(&self) -> Position;
    fn get_cursor_style(&self) -> CursorStyle;
}
