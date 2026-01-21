use crate::app::{App, Pane};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
};

pub fn draw(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(1)])
        .split(f.area());

    let main_area = chunks[0];
    let status_area = chunks[1];

    // Three-pane layout
    let panes = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(25),
            Constraint::Percentage(50),
            Constraint::Percentage(25),
        ])
        .split(main_area);

    // File explorer (left pane)
    draw_file_explorer(f, app, panes[0]);

    // Markdown preview (center pane)
    draw_preview(f, app, panes[1]);

    // Backlinks (right pane)
    draw_backlinks(f, app, panes[2]);

    // Status bar
    draw_status(f, app, status_area);
}

fn draw_file_explorer(f: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let is_active = app.active_pane == Pane::Files;
    let border_style = if is_active {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default().fg(Color::DarkGray)
    };

    let items: Vec<ListItem> = app
        .files
        .iter()
        .enumerate()
        .map(|(i, path)| {
            let name = app.file_display_name(path);
            let style = if i == app.file_list_state {
                Style::default()
                    .bg(Color::DarkGray)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };
            ListItem::new(name).style(style)
        })
        .collect();

    let files_block = Block::default()
        .title(" Files ")
        .borders(Borders::ALL)
        .border_style(border_style);

    let list = List::new(items).block(files_block);

    f.render_widget(list, area);
}

fn draw_preview(f: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let is_active = app.active_pane == Pane::Preview;
    let border_style = if is_active {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default().fg(Color::DarkGray)
    };

    let title = app
        .selected_file()
        .map(|p| app.file_display_name(&p))
        .unwrap_or_else(|| "No file selected".to_string());

    let preview_block = Block::default()
        .title(format!(" {} ", title))
        .borders(Borders::ALL)
        .border_style(border_style);

    let content = if app.selected_content.is_empty() {
        "Select a file to preview its contents."
    } else {
        &app.selected_content
    };

    let paragraph = Paragraph::new(content)
        .block(preview_block)
        .wrap(Wrap { trim: false });

    f.render_widget(paragraph, area);
}

fn draw_backlinks(f: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let is_active = app.active_pane == Pane::Backlinks;
    let border_style = if is_active {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default().fg(Color::DarkGray)
    };

    let items: Vec<ListItem> = app
        .backlinks
        .iter()
        .enumerate()
        .map(|(i, title)| {
            let style = if i == app.backlink_list_state {
                Style::default()
                    .bg(Color::DarkGray)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };
            ListItem::new(title.as_str()).style(style)
        })
        .collect();

    let count = app.backlinks.len();
    let title = format!(" Backlinks ({}) ", count);

    let backlinks_block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(border_style);

    let list = List::new(items).block(backlinks_block);

    f.render_widget(list, area);
}

fn draw_status(f: &mut Frame, _app: &App, area: ratatui::layout::Rect) {
    let status = Line::from(vec![
        Span::styled(" tenki ", Style::default().fg(Color::Black).bg(Color::Cyan)),
        Span::raw(" "),
        Span::styled("q", Style::default().fg(Color::Yellow)),
        Span::raw(":quit "),
        Span::styled("e", Style::default().fg(Color::Yellow)),
        Span::raw(":edit "),
        Span::styled("Tab", Style::default().fg(Color::Yellow)),
        Span::raw(":switch pane "),
        Span::styled("j/k", Style::default().fg(Color::Yellow)),
        Span::raw(":navigate "),
        Span::styled("r", Style::default().fg(Color::Yellow)),
        Span::raw(":refresh"),
    ]);

    let paragraph = Paragraph::new(status);
    f.render_widget(paragraph, area);
}
