use ratatui::crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

pub enum Input {
    InterpreterNext,
    InterpreterBack,
    TabRight,
    TabLeft,
}

impl TryFrom<Event> for Input {
    type Error = ();

    fn try_from(event: Event) -> Result<Self, Self::Error> {
        match event {
            Event::Key(KeyEvent {
                code: KeyCode::Right,
                modifiers: KeyModifiers::ALT,
                kind: KeyEventKind::Press,
                ..
            }) => Ok(Input::TabRight),
            Event::Key(KeyEvent {
                code: KeyCode::Left,
                modifiers: KeyModifiers::ALT,
                kind: KeyEventKind::Press,
                ..
            }) => Ok(Input::TabLeft),
            Event::Key(KeyEvent {
                code: KeyCode::Left,
                kind: KeyEventKind::Press,
                ..
            }) => Ok(Input::InterpreterBack),
            Event::Key(KeyEvent {
                code: KeyCode::Right,
                kind: KeyEventKind::Press,
                ..
            }) => Ok(Input::InterpreterNext),
            Event::FocusGained
            | Event::FocusLost
            | Event::Key(_)
            | Event::Mouse(_)
            | Event::Paste(_)
            | Event::Resize(_, _) => Err(()),
        }
    }
}
