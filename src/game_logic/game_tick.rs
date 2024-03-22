use crossterm::{
    cursor,
    style::{self, Stylize},
    terminal, ExecutableCommand, QueueableCommand,
};
use std::thread;
use std::{
    io::{self, Write},
    time::Duration,
};

pub fn tick() -> io::Result<()> {
    let mut stdout = io::stdout();
    loop {
        stdout.execute(terminal::Clear(terminal::ClearType::All))?;

        for y in 0..30 {
            for x in 0..50 {
                if (y == 0 || y == 30 - 1) || (x == 0 || x == 50 - 1) {
                    // in this loop we are more efficient by not flushing the buffer.
                    stdout
                        .queue(cursor::MoveTo(x, y))?
                        .queue(style::PrintStyledContent("█".cyan()))?;
                }
            }
        }
        stdout.flush()?;
        thread::sleep(Duration::from_millis(1000));
    }
}
