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
include!("app.rs");

fn main() -> std::io::Result<()> {
    // let info = MockExecuter {}.retrieve_info();
    let info = MockExecuter {};
    ratatui::run(|terminal| App::default(info).run(terminal))
    // if !info {
    //     ratatui::run(|terminal| App::default(info).run(terminal))
    // } else {
    //     println!("Failed to retrieve info: {}", info.err().unwrap());
    //     Ok(())
    // }
}
