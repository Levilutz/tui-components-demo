use std::fmt::Display;

use crossterm::event::KeyCode;
use ratatui::{layout::Rect, Frame};

use super::{form_text, form_toggle, Component};

pub struct FormProps {}

pub enum FormActions {
    /// An action used to halt event bubbling when the key event was handled internally.
    NoBubble,
    Submit {
        formatted: String,
    },
}

enum FormItem {
    Toggle(form_toggle::FormToggleProps, form_toggle::FormToggle),
    Text(form_text::FormTextProps, form_text::FormText),
}

impl FormItem {
    fn set_focused(&mut self, focused: bool) {
        match self {
            FormItem::Toggle(props, _) => props.focused = focused,
            FormItem::Text(props, _) => props.focused = focused,
        }
    }

    fn render(&mut self, frame: &mut Frame, area: Rect) {
        match self {
            FormItem::Toggle(props, component) => component.render(props, frame, area),
            FormItem::Text(props, component) => component.render(props, frame, area),
        }
    }

    fn handle_key(&mut self, code: KeyCode) -> Option<()> {
        match self {
            FormItem::Toggle(props, component) => match component.handle_key(props, code) {
                Some(form_toggle::FormToggleActions::SetValue(value)) => {
                    props.value = value;
                    Some(())
                }
                None => None,
            },
            FormItem::Text(props, component) => match component.handle_key(props, code) {
                Some(form_text::FormTextActions::SetContent(content)) => {
                    props.content = content;
                    Some(())
                }
                None => None,
            },
        }
    }
}

impl Display for FormItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FormItem::Toggle(props, _) => write!(f, "{}", props.value),
            FormItem::Text(props, _) => write!(f, "{}", props.content),
        }
    }
}

pub struct Form {
    items: Vec<FormItem>,
    focused: Option<usize>,
}

impl Form {
    fn delegate_key(&mut self, code: KeyCode) -> Option<()> {
        self.items.get_mut(self.focused?)?.handle_key(code)
    }
}

impl Default for Form {
    fn default() -> Self {
        Self {
            items: vec![
                FormItem::Toggle(
                    form_toggle::FormToggleProps {
                        focused: false,
                        value: false,
                    },
                    form_toggle::FormToggle::default(),
                ),
                FormItem::Text(
                    form_text::FormTextProps {
                        focused: false,
                        content: "".to_string(),
                    },
                    form_text::FormText::default(),
                ),
                FormItem::Toggle(
                    form_toggle::FormToggleProps {
                        focused: false,
                        value: false,
                    },
                    form_toggle::FormToggle::default(),
                ),
            ],
            focused: None,
        }
    }
}

impl Component for Form {
    type Props = FormProps;
    type Actions = FormActions;

    fn render(&mut self, _props: &Self::Props, frame: &mut Frame, area: Rect) {
        for (ind, item) in self.items.iter_mut().enumerate() {
            if ind >= area.height as usize {
                break;
            }
            item.set_focused(self.focused == Some(ind));
            item.render(
                frame,
                Rect {
                    x: area.x,
                    y: area.y + ind as u16,
                    width: area.width,
                    height: 1,
                },
            );
        }
    }

    fn handle_key(&mut self, _props: &Self::Props, code: KeyCode) -> Option<Self::Actions> {
        if let Some(_) = self.delegate_key(code) {
            Some(FormActions::NoBubble)
        } else {
            match code {
                KeyCode::Down => {
                    self.focused = match self.focused {
                        Some(ind) => Some((ind + 1).min(self.items.len() - 1)),
                        None => Some(0),
                    };
                    Some(FormActions::NoBubble)
                }
                KeyCode::Up => {
                    self.focused = match self.focused {
                        Some(ind) => Some(ind.saturating_sub(1)),
                        None => Some(self.items.len() - 1),
                    };
                    Some(FormActions::NoBubble)
                }
                KeyCode::Enter => Some(FormActions::Submit {
                    formatted: self
                        .items
                        .iter()
                        .map(|item| format!("{}", item))
                        .collect::<Vec<String>>()
                        .join(", "),
                }),
                _ => None,
            }
        }
    }
}
