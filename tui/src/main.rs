use ast::ast_parser;
use clap::Parser;
use interpreter::interpreter::Interpreter;
use ratatui::{
    crossterm::event::{Event, KeyCode, KeyEvent, poll, read},
    layout::{Constraint, Layout, Margin},
    prelude::{Buffer, Rect},
    widgets::{Block, Paragraph, Widget},
};
use std::{fs::File, io::Read, time::Duration};
use tui::{args::Args, input::Input};

pub struct Game {
    interpreter: Interpreter,
}

pub fn get_string_from_path(path: &str) -> String {
    let mut file = File::open(path).unwrap_or_else(|_| panic!("Failed to open file '{}'", path));
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .unwrap_or_else(|_| panic!("Failed to read file '{}'", path));
    contents
}

impl Game {
    pub fn new(contents: &str) -> Self {
        let ast = ast_parser(contents).unwrap_or_else(|e| {
            panic!(
                "Failed to parse this file '{}'.\nGot this error : \n{}",
                contents, e
            )
        });

        Self {
            interpreter: Interpreter::new(ast),
        }
    }

    fn interact(&mut self, input: Option<Input>) {
        match if let Some(input) = input {
            input
        } else {
            return;
        } {
            Input::Next => {
                self.interpreter.next();
            }
            Input::Previous => {
                todo!()
            }
            Input::NotRecognized => {}
        }
    }
}

impl Widget for &Game {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        use Constraint::*;

        Block::bordered().render(area, buf);
        let area = area.inner(Margin::new(1, 1));

        let status_line = Layout::vertical([Fill(1), Length(1)]);
        let layout = status_line.split(area);
        let (top, status_line) = (layout[0], layout[1]);

        Paragraph::new(format!(
            "[q] | [echap] to quit \t [->] to do a step in execution"
        ))
        .centered()
        .render(status_line, buf);

        let left_right = Layout::horizontal([Fill(1), Fill(1)]);
        let top_bottom = Layout::vertical([Fill(1), Fill(1)]);

        let layout = left_right.split(top);
        let (left, right) = (layout[0], layout[1]);

        let layout = top_bottom.split(right);
        let (top_right, bottom_right) = (layout[0], layout[1]);

        Paragraph::new(format!(
            "{}\n\n\n{}",
            self.interpreter.current, self.interpreter.continuation
        ))
        // .wrap(Wrap { trim: true })
        .block(Block::bordered())
        .render(left, buf);

        Paragraph::new(format!("{}", self.interpreter.memory.stack))
            .block(Block::bordered())
            .render(top_right, buf);

        self.interpreter.memory.heap.render(bottom_right, buf);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let contents = get_string_from_path(&args.path);
    let mut game = Game::new(&contents);

    ratatui::run(|terminal| {
        terminal.draw(|frame| (&game).render(frame.area(), frame.buffer_mut()))?;
        loop {
            if poll(Duration::from_millis(100))? {
                let event = read()?;
                if let Event::Key(KeyEvent {
                    code: KeyCode::Esc | KeyCode::Char('q'),
                    ..
                }) = event
                {
                    break;
                }
                game.interact(Input::try_from(event).ok());
                terminal.draw(|frame| (&game).render(frame.area(), frame.buffer_mut()))?;
            }
        }
        Ok(())
    })
}
