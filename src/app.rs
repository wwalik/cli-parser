use std::fmt;

use super::config::{ArgEntry, Cfg};
use super::modcmd::Command;

pub struct AppResult {
    pub remaining_verbs: Vec<String>,
    pub unused_arguments: Vec<(String, ArgEntry)>,
}

impl fmt::Display for AppResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let AppResult {
            remaining_verbs,
            unused_arguments,
        } = self;

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

        let remaining_verbs = verbs.cloned().collect();
        let app_result = self.cfg.transform_app_result(remaining_verbs);

        log::debug!("App Result: {app_result}");
        Ok(app_result)
    }
}
