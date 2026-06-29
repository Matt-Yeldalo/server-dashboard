use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Modifier, Style};
use ratatui::widgets::{Borders, Padding, Row, Table, Wrap};
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
        self.ui(frame, 6);
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

    fn ui(&mut self, frame: &mut Frame, item_count: usize) {
        match self.state {
            AppState::Dashboard => self.draw_dashboard(frame, item_count),
            AppState::ServerDetail => self.draw_server(frame, item_count),
        }
    }

    fn draw_server(&mut self, frame: &mut Frame, item_count: usize) {
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
        let file_content_list: Vec<FileContent> = self
            .executer
            .as_ref()
            .unwrap()
            .file_content_list(self.selected_server_index);

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

    fn draw_dashboard(&mut self, frame: &mut Frame, _item_count: usize) {
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
        let server_length = server_list.len();
        let mut rows = Vec::new();
        match server_length {
            0 => {
                rows.push(Row::new(vec!["No servers found"]));
            }
            1 => {
                rows.push(Row::new(vec![server_list[0].as_str()]));
            }
            _ => {
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
        }
        // for server in server_list {
        //     if server_list.len() > 1 {
        //         rows.push(Row::new(vec![server]).style(
        //             if self.selected_server_index == rows.len() {
        //                 Style::default().add_modifier(Modifier::REVERSED)
        //             } else {
        //                 Style::default()
        //             },
        //         ));
        //     } else {
        //         rows.push(Row::new(vec![server]));
        //     }
        // }
        // let rows = [Row::new(self.executer.as_ref().unwrap().server_list())];
        let widths = [Constraint::Percentage(100)];
        let table = Table::new(rows, widths)
            .block(block)
            .highlight_symbol(">> ")
            .widths(&[Constraint::Percentage(100)]);
        frame.render_widget(table, frame.area());
    }
}
