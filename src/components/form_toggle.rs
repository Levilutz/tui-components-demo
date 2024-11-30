use crossterm::event::KeyCode;
use ratatui::{layout::Rect, text::Span, Frame};

use super::Component;

pub struct FormToggleProps {
    pub focused: bool,
    pub value: bool,
}

pub enum FormToggleActions {
    Toggle,
    Set(bool),
}

pub struct FormToggle {}

impl FormToggle {
    pub fn new() -> Self {
        Self {}
    }
}

impl Component for FormToggle {
    type Props = FormToggleProps;
    type Actions = FormToggleActions;

    fn render(&mut self, props: Self::Props, frame: &mut Frame, area: Rect) {
        let mut content = "".to_string();
        if props.focused {
            content += "> ";
        } else {
            content += "  ";
        }
        if props.value {
            content += " no <yes>"
        } else {
            content += "<no> yes "
        }
        frame.render_widget(Span::from(content), area);
    }

    fn handle_key(&mut self, _props: Self::Props, code: KeyCode) -> Option<FormToggleActions> {
        match code {
            KeyCode::Tab => Some(FormToggleActions::Toggle),
            KeyCode::Right => Some(FormToggleActions::Set(true)),
            KeyCode::Left => Some(FormToggleActions::Set(false)),
            _ => None,
        }
    }
}
