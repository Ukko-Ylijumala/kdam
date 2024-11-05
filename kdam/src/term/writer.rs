use std::{
    fs::{File, OpenOptions},
    io::{stderr, stdout, Result, Write},
};

/// Stderr and Stdout writer.
#[derive(Debug, Clone)]
pub enum Writer {
    Stderr,
    Stdout,
    Tty,
}

#[cfg(target_os = "windows")]
const TTY_PATH: &str = "CON";

#[cfg(not(target_os = "windows"))]
const TTY_PATH: &str = "/dev/tty";

impl Writer {
    fn get_writer(&self) -> Result<Box<dyn Write>> {
        Ok(match self {
            Self::Stderr => Box::new(stderr().lock()),
            Self::Stdout => Box::new(stdout().lock()),
            Self::Tty => Box::new(OpenOptions::new().append(true).open(TTY_PATH)?),
        })
    }

    /// Print text buffer in terminal followed by a flush.
    pub fn print(&self, buf: &[u8]) -> Result<()> {
        let mut writer: Box<dyn Write> = self.get_writer()?;
        writer.write_all(buf)?;
        writer.flush()?;
        Ok(())
    }

    /// Print text buffer in terminal followed by a flush at specified position.
    ///
    /// # Note
    ///
    /// Cursor position is restored to original position after buffer is printed.
    ///
    /// # Example
    ///
    /// ```
    /// use kdam::term::Writer;
    ///
    /// Writer::Stderr.print_at(1, format!("1 + 1 = {}", 2).as_bytes()).unwrap();
    /// ```
    pub fn print_at(&self, position: u16, buf: &[u8]) -> Result<()> {
        let mut writer: Box<dyn Write> = self.get_writer()?;

        if position > 0 {
            writer.write_all("\n".repeat(position as usize).as_bytes())?;
            writer.write_all(buf)?;
            writer.write_fmt(format_args!("\x1b[{}A", position))?;
        } else {
            writer.write_all(&[b'\r'])?;
            writer.write_all(buf)?;
        }

        writer.flush()?;
        Ok(())
    }
}
