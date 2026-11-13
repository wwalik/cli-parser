use std::fmt;

use crate::cli::{AppConfig, Command, FlagEntry, Module};

pub struct AppSuccess {
    pub remaining_verbs: Vec<String>,
    pub useless_flags: Vec<FlagEntry>,
}

impl AppSuccess {
    pub fn log(&self) {
        let AppSuccess {
            useless_flags,
            remaining_verbs,
        } = self;

        useless_flags
            .iter()
            .for_each(|flag| log::warn!("Useless Flag: {flag:?}"));

        remaining_verbs
            .iter()
            .for_each(|verb| log::warn!("Useless verb: {verb}"));
    }
}

impl fmt::Display for AppSuccess {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let AppSuccess {
            remaining_verbs,
            useless_flags: unused_arguments,
        } = self;

        write!(f, "AppResult: {remaining_verbs:?} {unused_arguments:#?}")
    }
}
pub struct App {
    cfg: AppConfig,
    topmod: Module,
}

impl App {
    pub fn new(args: impl Iterator<Item = String>, topmod: Module) -> Self {
        let cfg = AppConfig::parse(args);
        Self { cfg, topmod }
    }
    pub fn run(self) -> anyhow::Result<AppSuccess> {
        let mut verbs = self.cfg.verbs();

        self.topmod.execute(&self.cfg, &mut verbs)?;

        let remaining_verbs = verbs.cloned().collect();
        let app_result = self.cfg.transform_app_result(remaining_verbs);

        log::debug!("App Result: {app_result}");
        Ok(app_result)
    }
}
