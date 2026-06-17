use ratatui::{DefaultTerminal, Frame};
// include!("cli.rs");
include!("mock_executer.rs");

fn main() -> color_eyre::Result<()> {
    // let cli = Cli::parse();
    // if let Some(name) = cli.name {
    //     println!("Value for name: {name}");
    // }
    color_eyre::install()?;
    ratatui::run(app)?;
    Ok(())
}

fn app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    loop {
        terminal.draw(render)?;
        if crossterm::event::read()?.is_key_press() {
            break Ok(());
        }
    }
}

fn render(frame: &mut Frame) {
    frame.render_widget("hello world", frame.area());
    let info = MockExecuter {}.retrieve_info();
    if let Ok(info) = info {
        for content in info.iter() {
            frame.render_widget(format!("{}", content), frame.area());
        }
    } else {
        println!("Failed to retrieve info: {}", info.err().unwrap());
    }
}
