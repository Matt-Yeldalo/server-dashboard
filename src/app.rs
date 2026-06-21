use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::widgets::{Borders, Padding, Wrap};
pub enum AppState {
    Dashboard,
    ServerDetail,
}

pub struct App {
    state: AppState,
    selected_server_index: usize,
    exit: bool,
    display_info: Option<MockExecuter>,
}

impl App {
    fn default(display_info: MockExecuter) -> Self {
        Self {
            selected_server_index: 0,
            exit: false,
            display_info: Some(display_info),
            state: AppState::Dashboard,
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> std::io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
        // frame.render_widget(self, frame.area());
        self.ui(frame, 6);
    }

    fn handle_events(&mut self) -> std::io::Result<()> {
        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(KeyEvent { code, kind, .. }) = event::read()? {
                if kind == KeyEventKind::Press {
                    match code {
                        KeyCode::Char('q') => self.exit = true,
                        // KeyCode::Char('c') => self.counter = self.counter.wrapping_add(1),
                        _ => {}
                    }
                }
            }
        }
        Ok(())
    }

    fn ui(&mut self, frame: &mut Frame, item_count: usize) {
        // Split screen into toolbar, content, footer
        let [toolbar, content, footer] = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(0),
                Constraint::Length(1),
            ])
            .areas(frame.area());

        frame.render_widget(
            Block::default().title("Toolbar").borders(Borders::ALL),
            toolbar,
        );

        frame.render_widget(
            Block::default().title("Footer").borders(Borders::ALL),
            footer,
        );

        let columns = 3;
        let rows = item_count.div_ceil(columns);

        let row_constraints = vec![Constraint::Ratio(1, rows as u32); rows];
        let row_areas = Layout::default()
            .direction(Direction::Vertical)
            .constraints(row_constraints)
            .split(content);

        let mut index = 0;
        let file_content_list: Vec<FileContent> =
            self.display_info.as_ref().unwrap().file_content_list();

        for row in row_areas.iter() {
            let col_constraints = vec![Constraint::Ratio(1, columns as u32); columns];
            let cols = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(col_constraints)
                .split(*row);

            for col in cols.iter() {
                if index >= item_count {
                    break;
                }

                let block = Block::default()
                    .title(file_content_list[index].label.to_string())
                    .title_alignment(ratatui::layout::Alignment::Center)
                    .borders(Borders::ALL)
                    .padding(Padding {
                        top: 1,
                        bottom: 1,
                        left: 2,
                        right: 2,
                    });
                let paragraph = Paragraph::new(file_content_list[index].content.clone())
                    .alignment(ratatui::layout::Alignment::Left)
                    .wrap(Wrap { trim: true })
                    .block(block);

                frame.render_widget(paragraph, *col);

                index += 1;
            }
        }
    }
}

// impl Widget for &App {
// fn render(self, area: Rect, buf: &mut Buffer) {
//     // Split screen into toolbar, content, footer
//     let [toolbar, content, footer] = Layout::default()
//         .direction(Direction::Vertical)
//         .constraints([
//             Constraint::Length(3),
//             Constraint::Min(0),
//             Constraint::Length(1),
//         ])
//         .areas(area);
//
//     area.render_widget(
//         Block::default().title("Toolbar").borders(Borders::ALL),
//         toolbar,
//     );
//
//     frame.render_widget(
//         Block::default().title("Footer").borders(Borders::ALL),
//         footer,
//     );
//
//     let columns = 3;
//     let rows = item_count.div_ceil(columns);
//
//     let row_constraints = vec![Constraint::Ratio(1, rows as u32); rows];
//     let row_areas = Layout::default()
//         .direction(Direction::Vertical)
//         .constraints(row_constraints)
//         .split(content);
//
//     let mut index = 0;
//
//     for row in row_areas.iter() {
//         let col_constraints = vec![Constraint::Ratio(1, columns as u32); columns];
//         let cols = Layout::default()
//             .direction(Direction::Horizontal)
//             .constraints(col_constraints)
//             .split(row);
//
//         for col in cols.iter() {
//             if index >= item_count {
//                 break;
//             }
//
//             frame.render_widget(
//                 Block::default()
//                     .title(format!("Item {}", index + 1))
//                     .borders(Borders::ALL),
//                 *col,
//             );
//
//             index += 1;
//         }
//     }
// }
// fn render(self, area: Rect, buf: &mut Buffer) {
// self.display_info.as_ref().map(|info| {
//     let grid = Table::new(vec![
//         vec![
//             Cell::new("Uptime").bold().yellow(),
//             Cell::new(info.uptime.content).yellow(),
//         ],
//         vec![
//             Cell::new("Storage").bold().yellow(),
//             Cell::new(info.storage.content).yellow(),
//         ],
//         vec![
//             Cell::new("Revision").bold().yellow(),
//             Cell::new(info.revision.content).yellow(),
//         ],
//         vec![
//             Cell::new("Git Log").bold().yellow(),
//             Cell::new(info.git_log.content).yellow(),
//         ],
//         vec![
//             Cell::new("Git Branch").bold().yellow(),
//             Cell::new(info.git_branch.content).yellow(),
//         ],
//         vec![
//             Cell::new("Status").bold().yellow(),
//             Cell::new(info.status.content).yellow(),
//         ],
//     ]);
//
//     let content_block = Block::bordered()
//         .title("System Info")
//         .border_set(border::THICK);
//
//     grid.centered().block(content_block).render(area, buf);
// });

//     let title = Line::from("Server Dashboard".bold());
//     let instructions = Line::from(vec![
//         " Decrement ".into(),
//         "<Left>".blue().bold(),
//         " Increment ".into(),
//         "<Right>".blue().bold(),
//         " Quit ".into(),
//         "<Q> ".blue().bold(),
//     ]);
//
//     let block = Block::bordered()
//         .title(title.centered())
//         .title_bottom(instructions.centered())
//         .border_set(border::THICK);
//
//     let selected_text = Text::from(vec![Line::from(vec![
//         "Value: ".into(),
//         self.selected_server_index.to_string().yellow(),
//     ])]);
//
//     Paragraph::new(selected_text)
//         .centered()
//         .block(block)
//         .render(area, buf);
// }
// }
