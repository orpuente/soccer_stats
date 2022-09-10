use winconsole::console;
use std::{io::{self, Write}};
use ansi_escapes;
use colored::Colorize;

#[derive(Debug)]
pub enum Command {
    Add(String, u32, u32),
    Delete(String)
}

#[derive(Clone, Debug)]
enum State {
    Q0,
    AddCommand,
    Name,
    S0,
    Goals,
    S1,
    Assists,
    DeleteCommand,
    ValidName,
    S2
}

pub struct InputFSM {
    state: State,
    buffer: Vec<char>,
    state_stack: Vec<State>
}

impl InputFSM {
    pub fn new() -> InputFSM {
        InputFSM { buffer: Vec::new(), state_stack: Vec::new(), state: State::Q0 }
    }

    fn update_prompt(&self) {
        // erase line
        print!("{}{}", ansi_escapes::EraseLine, ansi_escapes::CursorLeft);
        match self.state {
            State::Assists | State::ValidName => print!("{} ", ">".green()),
            _ => print!("{} ", ">".yellow())
        }
        let line: String = self.buffer.iter().collect();
        print!("{line}");

        print!("{}", ansi_escapes::CursorSavePosition);
        match self.state {
            State::AddCommand => print!("{}", "name goals assists".bright_yellow()),
            State::Name => print!("{}", " goals assists".bright_yellow()), 
            State::S0 => print!("{}", "goals assists".bright_yellow()),
            State::Goals => print!("{}", " assists".bright_yellow()),
            State::S1 => print!("{}", "assists".bright_yellow()),
            State::DeleteCommand => print!("{}", "name".bright_yellow()),
            _ => ()
        }
        print!("{}", ansi_escapes::CursorRestorePosition);

        io::stdout().flush().unwrap();
    }

    fn print(&mut self, c: char) {
        self.buffer.push(c);
        print!("{c}");
        io::stdout().flush().unwrap();
        self.state_stack.push(self.state.clone());
    }

    fn transition(&mut self, next_state: &State) {
        self.state = next_state.clone();
        //self.state_stack.push(next_state.clone());
    }

    fn erase(&mut self) {
        self.buffer.pop();
        self.state = self.state_stack.pop().unwrap_or_else(|| State::Q0);
        //self.state = self.state_stack.last().unwrap_or_else(|| &State::Q0).clone();
        print!(r"{0} {0}", 8u8 as char);
        io::stdout().flush().unwrap();
    }

    fn restart(&mut self) {
        while !self.buffer.is_empty() {
            self.erase();
        }
        for _ in 1..=20 {
            print!(r"{0} {0}", 8u8 as char);
        }
    }

    fn add_command(line: String) -> Option<Command> {
        let words: Vec<&str> = line.split_whitespace().collect();

        match &words[..] {
            ["add", name @ .., goals, assists] => {
                let name: String = name.join(" ");
                if let (Ok(goals), Ok(assists)) = (goals.parse::<u32>(), assists.parse::<u32>()) {
                    return Some(Command::Add(name, goals, assists));
                }
            },
            _ => ()
        };

        None
    }

    fn del_command(line: String) -> Option<Command> {
        let words: Vec<&str> = line.split_whitespace().collect();

        match &words[..] {
            ["del", name @ ..] => {
                let name: String = name.join(" ");
                return Some(Command::Delete(name));
            },
            _ => ()
        };

        None
    }

    pub fn get(&mut self) -> Option<Command> {
        self.update_prompt();
        let c = console::getch(true).unwrap();

        if c as i32 == 8 {
            self.erase();
            self.update_prompt();
            return None;
        }
        else if self.buffer.len() > crate::LINE_CAPACITY {
            return None;
        }

        match self.state {
            State::Q0 if matches!(c, 'a'..='z' | ' ') => {
                if matches!(c, 'a'..='z') {
                    self.print(c);
                }
                else if matches!(c, ' ') {
                    let command: String = self.buffer.iter().collect();
                    match command.as_str() {
                        "add" => {
                            self.transition(&State::AddCommand);
                        },
                        "del" => {
                            self.transition(&State::DeleteCommand);
                        },
                        _ => ()
                    };
                    self.print(c);
                }
            },
            State::AddCommand => {
                if matches!(c, 'a'..='z' | 'A'..='Z') {
                    self.print(c);
                    self.transition(&State::Name);
                }
            },
            State::Name => {
                match c {
                    'a'..='z' | 'A'..='Z' => {
                        self.print(c);
                    },
                    ' ' => {
                        self.print(c);
                        self.transition(&State::S0);
                    },
                    _ => ()
                }
            },
            State::S0 => {
                match c {
                    'a'..='z' | 'A'..='Z' => {
                        self.print(c);
                        self.transition(&State::Name);
                    },
                    '0'..='9' => {
                        self.print(c);
                        self.transition(&State::Goals);
                    },
                    _ => ()
                }
            },
            State::Goals => {
                match c {
                    '0'..='9' => {
                        self.print(c);
                    },
                    ' ' => {
                        self.print(c);
                        self.transition(&State::S1);
                    }
                    _ => ()
                }
            },
            State::S1 => {
                match c {
                    '0'..='9' => {
                        self.print(c);
                        self.transition(&State::Assists);
                    },
                    _ => ()
                }
            },
            State::Assists => {
                match c {
                    '0'..='9' => {
                        self.print(c);
                    },
                    '\r' => {
                        let line: String = self.buffer.iter().collect();
                        self.restart();
                        return InputFSM::add_command(line);
                    }
                    _ => ()
                }
            },
            State::DeleteCommand => {
                if matches!(c, 'a'..='z' | 'A'..='Z') {
                    self.print(c);
                    self.transition(&State::ValidName);
                }
            },
            State::ValidName => {
                match c {
                    'a'..='z' | 'A'..='Z' => {
                        self.print(c);
                    },
                    ' ' => {
                        self.print(c);
                        self.transition(&State::S2);
                    },
                    '\r' => {
                        let line: String = self.buffer.iter().collect();
                        self.restart();
                        return InputFSM::del_command(line);
                    }
                    _ => ()
                }
            },
            State::S2 => {
                match c {
                    'a'..='z' | 'A'..='Z' => {
                        self.print(c);
                        self.transition(&State::ValidName);
                    },
                    _ => ()
                }
            }
            _ => ()
        }
        None
    }
}