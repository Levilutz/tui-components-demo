use crossterm::event::KeyCode;
use ratatui::{layout::Rect, Frame};

use super::{form_toggle, Component};

pub struct FormProps {}

pub enum FormActions {
    /// An action used to halt event bubbling when the key event was handled internally.
    NoBubble,
    Submit {
        values: Vec<bool>,
    },
}

pub struct Form {
    values: Vec<bool>,
    inputs: Vec<form_toggle::FormToggle>,
    focused: usize,
}

impl Form {
    pub fn new(num_toggles: usize) -> Self {
        assert!(num_toggles > 0);
        Self {
            values: vec![false; num_toggles],
            inputs: (0..num_toggles)
                .map(|_| form_toggle::FormToggle::new())
                .collect(),
            focused: 0,
        }
    }
}

impl Component for Form {
    type Props = FormProps;
    type Actions = FormActions;

    fn render(&mut self, _props: Self::Props, frame: &mut Frame, area: Rect) {
        for y in 0..self.inputs.len().min(area.height as usize) {
            self.inputs[y].render(
                form_toggle::FormToggleProps {
                    focused: y == self.focused,
                    value: self.values[y],
                },
                frame,
                Rect {
                    x: area.x,
                    y: area.y + y as u16,
                    width: area.width,
                    height: 1,
                },
            );
        }
    }

    fn handle_key(&mut self, _props: Self::Props, code: KeyCode) -> Option<FormActions> {
        if let Some(action) = self.inputs[self.focused].handle_key(
            form_toggle::FormToggleProps {
                focused: true,
                value: self.values[self.focused],
            },
            code,
        ) {
            match action {
                form_toggle::FormToggleActions::Toggle => {
                    self.values[self.focused] = !self.values[self.focused]
                }
                form_toggle::FormToggleActions::Set(value) => self.values[self.focused] = value,
            }
            Some(FormActions::NoBubble)
        } else {
            match code {
                KeyCode::Down => {
                    self.focused = (self.focused + 1).min(self.inputs.len() - 1);
                    Some(FormActions::NoBubble)
                }
                KeyCode::Up => {
                    self.focused = self.focused.saturating_sub(1);
                    Some(FormActions::NoBubble)
                }
                KeyCode::Enter => Some(FormActions::Submit {
                    values: self.values.clone(),
                }),
                _ => None,
            }
        }
    }
}
