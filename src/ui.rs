use resize::{Pixel::RGB8, px::RGB};
use resize::Type::Lanczos3;
use crate::{config::Theme, state::Window};

use super::state::{ProgressState, State, MessageType, SystemMessageType};
use super::commands::{CommandManager};
use super::util::{split_each};

use tui::backend::CrosstermBackend;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, Cell, Paragraph, Row, Table, Wrap};
use tui::Frame;

use std::io::Write;
use crate::cardascii::terminal::draw_hand_from_vec_cards;

pub fn draw(
    frame: &mut Frame<CrosstermBackend<impl Write>>,
    state: &State,
    chunk: Rect,
    theme: &Theme,
) {
    let v_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)].as_ref())
        .split(chunk);

    let upper_chunk = v_chunks[0];
    draw_messages_panel(frame, state, v_chunks[0], theme);
    
    let upper_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(15), Constraint::Length(31)].as_ref())
        .split(upper_chunk);
    draw_messages_panel(frame, state, upper_chunks[0], theme);
    //draw_video_panel(frame, state, upper_chunks[1]);
    if let Some(game) = &state.game24 {
        draw_card_panel(frame, upper_chunks[1], & draw_hand_from_vec_cards(& game.get_gived_cards()));
    }
    else {
        draw_card_panel(frame, upper_chunks[1], & state.cards);
    }
    draw_input_panel(frame, state, v_chunks[1], theme);
}

fn draw_messages_panel(
    frame: &mut Frame<CrosstermBackend<impl Write>>,
    state: &State,
    chunk: Rect,
    theme: &Theme,
) {
    let message_colors = &theme.message_colors;

    let messages = state
        .messages()
        .iter()
        .rev()
        .map(|message| {
            let color = if let Some(id) = state.users_id().get(&message.user) {
                message_colors[id % message_colors.len()]
            } else {
                theme.my_user_color
            };
            let date = message.date.format("%H:%M:%S ").to_string();
            match &message.message_type {
                MessageType::Connection => Spans::from(vec![
                    Span::styled(date, Style::default().fg(theme.date_color)),
                    Span::styled(&message.user, Style::default().fg(color)),
                    Span::styled(" is online", Style::default().fg(color)),
                ]),
                MessageType::Disconnection => Spans::from(vec![
                    Span::styled(date, Style::default().fg(theme.date_color)),
                    Span::styled(&message.user, Style::default().fg(color)),
                    Span::styled(" is offline", Style::default().fg(color)),
                ]),
                MessageType::Text(content) => {
                    let mut ui_message = vec![
                        Span::styled(date, Style::default().fg(theme.date_color)),
                        Span::styled(&message.user, Style::default().fg(color)),
                        Span::styled(": ", Style::default().fg(color)),
                    ];
                    ui_message.extend(parse_content(content, theme));
                    Spans::from(ui_message)
                }
                MessageType::System(content, msg_type) => {
                    let (user_color, content_color) = match msg_type {
                        SystemMessageType::Info => theme.system_info_color,
                        SystemMessageType::Warning => theme.system_warning_color,
                        SystemMessageType::Error => theme.system_error_color,
                    };
                    Spans::from(vec![
                        Span::styled(date, Style::default().fg(theme.date_color)),
                        Span::styled(&message.user, Style::default().fg(user_color)),
                        Span::styled(content, Style::default().fg(content_color)),
                    ])
                }
                MessageType::Progress(state) => {
                    Spans::from(add_progress_bar(chunk.width, state, theme))
                }
            }
        })
        .collect::<Vec<_>>();

    let messages_panel = Paragraph::new(messages)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(Span::styled("LAN Room", Style::default().add_modifier(Modifier::BOLD))),
        )
        .style(Style::default().fg(theme.chat_panel_color))
        .alignment(Alignment::Left)
        .scroll((state.scroll_messages_view() as u16, 0))
        .wrap(Wrap { trim: false });

    frame.render_widget(messages_panel, chunk);
}

fn add_progress_bar<'a>(
    panel_width: u16,
    progress: &'a ProgressState,
    theme: &Theme,
) -> Vec<Span<'a>> {
    let color = theme.progress_bar_color;
    let width = (panel_width - 20) as usize;

    let (title, ui_current, ui_remaining) = match progress {
        ProgressState::Started(_) => ("Pending: ", 0, width),
        ProgressState::Working(total, current) => {
            let percentage = *current as f64 / *total as f64;
            let ui_current = (percentage * width as f64) as usize;
            let ui_remaining = width - ui_current;
            ("Sending: ", ui_current, ui_remaining)
        }
        ProgressState::Completed => ("Done! ", width, 0),
    };

    let current: String = "#".repeat(ui_current);
    let remaining: String = "-".repeat(ui_remaining);

    let msg = format!("[{}{}]", current, remaining);
    let ui_message = vec![
        Span::styled(title, Style::default().fg(color)),
        Span::styled(msg, Style::default().fg(color)),
    ];
    ui_message
}

fn parse_content<'a>(content: &'a str, theme: &Theme) -> Vec<Span<'a>> {
    if content.starts_with(CommandManager::COMMAND_PREFIX) {
        // The content represents a command
        content
            .split_whitespace()
            .enumerate()
            .map(|(index, part)| {
                if index == 0 {
                    Span::styled(part, Style::default().fg(theme.command_color))
                } else {
                    Span::raw(format!(" {}", part))
                }
            })
            .collect()
    } else {
        vec![Span::raw(content)]
    }
}

fn draw_input_panel(
    frame: &mut Frame<CrosstermBackend<impl Write>>,
    state: &State,
    chunk: Rect,
    theme: &Theme,
) {
    let inner_width = (chunk.width - 2) as usize;

    let input = state.input().iter().collect::<String>();
    let input = split_each(input, inner_width)
        .into_iter()
        .map(|line| Spans::from(vec![Span::raw(line)]))
        .collect::<Vec<_>>();

    let input_panel = Paragraph::new(input)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(Span::styled("Your message", Style::default().add_modifier(Modifier::BOLD))),
        )
        .style(Style::default().fg(theme.input_panel_color))
        .alignment(Alignment::Left);

    frame.render_widget(input_panel, chunk);

    let input_cursor = state.ui_input_cursor(inner_width);
    frame.set_cursor(chunk.x + 1 + input_cursor.0, chunk.y + 1 + input_cursor.1)
}

/*fn draw_video_panel(frame: &mut Frame<CrosstermBackend<impl Write>>, state: &State, chunk: Rect) {
    let windows = state.windows.values().collect();
    let fb = FrameBuffer::new(windows).block(Block::default().borders(Borders::ALL));
    frame.render_widget(fb, chunk);
}*/

fn draw_card_panel(frame: &mut Frame<CrosstermBackend<impl Write>>, chunk: Rect, visual_cards: &Vec<Vec<String>>) {
    let rows = visual_cards.iter().map(|v_card| {
        let height = v_card
            .iter()
            .map(|content| content.chars().filter(|c| *c == '\n').count())
            .max()
            .unwrap_or(1);

        let cells = v_card.iter().map(|string| Cell::from(string.as_str()));

        Row::new(cells).height(height as u16).bottom_margin(1)
    });

    let t = Table::new(rows)
        .block(Block::default().borders(Borders::ALL).title("Cards"))
        .widths(&[Constraint::Length(14), Constraint::Length(14)]);
    frame.render_widget(t, chunk);
}

#[derive(Default)]
struct FrameBuffer<'a> {
    windows: Vec<&'a Window>,
    block: Option<Block<'a>>,
}

impl<'a> FrameBuffer<'a> {
    /*fn new(windows: Vec<&'a Window>) -> Self {
        Self { windows, ..Default::default() }
    }

    fn block(mut self, block: Block<'a>) -> FrameBuffer<'a> {
        self.block = Some(block);
        self
    }*/
}

impl tui::widgets::Widget for FrameBuffer<'_> {
    fn render(mut self, area: Rect, buf: &mut tui::buffer::Buffer) {
        let area = match self.block.take() {
            Some(b) => {
                let inner_area = b.inner(area);
                b.render(area, buf);
                inner_area
            }
            None => area,
        };

        let windows_num = self.windows.len();
        let window_height = area.height / windows_num as u16;
        let y_start = area.y;
        for (idx, window) in self.windows.iter().enumerate() {
            let area =
                Rect::new(area.x, y_start + window_height * idx as u16, area.width, window_height);

            let mut resizer = resize::new(
                window.width / 2,
                window.height,
                area.width as usize,
                area.height as usize,
                RGB8,
                Lanczos3,
            )
            .unwrap();
            let mut dst = vec![RGB::new(0, 0, 0); (area.width * area.height) as usize];
            resizer.resize(&window.data, &mut dst).unwrap();

            let mut dst = dst.iter();
            for j in area.y..area.y + area.height {
                for i in area.x..area.x + area.width {
                    let rgb = dst.next().unwrap();
                    let r = rgb.r;
                    let g = rgb.g;
                    let b = rgb.b;
                    buf.get_mut(i, j).set_bg(Color::Rgb(r, g, b));
                }
            }
        }
    }
}
