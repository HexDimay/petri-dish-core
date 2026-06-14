use crate::cell::cmd::Command;

#[derive(Debug, Clone)]
pub struct Genome {
    step: usize,
    commands: Vec<Command>,
}

impl Genome {
    pub fn new() -> Self {
        Self {
            step: 0,
            commands: vec![],
        }
    }

    pub fn get_current(&self) -> &Command {
        &self.commands[self.step]
    }

    pub fn next(&mut self) {
        self.step += 1;

        if self.step >= self.commands.len() {
            self.step = 0;
        }
    }
}
