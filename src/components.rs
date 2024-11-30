pub mod app;
pub mod form;
pub mod form_toggle;

use crossterm::event::KeyCode;
use ratatui::{layout::Rect, Frame};

pub trait Component {
    type Props;
    type Actions;

    fn render(&mut self, props: Self::Props, frame: &mut Frame, area: Rect);

    fn handle_key(&mut self, props: Self::Props, code: KeyCode) -> Option<Self::Actions>;
}
