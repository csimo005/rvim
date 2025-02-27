use termion::event::Key;

use std::collections::VecDeque;

use crate::position::Position;
use crate::views::View;

use crate::app::ApplicationCommand;
use crate::views::text_view::TextCommand;

enum CommandViewState {
    TxtCommand,
    AppCommand,
}

pub struct CommandView {
    state: CommandViewState,
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
            state: CommandViewState::TxtCommand,
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
    }

    fn parse_app_command(&self, s: &str) -> Result<ApplicationCommand, &'static str> {
        if s == "q" {
            return Ok(ApplicationCommand::Quit(false));
        } else if s == "q!" {
            return Ok(ApplicationCommand::Quit(true));
        }

        Err("Unknown command")
    }

    fn parse_commands(&mut self) {
        if !self.cmd.is_empty() {
            match self.state {
                CommandViewState::TxtCommand => {
                    match self.cmd[0] {
                        Key::Char(':') => {
                            self.app_cmds.push_front(ApplicationCommand::FocusCommand);
                            self.state = CommandViewState::AppCommand;
                            for c in 0..(self.sz.col as usize) {
                                if self.view[c] != ' ' {
                                    self.view[c] = ' ';
                                    self.updates[c] = true;
                                }
                            }

                            self.view[0] = ':';
                            self.updates[0] = true;
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
                        Key::Char('g') => {
                            if self.cmd.len() > 1 && self.cmd[1] == Key::Char('g') {
                                self.txt_cmds.push_front(TextCommand::JumpTop);
                                self.cmd.drain(0..2);
                            }
                        }
                        Key::Char('G') => {
                            self.txt_cmds.push_front(TextCommand::JumpBottom);
                            self.cmd.drain(0..1);
                        }
                        Key::Char(_) => {
                            let mut i = 0;
                            while i < self.cmd.len() {
                                if self.cmd[i] == Key::Esc {
                                    self.cmd.drain(..=i);
                                    i = 0;
                                }
                                i += 1;
                            }
                            
                        }
                        _ => (),
                    }
                },
                CommandViewState::AppCommand => {
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
                                self.state = CommandViewState::TxtCommand;
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
                                    self.state = CommandViewState::TxtCommand;
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
                                self.state = CommandViewState::TxtCommand;
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
}
