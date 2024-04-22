use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, HighlightSpacing, List, ListItem, Paragraph, Wrap},
    Frame,
};

use crate::{
    app::{App, CurrentScreen, CurrentlyEditing},
    APP_NAME,
};

pub fn ui(frame: &mut Frame, app: &App) {
    // Create the layout sections.
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(frame.size());

    let title_block = Block::default()
        .borders(Borders::NONE)
        .style(Style::default());

    let title = Paragraph::new(Text::styled(
        format!("{}", APP_NAME),
        Style::default().fg(Color::Green),
    ))
    .alignment(Alignment::Center)
    .block(title_block);
    
    frame.render_widget(title, chunks[0]);
    let mut list_items = Vec::<ListItem>::new();

    for key in app.pairs.keys() {
        list_items.push(ListItem::new(Line::from(Span::styled(
            format!("{: <25} : {}", key, app.pairs.get(key).unwrap()),
            Style::default().fg(Color::Yellow),
        ))));
    }

    let chunk_one_block = Block::default().borders(Borders::ALL);

    frame.render_widget(chunk_one_block, chunks[1]);

    let current_keys_hint = {
        match app.current_screen {
            CurrentScreen::Main => Span::styled(
                "<↓↑> to move, <Enter> or <→> to select and <E> or <Esc> to quit.",
                Style::default().fg(Color::Red),
            ),
            CurrentScreen::Editing => Span::styled(
                "(ESC) to cancel/(Tab) to switch boxes/enter to complete",
                Style::default().fg(Color::Red),
            ),
            CurrentScreen::Exiting => Span::styled(
                "(q) to quit / (e) to make new pair",
                Style::default().fg(Color::Red),
            ),
        }
    };

    let key_notes_footer = Paragraph::new(Line::from(current_keys_hint))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::NONE));

    frame.render_widget(key_notes_footer, chunks[2]);

    if let Some(editing) = &app.currently_editing {
        let popup_block = Block::default()
            .title("Enter a new key-value pair")
            .borders(Borders::NONE)
            .style(Style::default().bg(Color::DarkGray));

        let area = centered_rect(60, 25, frame.size());
        frame.render_widget(popup_block, area);

        let popup_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);

        let mut key_block = Block::default().title("Key").borders(Borders::ALL);
        let mut value_block = Block::default().title("Value").borders(Borders::ALL);

        let active_style = Style::default().bg(Color::LightYellow).fg(Color::Black);

        match editing {
            CurrentlyEditing::Key => key_block = key_block.style(active_style),
            CurrentlyEditing::Value => value_block = value_block.style(active_style),
        };

        let key_text = Paragraph::new(app.key_input.clone()).block(key_block);
        frame.render_widget(key_text, popup_chunks[0]);

        let value_text = Paragraph::new(app.value_input.clone()).block(value_block);
        frame.render_widget(value_text, popup_chunks[1]);
    }

    match app.current_screen {
        CurrentScreen::Main => {
            // Creamos un estado para manejar la lista y la selección
            let mut list_state = app.main_screen.menu.list_state.clone();

            // Creamos una lista de elementos
            let items = app
                .main_screen
                .menu
                .items
                .iter()
                .map(|item| item.text.clone())
                .collect::<Vec<String>>();

            let list = List::new(
                items
                    .iter()
                    .map(|item| Text::raw(item.to_string()).alignment(Alignment::Center)),
            )
            .block(Block::default().borders(Borders::NONE))
            .style(Style::default().fg(Color::White))
            .highlight_style(
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::ITALIC),
            )
            .highlight_spacing(HighlightSpacing::Always);

            let area = centered_rect(30, 25, chunks[1]);
            frame.render_stateful_widget(list, area, &mut list_state);
        }
        CurrentScreen::Exiting => {
            let popup_block = Block::default()
                .title("Y/N")
                .borders(Borders::NONE)
                .style(Style::default().bg(Color::DarkGray));

            let exit_text = Text::styled(
                "Would you like to output the buffer as json? (y/n)",
                Style::default().fg(Color::Red),
            );
            // the `trim: false` will stop the text from being cut off when over the edge of the block
            let exit_paragraph = Paragraph::new(exit_text)
                .block(popup_block)
                .wrap(Wrap { trim: false });

            let area = centered_rect(60, 25, frame.size());
            frame.render_widget(exit_paragraph, area);
        }
        _ => {}
    }
}

/// helper function to create a centered rect using up certain percentage of the available rect `r`
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    // Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    // Then cut the middle vertical piece into three width-wise pieces
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1] // Return the middle chunk
}
