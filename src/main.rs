use crossterm::event::{self, Event, KeyCode, MouseEventKind, MouseButton};
use tui::{
    backend::{Backend, CrosstermBackend},
    widgets::{Block, Borders, Paragraph},
    layout::{Layout, Constraint, Direction},
    style::{Style, Modifier, Color},
    text::{Span},
    Terminal,
};
use std::io;

struct App {
    inputs: Vec<String>,    // Wektory na wartości inputów
    active_input: usize,    // Indeks aktywnego inputa
}

impl App {
    fn new() -> App {
        App {
            inputs: vec![String::new(); 4], // 4 puste pola
            active_input: 0,
        }
    }
}

fn main() -> Result<(), io::Error> {
    // Inicjalizacja terminala
    let mut stdout = io::stdout();
    crossterm::terminal::enable_raw_mode()?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();

    loop {
        terminal.draw(|f| ui(f, &app))?;

        // Obsługa zdarzeń
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char(c) => {
                    // Dodawanie znaków do aktywnego inputa
                    if let Some(input) = app.inputs.get_mut(app.active_input) {
                        input.push(c);
                    }
                }
                KeyCode::Backspace => {
                    // Usuwanie znaków z aktywnego inputa
                    if let Some(input) = app.inputs.get_mut(app.active_input) {
                        input.pop();
                    }
                }
                KeyCode::Tab => {
                    // Przełączanie między inputami
                    app.active_input = (app.active_input + 1) % app.inputs.len();
                }
                KeyCode::Esc => {
                    // Wyjście z aplikacji
                    break;
                }
                _ => {}
            }
        } else if let Event::Mouse(mouse) = event::read()? {
            if let Event::Mouse(mouse) = event::read()? {
                if mouse.kind == MouseEventKind::Down(MouseButton::Left) {
                    let (x, y) = (mouse.column, mouse.row); // Pobieranie współrzędnych kliknięcia
                    // Sprawdź, który input został kliknięty
                    if y >= 2 && y <= 3 { // Input 1
                        app.active_input = 0;
                    } else if y >= 5 && y <= 6 { // Input 2
                        app.active_input = 1;
                    } else if y >= 8 && y <= 9 { // Input 3
                        app.active_input = 2;
                    } else if y >= 11 && y <= 12 { // Input 4
                        app.active_input = 3;
                    }
                }
            }

        }
    }

    // Czyszczenie terminala
    crossterm::terminal::disable_raw_mode()?;
    Ok(())
}

fn ui<B: Backend>(f: &mut tui::Frame<B>, app: &App) {
    // Ustawienie layoutu
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(3), // Input 1
            Constraint::Length(2),
            Constraint::Length(3), // Input 2
            Constraint::Length(2),
            Constraint::Length(3), // Input 3
            Constraint::Length(2),
            Constraint::Length(3), // Input 4
            Constraint::Min(0),
        ].as_ref())
        .split(f.size());

    for (i, input) in app.inputs.iter().enumerate() {
        // Renderowanie każdego inputa
        let block_title = if app.active_input == i {
            Span::styled(
                format!("Input {}", i + 1),
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
            )
        } else {
            Span::raw(format!("Input {}", i + 1))
        };

        let input_paragraph = Paragraph::new(input.as_ref())
            .block(Block::default().borders(Borders::ALL).title(block_title));
        f.render_widget(input_paragraph, chunks[i * 2]);
    }
}
