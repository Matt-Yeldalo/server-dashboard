use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
};
// include!("cli.rs");
include!("mock_executer.rs");

fn main() -> std::io::Result<()> {
    let info = MockExecuter {}.retrieve_info();
    if let Ok(info) = info {
        ratatui::run(|terminal| App::default(info).run(terminal))
    } else {
        println!("Failed to retrieve info: {}", info.err().unwrap());
        Ok(())
    }
}

pub struct App {
    counter: u8,
    exit: bool,
    display_info: Option<DisplayInfo>,
}

impl App {
    fn default(display_info: DisplayInfo) -> Self {
        Self {
            counter: 0,
            exit: false,
            display_info: Some(display_info),
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> std::io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> std::io::Result<()> {
        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(KeyEvent { code, kind, .. }) = event::read()? {
                if kind == KeyEventKind::Press {
                    match code {
                        KeyCode::Char('q') => self.exit = true,
                        KeyCode::Char('c') => self.counter = self.counter.wrapping_add(1),
                        _ => {}
                    }
                }
            }
        }
        Ok(())
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.display_info.as_ref().map(|info| {
            let content = Text::from(vec![
                Line::from(format!("Uptime: {}", info.uptime.content)),
                Line::from(format!("Storage: {}", info.storage.content)),
                Line::from(format!("Revision: {}", info.revision.content)),
                Line::from(format!("Git Log: {}", info.git_log.content)),
                Line::from(format!("Git Branch: {}", info.git_branch.content)),
                Line::from(format!("Status: {}", info.status.content)),
            ]);

            Paragraph::new(content)
                .centered()
                .block(
                    Block::bordered()
                        .title("System Info")
                        .border_set(border::THICK),
                )
                .render(area, buf);
        });

        let title = Line::from(" Counter App Tutorial ".bold());
        let instructions = Line::from(vec![
            " Decrement ".into(),
            "<Left>".blue().bold(),
            " Increment ".into(),
            "<Right>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]);

        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        let counter_text = Text::from(vec![Line::from(vec![
            "Value: ".into(),
            self.counter.to_string().yellow(),
        ])]);

        Paragraph::new(counter_text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}
