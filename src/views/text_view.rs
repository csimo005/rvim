use crate::interface::CursorStyle;
use crate::piece_table::PieceTable;
use crate::position::Position;
use crate::views::View;

#[derive(Debug, Copy, Clone)]
pub enum TextCommand {
    CursorUp(u16), // For cursor up, down, left, right u16 defines how many spaces to move in that
    CursorDown(u16), // direction
    CursorLeft(u16),
    CursorRight(u16),
    JumpTop(u16),
    JumpBottom(u16),
    SetCursorStyle(CursorStyle),
    Insert(char),
    Delete,
}

pub struct TextView {
    offset: Position,
    text: PieceTable,
    cursor: Position,
    sz: Position,
    view: Vec<char>,
    updates: Vec<bool>,
    cursor_style: CursorStyle,
}

impl TextView {
    pub fn new(text: &str) -> Self {
        Self {
            cursor: Position { row: 0, col: 5 },
            offset: Position { row: 0, col: 0 },
            text: PieceTable::new(text),
            sz: Position { row: 0, col: 0 },
            view: Vec::<char>::new(),
            updates: Vec::<bool>::new(),
            cursor_style: CursorStyle::Block,
        }
    }

    pub fn refresh_text(&mut self) {
        for i in 0..self.view.len() {
            self.view[i] = ' ';
        }

        for r in 0..self.sz.row {
            let ln = self.offset.row + r + 1;
            if ln < 1000 {
                self.view[(r * self.sz.col) as usize] = ' ';
            } else {
                self.view[(r * self.sz.col) as usize] =
                    char::from_digit(((ln / 1000) % 10) as u32, 10).unwrap();
            }

            if ln < 100 {
                self.view[(r * self.sz.col + 1) as usize] = ' ';
            } else {
                self.view[(r * self.sz.col + 1) as usize] =
                    char::from_digit(((ln / 100) % 10) as u32, 10).unwrap();
            }

            if ln < 10 {
                self.view[(r * self.sz.col + 2) as usize] = ' ';
            } else {
                self.view[(r * self.sz.col + 2) as usize] =
                    char::from_digit(((ln / 10) % 10) as u32, 10).unwrap();
            }

            self.view[(r * self.sz.col + 3) as usize] =
                char::from_digit((ln % 10) as u32, 10).unwrap();
            self.view[(r * self.sz.col + 4) as usize] = ' ';

            if let Some(line) = self.text.get_line((self.offset.row + r) as usize) {
                for c in 5..self.sz.col {
                    if ((self.offset.col + c - 5) as usize) < line.len() {
                        self.view[(r * self.sz.col + c) as usize] = line[(self.offset.col + c - 5) as usize];
                    } else {
                        break;
                    }
                }
            }
        }

        for i in 0..self.updates.len() {
            self.updates[i] = true;
        }
    }

    pub fn process_command(&mut self, cmd: TextCommand) {
        match cmd {
            TextCommand::CursorUp(y) => {
                if y <= self.cursor.row {
                    self.cursor.row -= y;
                } else {
                    if (y - self.cursor.row) <= self.offset.row {
                        self.offset.row -= y - self.cursor.row;
                    } else {
                        self.offset.row = 0;
                    }
                    self.cursor.row = 0;
                    self.refresh_text();
                }
                
                if self.cursor.col >= 5 + self.text.get_line_length(self.cursor.row as usize).unwrap() as u16 {
                    if self.text.get_line_length((self.cursor.row + self.offset.row) as usize).unwrap() > 0 {
                        self.cursor.col = 4 + self.text.get_line_length((self.cursor.row + self.offset.row) as usize).unwrap() as u16;
                    } else {
                        self.cursor.col = 5;
                    }
                }
            }
            TextCommand::CursorDown(y) => {
                if ((self.cursor.row + self.offset.row + 1) as usize) < self.text.lines() {
                    if self.cursor.row + y < self.sz.row {
                        self.cursor.row += y;
                    } else {
                        self.offset.row += 1;
                        if ((self.offset.row + self.cursor.row + 1) as usize) >= self.text.lines() {
                            self.offset.row = (self.text.lines() - (self.sz.row as usize) - 1) as u16;
                        }
                        self.cursor.row = self.sz.row - 1;
                        self.refresh_text();
                    }
                    
                    if self.cursor.col >= 5 + self.text.get_line_length((self.cursor.row + self.offset.row) as usize).unwrap() as u16 {
                        if self.text.get_line_length((self.cursor.row + self.offset.row) as usize).unwrap() > 0 {
                            self.cursor.col = 4 + self.text.get_line_length((self.cursor.row + self.offset.row) as usize).unwrap() as u16;
                        } else {
                            self.cursor.col = 5;
                        }
                    }
                }
            }
            TextCommand::CursorLeft(x) => {
                if x + 5 <= self.cursor.col {
                    self.cursor.col -= x;
                } else {
                    self.cursor.col = 5;
                }
            }
            TextCommand::CursorRight(x) => {
                self.cursor.col += x;
                if self.cursor.col >= self.sz.col {
                    self.offset.col += self.cursor.col - self.sz.col + 1;
                    self.cursor.col = self.sz.col - 1;
                }

                let line_length = self.text.get_line_length((self.cursor.row + self.offset.row) as usize).unwrap() as u16;
                if self.cursor_style == CursorStyle::Block && (self.offset.col + self.cursor.col - 5) >= line_length {
                    if self.cursor.col + 1 == self.sz.col {
                        self.offset.col = line_length - self.cursor.col - 1;
                    } else {
                        self.cursor.col = 5 + line_length - 1;
                    }
                } else if self.cursor_style == CursorStyle::Bar && (self.offset.col + self.cursor.col - 5) > line_length {
                    if self.cursor.col + 1 == self.sz.col {
                        self.offset.col = line_length - self.cursor.col;
                    } else {
                        self.cursor.col = 5 + line_length;
                    }
                }
            }
            TextCommand::JumpTop(_reps) => {
                self.cursor.row = 0;
                self.offset.row = 0;
                
                if self.cursor.col >= 5 + self.text.get_line_length((self.cursor.row + self.offset.row) as usize).unwrap() as u16 {
                    if self.text.get_line_length((self.cursor.row + self.offset.row) as usize).unwrap() > 0 {
                        self.cursor.col = 4 + self.text.get_line_length((self.cursor.row + self.offset.row) as usize).unwrap() as u16;
                    } else {
                        self.cursor.col = 5;
                    }
                }
                self.refresh_text();
            },
            TextCommand::JumpBottom(_reps) => {
                if self.text.lines() > (self.sz.row as usize) {
                    self.offset.row = (self.text.lines() - (self.sz.row as usize) - 1) as u16;
                    self.cursor.row = self.sz.row - 1;
                } else {
                    self.cursor.row = self.text.lines() as u16;
                }
                
                if self.cursor.col >= 5 + self.text.get_line_length((self.cursor.row + self.offset.row) as usize).unwrap() as u16 {
                    if self.text.get_line_length((self.cursor.row + self.offset.row) as usize).unwrap() > 0 {
                        self.cursor.col = 4 + self.text.get_line_length((self.cursor.row + self.offset.row) as usize).unwrap() as u16;
                    } else {
                        self.cursor.col = 5;
                    }
                }
                self.refresh_text();
                
            },
            TextCommand::SetCursorStyle(sty) => {
                self.cursor_style = sty;
            }
            TextCommand::Insert(c) => {
                if let Some(line_offset) = self.text.get_line_offset((self.offset.row + self.cursor.row) as usize) {
                    let idx = line_offset + (self.offset.col + self.cursor.col - 5) as usize;
                    self.text.insert(idx, c);

                    if c == '\n' {
                        self.cursor.row += 1;
                        self.cursor.col = 5;
                    } else {
                        self.cursor.col += 1;
                    }

                    self.refresh_text();
                }
            },
            TextCommand::Delete => {
                if let Some(line_offset) = self.text.get_line_offset((self.offset.row + self.cursor.row) as usize) {
                    let idx = line_offset + (self.offset.col + self.cursor.col - 5) as usize;
                    
                    match self.cursor_style {
                        CursorStyle::Underline => panic!("Recieved TextCommand::Delete while cursor style is underscored"),
                        CursorStyle::Block => self.text.delete(idx),
                        CursorStyle::Bar => {
                            if idx > 0 {
                                if self.cursor.col == 5 && self.offset.col == 0 {
                                    let prev_line_length = self.text.get_line_length((self.cursor.row + self.offset.row - 1) as usize).unwrap() as u16;
                                    self.text.delete(idx - 1);

                                    if self.cursor.row == 0 {
                                        self.offset.row -= 1;
                                    } else {
                                        self.cursor.row -= 1; 
                                    }

                                    if prev_line_length + 5 < self.sz.col {
                                        self.cursor.col = prev_line_length + 5;
                                        self.offset.col = 0
                                    } else {
                                        self.cursor.col = self.sz.col - 1;
                                        self.offset.col = prev_line_length - self.sz.col + 4;
                                    }
                                } else {
                                    self.text.delete(idx - 1);
                                    self.cursor.col -= 1;
                                    if self.cursor.col < 5 {
                                        self.cursor.col = 5;
                                        self.offset.col -= 1;
                                    }
                                }
                            }
                        },
                    };
                    self.refresh_text();
                }

            }
        }
    }
}

impl Default for TextView {
    fn default() -> Self {
        Self::new("")
    }
}

impl View for TextView {
    fn set_size(&mut self, p: Position) {
        self.sz = p;

        self.view = vec![' '; (self.sz.row * self.sz.col) as usize];
        self.updates = vec![false; (self.sz.row * self.sz.col) as usize];
        self.refresh_text();
    }

    fn get_size(&self) -> Position {
        self.sz
    }

    fn get_view(&self) -> Vec<char> {
        self.view.clone()
    }

    fn get_updates(&mut self) -> Vec<bool> {
        let updates = self.updates.clone();

        for i in 0..self.updates.len() {
            self.updates[i] = false;
        }

        updates
    }

    fn get_cursor_pos(&self) -> Position {
        self.cursor
    }

    fn get_cursor_style(&self) -> CursorStyle {
        self.cursor_style
    }
}
