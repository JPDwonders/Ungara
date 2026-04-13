use std::io;
use crossterm::{terminal, execute};

pub fn restore() -> io::Result<()> {
    terminal::disable_raw_mode()?;
    execute!(io::stdout(), terminal::LeaveAlternateScreen)?;
    Ok(())
}

pub fn init() -> io::Result<()> {
    // Enabling raw mode allows for more  
    // direct control over input and output. 
    terminal::enable_raw_mode()?;

    // Switches to an alternate screen buffer, 
    // which allows the application to draw its UI
    // without affecting the main terminal screen.
    execute!(io::stdout(), terminal::EnterAlternateScreen)?;

    // Installs a wrapper around the existing panic hook 
    // that first resets the terminal to a clean state, 
    // then delegates to the original hook.
    std::panic::set_hook(std::boxed::Box::new(move |info| {
        let _ = restore(); 
        std::panic::take_hook()(info);
    }));

    Ok(())
}

