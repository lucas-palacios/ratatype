use std::io::{stdout, Error};

mod phrase;
use phrase::Phrase;

use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{layout::Flex, prelude::*, style::Style, widgets::*, Frame, Terminal};

fn main() -> Result<(), Error> {
    enable_raw_mode()?; // no line buffering
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    let mut should_quit = false;
    let mut is_correct: Option<bool>;

    let mut phrase = Phrase::new();

    while !should_quit {
        terminal.draw(|f| ui(f, &phrase))?;
        (is_correct, should_quit) = handle_event(&phrase).unwrap();
        phrase.update(is_correct);
    }
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;

    Ok(())
}

// returns following tuple:
// 1 - Option<bool>: None if didn't pressed a key.
//                   Some(true) if pressed correctly.
//                   Some(false) if pressed wrong key.
// 2 - bool: true if wants to exit
pub fn handle_event(phrase: &Phrase) -> Result<(Option<bool>, bool), Error> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(k) = event::read()? {
            if k.kind == event::KeyEventKind::Press {
                let chars: Vec<char> = phrase.queue.0.chars().collect();
                let cur_char: char = *chars.get(phrase.char_ptr).unwrap();
                match k.code {
                    KeyCode::Esc => {
                        return Ok((Some(false), true));
                    }
                    KeyCode::Char(c) => {
                        if c == cur_char {
                            return Ok((Some(true), false));
                        }
                        return Ok((Some(false), false));
                    }
                    _ => {
                        return Ok((Some(false), false));
                    }
                }
            }
        }
    }
    return Ok((None, false));
}

pub fn ui(frame: &mut Frame, phrase: &Phrase) {
    let main_layout = Layout::new(
        Direction::Vertical,
        [Constraint::Length(1), Constraint::Percentage(10)],
    )
    .flex(Flex::Center)
    .split(frame.size());

    frame.render_widget(
        Block::new()
            .borders(Borders::TOP)
            .title("ratatype".black().on_white().to_centered_line())
            .bold()
            .white(),
        main_layout[0],
    );

    let inner_layout = Layout::new(
        Direction::Vertical,
        [Constraint::Length(1), Constraint::Length(1)],
    )
    .flex(Flex::Center)
    .split(main_layout[1]);

    // current line being typed
    frame.render_widget(
        Paragraph::new(colored_text(phrase)).alignment(Alignment::Center),
        inner_layout[0],
    );

    frame.render_widget(
        Paragraph::new(Span::styled(
            phrase.queue.1.as_str(),
            Style::new().fg(Color::Gray),
        ))
        .alignment(Alignment::Center),
        inner_layout[1],
    );
}

pub fn colored_text(phrase: &Phrase) -> Text {
    let mut colored_chars: Vec<Span> = Vec::new();
    let phrase_chars: Vec<char> = phrase.queue.0.chars().collect();
    for i in 0..phrase.queue.0.len() {
        let phrase_char = phrase_chars.get(i).unwrap();
        if i < phrase.colors.len() {
            let correct = phrase.colors.get(i).unwrap();
            colored_chars.push(Span::styled(
                phrase_char.to_string().clone(),
                Style::new().fg(if *correct {
                    Color::LightGreen
                } else {
                    Color::Red
                }),
            ));
        } else {
            colored_chars.push(Span::styled(
                phrase_char.to_string().clone(),
                Style::new().fg(Color::Gray),
            ));
        }
    }
    let line = Line::from(colored_chars);
    Text::from(vec![line])
}
