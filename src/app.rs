use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Modifier, Style};
use ratatui::widgets::{Borders, Padding, Row, Table, Wrap};

fn friendly_label(raw: &str) -> &str {
    match raw {
        "uptime" => "Uptime",
        "df" => "Storage",
        "REVISION" => "Revision",
        "git-log" => "Recent Commit",
        "git-branch" => "Branch",
        "status" => "Status",
        "releases" => "Releases",
        other => other,
    }
}

pub enum AppState {
    Dashboard,
    ServerDetail,
}

pub struct App {
    state: AppState,
    selected_server_index: usize,
    exit: bool,
    executer: Option<MockExecuter>,
}

impl App {
    fn default(executer: MockExecuter) -> Self {
        Self {
            selected_server_index: 0,
            exit: false,
            executer: Some(executer),
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
        self.ui(frame);
    }

    fn handle_events(&mut self) -> std::io::Result<()> {
        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(KeyEvent { code, kind, .. }) = event::read()? {
                if kind == KeyEventKind::Press {
                    match code {
                        KeyCode::Char('q') => self.exit = true,
                        KeyCode::Esc => {
                            self.state = AppState::Dashboard;
                        }
                        KeyCode::Enter => {
                            self.state = AppState::ServerDetail;
                        }
                        KeyCode::Up => {
                            if self.selected_server_index > 0 {
                                self.selected_server_index -= 1;
                            }
                        }
                        KeyCode::Down => {
                            if self.selected_server_index + 1
                                < self.executer.as_ref().unwrap().server_list().len()
                            {
                                self.selected_server_index += 1;
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
        Ok(())
    }

    fn ui(&mut self, frame: &mut Frame) {
        match self.state {
            AppState::Dashboard => self.draw_dashboard(frame),
            AppState::ServerDetail => self.draw_server(frame),
        }
    }

    fn draw_server(&mut self, frame: &mut Frame) {
        let file_content_list: Vec<FileContent> = self
            .executer
            .as_ref()
            .unwrap()
            .file_content_list(self.selected_server_index);
        let item_count = file_content_list.len();

        let server_name = self
            .executer
            .as_ref()
            .unwrap()
            .server_list()[self.selected_server_index]
            .trim_end_matches('/')
            .to_string();

        let [toolbar, content, footer] = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(0),
                Constraint::Length(1),
            ])
            .areas(frame.area());

        frame.render_widget(
            Block::default()
                .title(server_name)
                .title_alignment(ratatui::layout::Alignment::Center)
                .borders(Borders::ALL),
            toolbar,
        );

        frame.render_widget(
            Paragraph::new(" Esc: back to dashboard  |  q: quit"),
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
                    .title(friendly_label(&file_content_list[index].label))
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

    fn draw_dashboard(&mut self, frame: &mut Frame) {
        // show list of servers in a table
        let block = Block::default()
            .title("Servers")
            .title_alignment(ratatui::layout::Alignment::Center)
            .borders(Borders::ALL)
            .padding(Padding {
                top: 1,
                bottom: 1,
                left: 2,
                right: 2,
            });
        let server_list = self.executer.as_ref().unwrap().server_list();
        let mut rows: Vec<Row> = Vec::new();
        if server_list.is_empty() {
            rows.push(Row::new(vec!["No servers found"]));
        } else {
            for server in server_list {
                rows.push(Row::new(vec![server]).style(
                    if self.selected_server_index == rows.len() {
                        Style::default().add_modifier(Modifier::REVERSED)
                    } else {
                        Style::default()
                    },
                ));
            }
        }
        let table = Table::new(rows, [Constraint::Percentage(100)])
            .block(block)
            .highlight_symbol(">> ");
        frame.render_widget(table, frame.area());
    }
}
