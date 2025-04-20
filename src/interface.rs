use std::io::{stdout, Stdout, Write};
use std::sync::mpsc;
use std::thread;

use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::screen::{AlternateScreen, IntoAlternateScreen};
//use termion::style;

use crate::position::Position;
use crate::views::View;

pub struct Interface {
    stdout: AlternateScreen<RawTerminal<Stdout>>,
    io_rx: mpsc::Receiver<termion::event::Key>,
    cursor: Position,
    cursor_style: CursorStyle,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum CursorStyle {
    Block,
    Bar,
    Underline,
}


impl Interface {
    pub fn new() -> Self {
        let mut stdout = stdout()
            .into_raw_mode()
            .unwrap()
            .into_alternate_screen()
            .unwrap();
        write!(stdout, "{}", termion::clear::All).unwrap();
        let _ = stdout.flush();

        let mut stdin = termion::async_stdin().keys();
        let (io_tx, io_rx) = mpsc::channel::<termion::event::Key>();

        thread::spawn(move || {
            loop {
                if let Some(Ok(key)) = stdin.next() {
                    io_tx.send(key).unwrap()
                }
            }
        });

        Self { stdout, io_rx, cursor: Position{row: 0, col: 0}, cursor_style: CursorStyle::Block}
    }

    pub fn set_cursor(&mut self, pos: Position, view: &impl View) {
        let cursor = view.get_cursor_pos();
        self.cursor.row = pos.row + cursor.row;
        self.cursor.col = pos.col + cursor.col;
        self.cursor_style = view.get_cursor_style();
    }

    pub fn draw(
        &mut self,
        pos: Position,
        view: &impl View,
    ) -> Result<(), Box<dyn std::error::Error>> {
        write!(self.stdout, "{}", termion::cursor::Hide)?;
        let sz = view.get_size();
        let text = view.get_view();

        for r in 0..sz.row {
            write!(self.stdout, "{}", termion::cursor::Goto(pos.col + 1, pos.row + r + 1))?;
            write!(self.stdout, "{}", text[((r * sz.col) as usize)..((r * sz.col + sz.col -1) as usize)].iter().collect::<String>())?;
/*            for c in 0..sz.col {
                write!(
                    self.stdout,
                    "{}",
                    text[(r * sz.col + c) as usize]
                )?;
            }*/
        }
        write!(self.stdout, "{}{}", termion::cursor::Goto(self.cursor.col + 1, self.cursor.row + 1), termion::cursor::Show)?;
        match self.cursor_style {
            CursorStyle::Block => write!(self.stdout, "{}", termion::cursor::BlinkingBlock)?,
            CursorStyle::Bar => write!(self.stdout, "{}", termion::cursor::BlinkingBar)?,
            CursorStyle::Underline => write!(self.stdout, "{}", termion::cursor::BlinkingUnderline)?,
        };
        self.stdout.flush()?;

        Ok(())
    }

    pub fn get_keys(&mut self) -> Vec<termion::event::Key> {
        self.io_rx.try_iter().collect()
    }
}

impl Drop for Interface {
    fn drop(&mut self) {}
}
