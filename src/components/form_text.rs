use crossterm::event::KeyCode;
use ratatui::{layout::Rect, text::Span, Frame};

use super::Component;

pub struct FormTextProps {
    pub focused: bool,
    pub content: String,
}

pub enum FormTextActions {
    SetContent(String),
}

#[derive(Default)]
pub struct FormText {}

impl Component for FormText {
    type Props = FormTextProps;
    type Actions = FormTextActions;

    fn render(&mut self, props: &Self::Props, frame: &mut Frame, area: Rect) {
        let mut content = "".to_string();
        if props.focused {
            content += "> ";
        } else {
            content += "  ";
        }
        content += &props.content;
        frame.render_widget(Span::from(content), area);
    }

    fn handle_key(&mut self, props: &Self::Props, code: KeyCode) -> Option<Self::Actions> {
        match code {
            KeyCode::Char(c) => Some(FormTextActions::SetContent(format!(
                "{}{}",
                props.content, c
            ))),
            KeyCode::Backspace => Some(FormTextActions::SetContent(
                props.content[..props.content.len().saturating_sub(1)].to_string(),
            )),
            _ => None,
        }
    }
}
