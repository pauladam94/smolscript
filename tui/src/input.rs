use ratatui::crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};

pub enum Input {
    Next,
    Previous,
    NotRecognized,
}

impl TryFrom<Event> for Input {
    type Error = ();

    fn try_from(event: Event) -> Result<Self, Self::Error> {
        match event {
            Event::Key(KeyEvent {
                code: KeyCode::Left,
                kind: KeyEventKind::Press,
                ..
            }) => Ok(Input::Previous),
            Event::Key(KeyEvent {
                code: KeyCode::Right,
                kind: KeyEventKind::Press,
                ..
            }) => Ok(Input::Next),
            Event::FocusGained
            | Event::FocusLost
            | Event::Key(_)
            | Event::Mouse(_)
            | Event::Paste(_)
            | Event::Resize(_, _) => Err(()),
        }
    }
}
