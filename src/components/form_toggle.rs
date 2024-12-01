use crossterm::event::KeyCode;
use ratatui::{layout::Rect, text::Span, Frame};

use super::Component;

pub struct FormToggleProps {
    pub focused: bool,
    pub value: bool,
}

pub enum FormToggleActions {
    SetValue(bool),
}

#[derive(Default)]
pub struct FormToggle {}

impl Component for FormToggle {
    type Props = FormToggleProps;
    type Actions = FormToggleActions;

    fn render(&mut self, props: &Self::Props, frame: &mut Frame, area: Rect) {
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

    fn handle_key(&mut self, props: &Self::Props, code: KeyCode) -> Option<Self::Actions> {
        match code {
            KeyCode::Tab => Some(FormToggleActions::SetValue(!props.value)),
            KeyCode::Right => Some(FormToggleActions::SetValue(true)),
            KeyCode::Left => Some(FormToggleActions::SetValue(false)),
            _ => None,
        }
    }
}
