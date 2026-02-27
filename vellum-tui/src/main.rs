use crossterm::event::{Event, KeyCode, KeyEvent};
use crossterm::terminal::ClearType;
use crossterm::{cursor, event, execute, terminal};
use std::io::{stdout, Result, Write};
// use std::path::Path;
use std::time::Duration;
use vellum_app::actions::{Action, ActionResult};
use vellum_app::application::Application;
struct CleanUp;

impl Drop for CleanUp {
    fn drop(&mut self) {
        terminal::disable_raw_mode().expect("Unable to disable terminal raw mode.");
        ViewPort::clear_screen().expect("Error");
    }
}

struct ViewPort {
    view_size: (usize, usize),
}

impl ViewPort {
    fn new() -> Self {
        let view_size = terminal::size()
            .map(|(x, y)| (x as usize, y as usize))
            .unwrap();
        Self { view_size }
    }

    fn draw_rows(&self) {
        for i in 0..self.view_size.1 {
            print!("~");
            if i < self.view_size.1 - 1 {
                println!("\r")
            }
            stdout().flush().expect("Error");
        }
    }

    fn clear_screen() -> Result<()> {
        execute!(stdout(), terminal::Clear(ClearType::All))?;
        execute!(stdout(), cursor::MoveTo(0, 0))
    }

    fn refresh_screen(&self) -> Result<()> {
        Self::clear_screen()?;
        self.draw_rows();
        execute!(stdout(), cursor::MoveTo(0, 0))
    }
}

fn main() -> Result<()> {
    // let fp = Path::new("hello_world.txt");
    let _clean_up = CleanUp;
    let view = ViewPort::new();
    let win_size = terminal::size()
        .map(|(x, y)| (x as usize, y as usize))
        .unwrap();
    println!("TERMINAL SIZE = ({:?}, {:?})", win_size.0, win_size.1);
    let mut app = Application::new(win_size);
    // app.editor.load_document_from_file(fp);
    terminal::enable_raw_mode()?;
    loop {
        view.refresh_screen()?;
        if event::poll(Duration::from_millis(500))? {
            if let Event::Key(event) = event::read()? {
                match event {
                    KeyEvent {
                        code: KeyCode::Char('q'),
                        modifiers: event::KeyModifiers::CONTROL,
                        kind: _,
                        state: _,
                    } => break,
                    KeyEvent {
                        code: KeyCode::Up,
                        modifiers: event::KeyModifiers::NONE,
                        kind: _,
                        state: _,
                    } => {
                        if let ActionResult::CursorShifted { line, col, .. } =
                            app.execute_action(Action::ShiftCursorUp)
                        {
                            execute!(stdout(), cursor::MoveTo(line as u16, col as u16))?;
                        }
                    }
                    KeyEvent {
                        code: KeyCode::Down,
                        modifiers: event::KeyModifiers::NONE,
                        kind: _,
                        state: _,
                    } => {
                        if let ActionResult::CursorShifted { line, col, .. } =
                            app.execute_action(Action::ShiftCursorDown)
                        {
                            execute!(stdout(), cursor::MoveTo(line as u16, col as u16))?;
                        }
                    }
                    KeyEvent {
                        code: KeyCode::Left,
                        modifiers: event::KeyModifiers::NONE,
                        kind: _,
                        state: _,
                    } => {
                        if let ActionResult::CursorShifted { line, col, .. } =
                            app.execute_action(Action::ShiftCursorLeft)
                        {
                            execute!(stdout(), cursor::MoveTo(line as u16, col as u16))?;
                        }
                    }
                    KeyEvent {
                        code: KeyCode::Right,
                        modifiers: event::KeyModifiers::NONE,
                        kind: _,
                        state: _,
                    } => {
                        if let ActionResult::CursorShifted { line, col, .. } =
                            app.execute_action(Action::ShiftCursorRight)
                        {
                            execute!(stdout(), cursor::MoveTo(line as u16, col as u16))?;
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    Ok(())
    // let fp = Path::new("vellum-tui/src/hello_world.txt");
    // let doc = DocumentBuffer::from_file(fp).expect("Failed to read document.");
    // println!("Document was read:\n{}", doc.get_contents());
}
