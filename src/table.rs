use std::io::{self, Write};
use crate::input_fsm::Command;
use ansi_escapes;

pub fn redraw_top(table: &StatsTable, top: usize) {
    let top = std::cmp::min(top, table.entries.len());

    // erase current table
    print!("{}", ansi_escapes::CursorSavePosition);
    for _ in 0..=top+1 {
        print!("{}{}", ansi_escapes::CursorDown(1), ansi_escapes::EraseLine);
    }
    print!("{}", ansi_escapes::CursorRestorePosition);

    print!("{}", ansi_escapes::CursorSavePosition);
    println!();
    println!("-------------");
    for entry in &table.entries[0..top] {
        println!("{} {} {}", entry.name(), entry.goals, entry.assists);
    }
    print!("{}", ansi_escapes::CursorRestorePosition);
    io::stdout().flush().unwrap();
}

pub struct StatsTableEntry {
    name: String,
    pub goals: u32,
    pub assists: u32
}

impl StatsTableEntry {
    pub fn from(name: &str, goals: u32, assists: u32) -> Option<StatsTableEntry> {
        if !name.is_empty() {
            let name = String::from(name);
            Some(StatsTableEntry { name, goals, assists })
        }
        else {
            None
        }
    }

    pub fn from_command(command: Command) -> Option<StatsTableEntry> {
        if let Command::Add(name, goals, assists) = command {
            Some(StatsTableEntry { name, goals, assists })
        }
        else {
            None
        }
    }

    pub fn parse(entry: String) -> Option<StatsTableEntry> {
        let mut entry = entry.split_whitespace();

        let name = match entry.next() {
            Some(name) => Some(name),
            None => None
        };

        let goals = match entry.next() {
            Some(goals) => Some(goals),
            None => None
        };

        let assists = match entry.next() {
            Some(assists) => Some(assists),
            None => None
        };

        match (name, goals, assists) {
            (Some(name), Some(goals), Some(assists)) => {
                if let (Ok(goals), Ok(assists)) = (goals.parse::<u32>(), assists.parse::<u32>()) {
                    StatsTableEntry::from(
                        name,
                        goals,
                        assists
                    )
                }
                else {
                    None
                }
            },
            _ => None
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }
}

pub struct StatsTable {
    entries: Vec<StatsTableEntry>
}

impl StatsTable {
    pub fn new() -> StatsTable {
        StatsTable { entries: vec![] }
    }

    fn push(&mut self, data: StatsTableEntry) {
        for entry in self.entries.iter_mut() {
            if entry.name == data.name {
                entry.goals += data.goals;
                entry.assists += data.assists;
                return;
            }
        }

        self.entries.push(data);
    }

    fn remove(&mut self, name: String) {
        let index = self.entries.iter().position(|x| *x.name() == name);
        if let Some(index) = index {
            self.entries.remove(index);
        }
    }

    pub fn execute_command(&mut self, command: Command) {
        match command {
            Command::Add(_, _, _) => self.push(StatsTableEntry::from_command(command).unwrap()),
            Command::Delete(name) => self.remove(name)
        }
    }

    pub fn entries(&self) -> &Vec<StatsTableEntry> {
        &self.entries
    }
}