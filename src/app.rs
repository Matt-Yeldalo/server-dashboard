use ratatui::layout::{Alignment, Constraint, Direction, Layout};
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
    ServerView,
    DeploymentDetail,
}

pub struct App {
    state: AppState,
    selected_server_index: usize,
    selected_deployment_index: usize,
    exit: bool,
    executer: Option<MockExecuter>,
}

impl App {
    fn default(executer: MockExecuter) -> Self {
        Self {
            selected_server_index: 0,
            selected_deployment_index: 0,
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
        match self.state {
            AppState::Dashboard => self.draw_dashboard(frame),
            AppState::ServerView => self.draw_server_view(frame),
            AppState::DeploymentDetail => self.draw_deployment_detail(frame),
        }
    }

    fn handle_events(&mut self) -> std::io::Result<()> {
        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(KeyEvent { code, kind, .. }) = event::read()? {
                if kind == KeyEventKind::Press {
                    match self.state {
                        AppState::Dashboard => match code {
                            KeyCode::Char('q') => self.exit = true,
                            KeyCode::Enter => {
                                self.selected_deployment_index = 0;
                                self.state = AppState::ServerView;
                            }
                            KeyCode::Up => {
                                if self.selected_server_index > 0 {
                                    self.selected_server_index -= 1;
                                }
                            }
                            KeyCode::Down => {
                                let server_count =
                                    self.executer.as_ref().unwrap().server_list().len();
                                if self.selected_server_index + 1 < server_count {
                                    self.selected_server_index += 1;
                                }
                            }
                            _ => {}
                        },
                        AppState::ServerView => match code {
                            KeyCode::Char('q') => self.exit = true,
                            KeyCode::Esc => self.state = AppState::Dashboard,
                            KeyCode::Enter => self.state = AppState::DeploymentDetail,
                            KeyCode::Up => {
                                if self.selected_deployment_index > 0 {
                                    self.selected_deployment_index -= 1;
                                }
                            }
                            KeyCode::Down => {
                                let deployment_count = self
                                    .executer
                                    .as_ref()
                                    .unwrap()
                                    .deployment_list(self.selected_server_index)
                                    .len();
                                if self.selected_deployment_index + 1 < deployment_count {
                                    self.selected_deployment_index += 1;
                                }
                            }
                            _ => {}
                        },
                        AppState::DeploymentDetail => match code {
                            KeyCode::Char('q') => self.exit = true,
                            KeyCode::Esc => self.state = AppState::ServerView,
                            _ => {}
                        },
                    }
                }
            }
        }
        Ok(())
    }

    fn draw_dashboard(&mut self, frame: &mut Frame) {
        let block = Block::default()
            .title("Servers")
            .title_alignment(Alignment::Center)
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

    fn draw_server_view(&mut self, frame: &mut Frame) {
        let server_name =
            self.executer.as_ref().unwrap().server_list()[self.selected_server_index].clone();

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
                .title(server_name.clone())
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL),
            toolbar,
        );

        frame.render_widget(
            Paragraph::new(" Esc: back  |  Enter: open deployment  |  q: quit"),
            footer,
        );

        let [left, right] = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
            .areas(content);

        // Host-level panels (uptime, df) stacked on the left
        let host_content = self
            .executer
            .as_ref()
            .unwrap()
            .server_host_content(self.selected_server_index);
        let panel_count = host_content.len().max(1) as u32;
        let panel_constraints = vec![Constraint::Ratio(1, panel_count); host_content.len()];
        let panel_areas = Layout::default()
            .direction(Direction::Vertical)
            .constraints(panel_constraints)
            .split(left);

        for (i, area) in panel_areas.iter().enumerate() {
            let block = Block::default()
                .title(friendly_label(&host_content[i].label))
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .padding(Padding {
                    top: 1,
                    bottom: 1,
                    left: 2,
                    right: 2,
                });
            let paragraph = Paragraph::new(host_content[i].content.clone())
                .wrap(Wrap { trim: true })
                .block(block);
            frame.render_widget(paragraph, *area);
        }

        // Deployment list on the right
        let deployments = self
            .executer
            .as_ref()
            .unwrap()
            .deployment_list(self.selected_server_index);
        let deployment_block = Block::default()
            .title("Deployments")
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .padding(Padding {
                top: 1,
                bottom: 1,
                left: 2,
                right: 2,
            });

        let mut rows: Vec<Row> = Vec::new();
        if deployments.is_empty() {
            rows.push(Row::new(vec!["No deployments found"]));
        } else {
            for deployment in deployments {
                rows.push(Row::new(vec![deployment]).style(
                    if self.selected_deployment_index == rows.len() {
                        Style::default().add_modifier(Modifier::REVERSED)
                    } else {
                        Style::default()
                    },
                ));
            }
        }

        let table = Table::new(rows, [Constraint::Percentage(100)]).block(deployment_block);
        frame.render_widget(table, right);
    }

    fn draw_deployment_detail(&mut self, frame: &mut Frame) {
        let server_name =
            self.executer.as_ref().unwrap().server_list()[self.selected_server_index].clone();
        let deployments = self
            .executer
            .as_ref()
            .unwrap()
            .deployment_list(self.selected_server_index);
        let deployment_name = deployments[self.selected_deployment_index].clone();
        let title = format!("{} > {}", server_name, deployment_name);

        let content_list = self
            .executer
            .as_ref()
            .unwrap()
            .deployment_content(self.selected_server_index, self.selected_deployment_index);
        let item_count = content_list.len();

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
                .title(title)
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL),
            toolbar,
        );

        frame.render_widget(
            Paragraph::new(format!(" Esc: back to {}  |  q: quit", server_name)),
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
                    .title(friendly_label(&content_list[index].label))
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL)
                    .padding(Padding {
                        top: 1,
                        bottom: 1,
                        left: 2,
                        right: 2,
                    });
                let paragraph = Paragraph::new(content_list[index].content.clone())
                    .alignment(Alignment::Left)
                    .wrap(Wrap { trim: true })
                    .block(block);

                frame.render_widget(paragraph, *col);
                index += 1;
            }
        }
    }
}
