use clap::Parser;
use compiler::{
    api::SmolAPI, ast::AST, ast_interpreter::interpreter::Interpreter, parser, type_check,
};
use ratatui::{
    crossterm::event::{Event, KeyCode, KeyEvent, poll, read},
    layout::{Constraint, Layout, Margin},
    prelude::{Buffer, Rect},
    style::Style,
    widgets::{Block, Paragraph, ScrollbarState, Tabs, Widget},
};
use std::time::Duration;
use tui::{args::Args, cli_api::CLI, input::Input};

const TABS_NUMBER: usize = 7;
const TABS_NAMES: [&str; TABS_NUMBER] = [
    "Source File",
    "AST",
    "AST Dbg",
    "AST Checked",
    "AST Checked Dbg",
    "C Backend",
    "Interpreter",
];

pub struct State<'src, T: SmolAPI> {
    scrollbar: ScrollbarState,
    tabs_selection: usize,
    source_file: &'src str,
    ast: AST<'src>,
    ast_type_checked: AST<'src>,
    interpreter: Interpreter<'src, T>,
}

impl<'src, T: SmolAPI> State<'src, T> {
    pub fn new(source_file: &'src str, api: T) -> State<'src, T> {
        let ast = parser(source_file).unwrap_or_else(|e| {
            panic!("Failed to parse this file '{source_file}'.\nGot this error : \n{e}")
        });

        let mut ast_type_checked = ast.clone();
        let ast_type = type_check(&mut ast_type_checked);

        Self {
            scrollbar: ScrollbarState::new(4000),
            tabs_selection: 0,
            source_file,
            interpreter: Interpreter::new(ast.clone(), true, api),
            ast,
            ast_type_checked,
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
            Input::TabRight => {
                self.scrollbar = self.scrollbar.position(0);
                self.tabs_selection = ((self.tabs_selection + 1) % TABS_NUMBER)
            }
            Input::TabLeft => {
                self.scrollbar = self.scrollbar.position(0);
                self.tabs_selection = if self.tabs_selection == 0 {
                    TABS_NUMBER - 1
                } else {
                    self.tabs_selection - 1
                }
            }
            Input::ScrollDown => self.scrollbar.next(),
            Input::ScrollUp => self.scrollbar.prev(),
        }
    }
}

impl<T: SmolAPI> Widget for &State<'_, T> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        use Constraint::*;

        Block::bordered().render(area, buf);
        let area = area.inner(Margin::new(1, 1));

        let status_line = Layout::vertical([Length(3), Fill(1), Length(2)]);
        let layout = status_line.split(area);
        let (top_line, middle, bottom_line) = (layout[0], layout[1], layout[2]);

        Tabs::new(TABS_NAMES)
            .block(Block::bordered().title("Tabs"))
            .style(Style::default().white())
            .highlight_style(Style::default().blue())
            .select(self.tabs_selection)
            .divider("  ")
            .padding("[", "]")
            .render(top_line, buf);

        match self.tabs_selection {
            0 => Paragraph::new(format!("{}", self.source_file))
                .scroll((self.scrollbar.get_position() as u16, 0))
                .render(middle, buf),
            1 => Paragraph::new(format!("{}", self.ast))
                .scroll((self.scrollbar.get_position() as u16, 0))
                .render(middle, buf),
            2 => Paragraph::new(format!("{:#?}", self.ast))
                .scroll((self.scrollbar.get_position() as u16, 0))
                .render(middle, buf),
            3 => Paragraph::new(format!("{}", self.ast_type_checked))
                .scroll((self.scrollbar.get_position() as u16, 0))
                .render(middle, buf),
            4 => Paragraph::new(format!("{:#?}", self.ast_type_checked))
                .scroll((self.scrollbar.get_position() as u16, 0))
                .render(middle, buf),
            5 => Paragraph::new(format!("C generated code not implemented yet"))
                .scroll((self.scrollbar.get_position() as u16, 0))
                .render(middle, buf),
            6 => {
                let left_right = Layout::horizontal([Fill(1), Fill(1)]);

                let layout = left_right.split(middle);
                let (left, right) = (layout[0], layout[1]);

                Paragraph::new(format!("{}", self.interpreter))
                    .scroll((self.scrollbar.get_position() as u16, 0))
                    .block(Block::bordered())
                    .render(left, buf);

                Paragraph::new(format!("{}", self.interpreter.memory))
                    .scroll((self.scrollbar.get_position() as u16, 0))
                    .block(Block::bordered())
                    .render(right, buf);
            }
            TABS_NUMBER => {}
            _ => {}
        }
        Paragraph::new("[q]     [echap] quit    [<-]/[->] step interpreter     [alt] + [<-]/[->] change tabs     [↑]/[↓] scrool up/down".to_string()).block(
            Block::bordered()
            )
            .centered()
            .           render(Rect::new(

                bottom_line.x - 1, bottom_line.y, bottom_line.width + 2, bottom_line.height + 1), buf);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let contents = std::fs::read_to_string(&args.path)
        .unwrap_or_else(|e| panic!("Error reading '{}'.\n{}", args.path, e));
    let api_tui = CLI::new();
    let mut state = State::new(&contents, api_tui);

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
