use std::io;
pub mod tui;
use crate::tui::TUI;

fn main() -> io::Result<()> {
    eprintln!("pid={}", std::process::id());
    std::thread::sleep(std::time::Duration::from_secs(20));
    let mut tui = TUI::new();
    while tui.run(500)? {}
    Ok(())
}
