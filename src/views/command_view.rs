use termion::event::Key;

use std::collections::VecDeque;

use crate::interface::CursorStyle;
use crate::position::Position;
use crate::views::View;

use crate::app::ApplicationCommand;
use crate::views::text_view::TextCommand;

enum CommandViewModes {
    NormalMode,
    InsertMode,
    CommandLineMode,
}

pub struct CommandView {
    state: CommandViewModes,
    cmd: Vec<Key>,
    txt_cmds: VecDeque::<TextCommand>,
    app_cmds: VecDeque::<ApplicationCommand>,
    cursor: Position,
    sz: Position,
    view: Vec<char>,
    updates: Vec<bool>,
}

impl CommandView {
    pub fn new() -> Self {
        Self {
            state: CommandViewModes::NormalMode,
            cmd: Vec::<Key>::new(),
            txt_cmds: VecDeque::<TextCommand>::new(),
            app_cmds: VecDeque::<ApplicationCommand>::new(),
            cursor: Position { row: 0, col: 0 },
            sz: Position { row: 0, col: 0 },
            view: Vec::<char>::new(),
            updates: Vec::<bool>::new(),
        }
    }

    pub fn add_keystrokes(&mut self, mut keys: Vec<Key>) {
        self.cmd.append(&mut keys);
        self.parse_commands();
    }

    pub fn has_text_command(&mut self) -> bool {
        !self.txt_cmds.is_empty()
    }

    pub fn get_text_command(&mut self) -> Option<TextCommand> {
        self.txt_cmds.pop_back()
    }

    pub fn has_app_command(&mut self) -> bool {
        !self.txt_cmds.is_empty()
    }

    pub fn get_app_command(&mut self) -> Option<ApplicationCommand> {
        self.app_cmds.pop_back()
    }

    fn refresh_view(&mut self) {
        match self.state {
            CommandViewModes::NormalMode => {
                let placeholder = "-- Normal --";
                for (i, c) in placeholder.chars().enumerate() {
                    if i < (self.sz.col as usize) {
                        self.view[i] = c;
                    }
                }
            },
            CommandViewModes::InsertMode => {
                let placeholder = "-- Insert --";
                for (i, c) in placeholder.chars().enumerate() {
                    if i < (self.sz.col as usize) {
                        self.view[i] = c;
                    }
                }
            }
            CommandViewModes::CommandLineMode => {
                for c in 0..(self.sz.col as usize) {
                    if self.view[c] != ' ' {
                        self.view[c] = ' ';
                    }
                }
                self.view[0] = ':';
            }
        }
    }

    fn parse_app_command(&self, s: &str) -> Result<ApplicationCommand, &'static str> {
        if s == "q" {
            return Ok(ApplicationCommand::Quit(false));
        } else if s == "q!" {
            return Ok(ApplicationCommand::Quit(true));
        }

        Err("Unknown command")
    }

    fn parse_txt_command(&mut self) {
        while !self.cmd.is_empty() {
            match self.cmd[0] {
                Key::Char(':') => {
                    return;
                },
                Key::Char('h') => {
                    self.txt_cmds.push_front(TextCommand::CursorLeft(1));
                    self.cmd.drain(0..1);
                },
                Key::Char('j') => {
                    self.txt_cmds.push_front(TextCommand::CursorDown(1));
                    self.cmd.drain(0..1);
                },
                Key::Char('k') => {
                    self.txt_cmds.push_front(TextCommand::CursorUp(1));
                    self.cmd.drain(0..1);
                },
                Key::Char('l') => {
                    self.txt_cmds.push_front(TextCommand::CursorRight(1));
                    self.cmd.drain(0..1);
                },
                Key::Char('i') => {
                    self.txt_cmds.push_front(TextCommand::SetCursorStyle(CursorStyle::Bar));
                    self.state = CommandViewModes::InsertMode;
                    self.refresh_view();
                    self.cmd.drain(0..1);
                    return;
                }
                Key::Char('a') => {
                    self.txt_cmds.push_front(TextCommand::SetCursorStyle(CursorStyle::Bar));
                    self.txt_cmds.push_front(TextCommand::CursorRight(1));
                    self.state = CommandViewModes::InsertMode;
                    self.refresh_view();
                    self.cmd.drain(0..1);
                    return;
                }
                Key::Char('x') => {
                    self.txt_cmds.push_front(TextCommand::Delete);
                    self.cmd.drain(0..1);
                }
                k => {
                    eprintln!("Unhandled input in normal mode: {:?}", k);
                    self.cmd.drain(0..1);
                }
            }
        }
    }

    fn parse_commands(&mut self) {
        if !self.cmd.is_empty() {
            match self.state {
                CommandViewModes::NormalMode => {
                    match self.cmd[0] {
                        Key::Char(':') => {
                            self.app_cmds.push_front(ApplicationCommand::FocusCommand);
                            self.state = CommandViewModes::CommandLineMode;
                            self.refresh_view();

                            self.cursor.col = 1;
                            self.cmd.drain(0..1);

                            for i in 0..self.cmd.len() {
                                if i+1 < (self.sz.col as usize) {
                                    if let Key::Char(c) = self.cmd[i] {
                                        self.view[i+1] = c;
                                        self.updates[i+1] = true;
                                        self.cursor.col += 1;
                                    }
                                }
                            }
                        }
                        Key::Char(_) => {
                            self.parse_txt_command();
                        }
                        _ => {
                            self.cmd.drain(0..1);
                        },
                    }
                },
                CommandViewModes::InsertMode => {
                    while !self.cmd.is_empty() {
                        match self.cmd[0] {
                            Key::Esc => {
                                self.cmd.drain(0..1);
                                self.txt_cmds.push_front(TextCommand::SetCursorStyle(CursorStyle::Block));
                                self.txt_cmds.push_front(TextCommand::CursorLeft(1));
                                self.state = CommandViewModes::NormalMode;
                                self.refresh_view();
                                break;
                            },
                            Key::Char(c) => {
                                self.txt_cmds.push_front(TextCommand::Insert(c));
                                self.cmd.drain(0..1);
                            },
                            Key::Backspace => {
                                self.txt_cmds.push_front(TextCommand::Delete);
                                self.cmd.drain(0..1);
                            }
                            k => {
                                eprintln!("Unhandled input in insert mode: {:?}", k);
                                self.cmd.drain(0..1);
                            },
                        }
                    }
                },
                CommandViewModes::CommandLineMode => {
                    let mut i = 0;
                    while i < self.cmd.len() {
                        match self.cmd[i] {
                            Key::Char('\n') => {
                                for j in 0..=i {
                                    self.view[j] = ' ';
                                    self.updates[j] = true;
                                }
                                self.cursor.col = 0;

                                let s: String = self.cmd.drain(..=i).filter_map(|k| match k {
                                    Key::Char('\n') => None,
                                    Key::Char(c) => Some(c),
                                    _ => None,
                                }).collect();

                                match self.parse_app_command(&s) {
                                    Ok(cmd) => self.app_cmds.push_front(cmd),
                                    Err(_e) => (),
                                }
                                self.app_cmds.push_front(ApplicationCommand::FocusText);
                                self.state = CommandViewModes::NormalMode;
                                self.refresh_view();
                            }
                            Key::Char(c) => {
                                if i+1 < (self.sz.col as usize) && self.view[i+1] != c {
                                    self.view[i+1] = c;
                                    self.updates[i+1] = true;
                                    self.cursor.col += 1;
                                }
                                i += 1;
                            },
                            Key::Backspace => {
                                if i == 0 {
                                    self.cmd.drain(0..1);
                                    self.view[i] = ' ';
                                    self.updates[i] = true;
                                
                                    self.app_cmds.push_front(ApplicationCommand::FocusText);
                                    self.state = CommandViewModes::NormalMode;
                                    self.refresh_view();
                                    break;
                                } else {
                                    self.cmd.drain(i-1..=i);
                                    self.view[i] = ' ';
                                    self.updates[i] = true;
                                    self.cursor.col -= 1;
                                } 
                            }
                            Key::Esc => {
                                for j in 0..=i {
                                    self.view[j] = ' ';
                                    self.updates[j] = true;
                                }
                                self.cursor.col = 0;

                                self.cmd.drain(..=i);
                                self.app_cmds.push_front(ApplicationCommand::FocusText);
                                self.state = CommandViewModes::NormalMode;
                                self.refresh_view();
                                break;
                            }
                            _ => (),
                        }
                    }
                },
            }
        }
    }
}

impl Default for CommandView {
    fn default() -> Self {
        Self::new()
    }
}

impl View for CommandView {
    fn set_size(&mut self, p: Position) {
        self.sz = p;

        self.view = vec![' '; (self.sz.row * self.sz.col) as usize];
        self.updates = vec![false; (self.sz.row * self.sz.col) as usize];
        self.refresh_view();
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
        CursorStyle::Block
    }
}
