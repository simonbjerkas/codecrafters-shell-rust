mod cd;
mod describe;
mod echo;
mod exit;
mod history;
mod pwd;

use std::fmt::Debug;

use cd::Cd;
use describe::Describe;
use echo::Echo;
use exit::Exit;
use history::History;
use pwd::Pwd;

use super::{ShellCtx, ShellError};
use anyhow::Result;

#[derive(Debug)]
pub struct Builtins;

impl Builtins {
    pub fn new(cmd: &str) -> Option<Box<dyn ShellCommand>> {
        match cmd {
            "exit" => Some(Box::new(Exit)),
            "echo" => Some(Box::new(Echo)),
            "pwd" => Some(Box::new(Pwd)),
            "type" => Some(Box::new(Describe)),
            "cd" => Some(Box::new(Cd)),
            "history" => Some(Box::new(History)),
            _ => None,
        }
    }

    pub fn all_builtins() -> Vec<&'static str> {
        Vec::from(["exit", "echo", "pwd", "type", "cd", "history"])
    }
}

pub enum ExecResult {
    Continue,
    Res(String),
    Exit(i32),
}

pub trait ShellCommand: Debug {
    fn name(&self) -> &str;
    fn description(&self) -> String {
        format!("{} is a shell builtin", self.name())
    }
    fn execute(&self, args: &Vec<String>, ctx: &mut ShellCtx) -> Result<ExecResult>;
}
