use ast::{AST, cst_to_ast};
use ast_interpreter::interpreter::Interpreter;
use clap::Parser;
use cst::{CST, cst_parser};
use ratatui::{
    crossterm::event::{Event, KeyCode, KeyEvent, poll, read},
    layout::{Constraint, Layout, Margin},
    prelude::{Buffer, Rect},
    widgets::{Block, Paragraph, Widget},
};
use std::time::Duration;
use tui::{args::Args, input::Input};

pub struct State<'src> {
    original_file: &'src str,
    cst: CST<'src>,
    ast: AST,
    interpreter: Interpreter,
}

impl<'src> State<'src> {
    pub fn new(original_file: &'src str) -> State<'src> {
        let cst = cst_parser(original_file).unwrap_or_else(|e| {
            panic!("Failed to parse this file '{original_file}'.\nGot this error : \n{e}")
        });
        let ast = cst_to_ast(&cst);

        Self {
            original_file,
            interpreter: Interpreter::new(ast.clone()),
            cst,
            ast,
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
                self.interpreter.next_back();
            }
            Input::NotRecognized => {}
        }
    }
}

impl Widget for &State<'_> {
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

        Paragraph::new("[q] | [echap] to quit \t [->] to do a step in execution".to_string())
            .centered()
            .render(status_line, buf);

        let left_right = Layout::horizontal([Fill(1), Fill(1)]);

        let layout = left_right.split(top);
        let (left, right) = (layout[0], layout[1]);

        Paragraph::new(format!(
            "{}\n\n[κ]\n{}\n\n[Step] {}",
            self.interpreter.state, self.interpreter.continuation, self.interpreter.step
        ))
        // .wrap(Wrap { trim: true })
        .block(Block::bordered())
        .render(left, buf);

        Paragraph::new(format!("{}", self.interpreter.memory))
            .block(Block::bordered())
            .render(right, buf);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let contents = std::fs::read_to_string(&args.path)
        .unwrap_or_else(|e| panic!("Error reading '{}'.\n{}", args.path, e));
    let mut state = State::new(&contents);

    ratatui::run(|terminal| {
        terminal.draw(|frame| (&state).render(frame.area(), frame.buffer_mut()))?;
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
                state.interact(Input::try_from(event).ok());
                terminal.draw(|frame| state.render(frame.area(), frame.buffer_mut()))?;
            }
        }
        Ok(())
    })
}
