use std::fmt;

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

impl fmt::Display for AppResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let remaining_verbs = self.remaining_verbs();
        let unused_arguments = self.unused_arguments();

        write!(f, "AppResult: {remaining_verbs:?} {unused_arguments:#?}")
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
    pub fn run(self) -> anyhow::Result<AppResult> {
        let mut verbs = self.cfg.verbs();

        self.topmod.execute(&self.cfg, &mut verbs)?;

        let app_result = AppResult {
            remaining_verbs: verbs.cloned().collect(),
            cfg: self.cfg,
        };

        log::debug!("App Result: {app_result}");
        Ok(app_result)
    }
}
