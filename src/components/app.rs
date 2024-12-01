use crossterm::event::KeyCode;
use ratatui::{layout::Rect, Frame};

use super::{form, Component};

pub struct AppProps {}

pub enum AppActions {
    /// An action used to halt event bubbling when the key event was handled internally.
    NoBubble,
    Quit,
}

pub struct App {
    form: form::Form,
}

impl App {
    pub fn new() -> Self {
        Self {
            form: form::Form::default(),
        }
    }
}

impl Component for App {
    type Props = AppProps;
    type Actions = AppActions;

    fn render(&mut self, _props: &Self::Props, frame: &mut Frame, area: Rect) {
        self.form.render(&form::FormProps {}, frame, area);
    }

    fn handle_key(&mut self, _props: &Self::Props, code: KeyCode) -> Option<Self::Actions> {
        if let Some(action) = self.form.handle_key(&form::FormProps {}, code) {
            match action {
                form::FormActions::Submit { formatted: _ } => todo!(),
                _ => {}
            }
            Some(AppActions::NoBubble)
        } else {
            match code {
                KeyCode::Char('q') => Some(AppActions::Quit),
                _ => None,
            }
        }
    }
}
