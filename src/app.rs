use super::config::{ArgEntry, Cfg};
use super::modcmd::Command;

// TODO: I think I want to be able to ? inside of my <impl Command> structs, so i need a way to
// return an error
pub struct AppResult {
    remaining_verbs: Vec<String>,
    cfg: Cfg,
}

impl AppResult {
    pub fn remaining_verbs(&self) -> &Vec<String> {
        &self.remaining_verbs
    }
    pub fn unused_arguments(&self) -> Vec<(&String, &ArgEntry)> {
        self.cfg.retrieve_unused_arguments()
    }
}
pub struct App {
    cfg: Cfg,
    topmod: super::modcmd::Module,
}

impl App {
    pub fn new(args: impl Iterator<Item = String>, topmod: super::modcmd::Module) -> Self {
        let cfg = Cfg::parse(args);
        Self { cfg, topmod }
    }
    pub fn run(self) -> AppResult {
        let mut verbs = self.cfg.verbs();

        self.topmod.execute(&self.cfg, &mut verbs);

        AppResult {
            remaining_verbs: verbs.cloned().collect(),
            cfg: self.cfg,
        }
    }
}
