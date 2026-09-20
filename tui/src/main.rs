use ast::{AST, cst_to_ast};
use ast_interpreter::interpreter::Interpreter;
use clap::Parser;
use cst::{CST, cst_parser};
use ratatui::{
    crossterm::event::{Event, KeyCode, KeyEvent, poll, read},
    layout::{Constraint, Layout, Margin},
    prelude::{Buffer, Rect},
    style::Style,
    symbols,
    widgets::{Block, Paragraph, Tabs, Widget},
};
use std::time::Duration;
use tui::{args::Args, input::Input};

const TABS_NUMBER: usize = 5;
const TABS_NAMES: [&str; TABS_NUMBER] = [
    "Source File",
    "CST",
    "AST",
    "C Generated Code",
    "Interpreter",
];

pub struct State<'src> {
    tabs_selection: usize,
    source_file: &'src str,
    cst: CST<'src>,
    ast: AST,
    interpreter: Interpreter,
}

impl<'src> State<'src> {
    pub fn new(source_file: &'src str) -> State<'src> {
        let cst = cst_parser(source_file).unwrap_or_else(|e| {
            panic!("Failed to parse this file '{source_file}'.\nGot this error : \n{e}")
        });
        let ast = cst_to_ast(&cst);

        Self {
            tabs_selection: 0,
            source_file,
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
            Input::InterpreterNext => {
                self.interpreter.next();
            }
            Input::InterpreterBack => {
                self.interpreter.next_back();
            }
            Input::TabRight => self.tabs_selection = ((self.tabs_selection + 1) % TABS_NUMBER),
            Input::TabLeft => {
                self.tabs_selection = if self.tabs_selection == 0 {
                    TABS_NUMBER - 1
                } else {
                    self.tabs_selection - 1
                }
            }
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

        let status_line = Layout::vertical([Length(3), Fill(1), Length(1)]);
        let layout = status_line.split(area);
        let (top_line, middle, bottom_line) = (layout[0], layout[1], layout[2]);

        Tabs::new(TABS_NAMES)
            .block(Block::bordered().title("Tabs"))
            .style(Style::default().white())
            .highlight_style(Style::default().blue())
            .select(self.tabs_selection)
            .divider(symbols::DOT)
            .padding("->", "<-")
            .render(top_line, buf);

        match self.tabs_selection {
            0 => Paragraph::new(format!("{}", self.source_file)).render(middle, buf),
            1 => Paragraph::new(format!("{}", self.cst)).render(middle, buf),
            2 => Paragraph::new(format!("{}", self.ast)).render(middle, buf),
            3 => {
                Paragraph::new(format!("C generated code not implemented yet")).render(middle, buf)
            }
            4 => {
                let left_right = Layout::horizontal([Fill(1), Fill(1)]);

                let layout = left_right.split(middle);
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
            TABS_NUMBER => {}
            _ => {}
        }

        Paragraph::new("[q] | [echap] to quit \t [<-] | [->] step in interpreter \t [alt] + ([<-] | [->]) change tabs".to_string())
            .centered()
            .render(bottom_line, buf);
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
