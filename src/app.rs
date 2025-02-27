use std::fs;
use std::{thread, time};
use termion::terminal_size;

use crate::config::Config;
use crate::interface::Interface;
use crate::position::Position;

use crate::views::command_view::CommandView;
use crate::views::text_view::TextView;
use crate::views::View;

#[derive(Debug, Copy, Clone)]
pub enum ApplicationCommand {
    Quit(bool),
    FocusText,
    FocusCommand,
}

pub struct App {
    txt_view: TextView,
    txt_pos: Position,
    txt_sz: Position,
    cmd_view: CommandView,
    cmd_pos: Position,
    cmd_sz: Position,
    interface: Interface,
    running: bool,
    txt_focus: bool,
}

impl App {
    pub fn new(cfg: Config) -> Self {
        let text_view = match cfg.fname {
            Some(fname) => TextView::new(&fs::read_to_string(fname).unwrap()),
            None => TextView::new(""),
        };

        let win_sz = match terminal_size() {
            Ok(res) => res,
            Err(_) => panic!(),
        };

        Self {
            txt_view: text_view,
            txt_pos: Position { row: 0, col: 0 },
            txt_sz: Position {
                row: win_sz.1 - 1,
                col: win_sz.0,
            },
            cmd_view: CommandView::new(),
            cmd_pos: Position {
                row: win_sz.1 - 1,
                col: 0,
            },
            cmd_sz: Position {
                row: 1,
                col: win_sz.0,
            },
            interface: Interface::new(),
            running: true,
            txt_focus: true,
        }
    }

    pub fn exec(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.init_screen()?;
        while self.running {
            self.cmd_view.add_keystrokes(self.interface.get_keys());
            while let Some(cmd) = self.cmd_view.get_app_command() {
                self.process_command(cmd);
            }
            while let Some(cmd) = self.cmd_view.get_text_command() {
                self.txt_view
                    .process_command(cmd);
            }
        

            if self.txt_focus {
                self.interface.set_cursor(self.txt_pos, &self.txt_view);
            } else {
                self.interface.set_cursor(self.cmd_pos, &self.cmd_view);
            }
            self.interface.draw(self.txt_pos, &self.txt_view)?;
            self.interface.draw(self.cmd_pos, &self.cmd_view)?;

            thread::sleep(time::Duration::from_millis(30));
        }

        Ok(())
    }

    fn init_screen(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.txt_view.set_size(self.txt_sz);
        self.cmd_view.set_size(self.cmd_sz);
            
        self.interface.set_cursor(self.txt_pos, &self.txt_view);
        self.interface.draw(self.txt_pos, &self.txt_view)?;
        self.interface.draw(self.cmd_pos, &self.cmd_view)?;

        Ok(())
    }

    fn process_command(&mut self, cmd: ApplicationCommand) {
        match cmd {
            ApplicationCommand::Quit(true) => self.running = false, // Force quit
            ApplicationCommand::Quit(false) => self.running = false, // Quit if saved
            ApplicationCommand::FocusText => self.txt_focus = true,
            ApplicationCommand::FocusCommand => self.txt_focus = false,
        }
    }
}
