use super::config;
use super::config::AppConfig;

pub type ExecuteResult = anyhow::Result<config::VerbsDb>;

pub trait Command {
    fn name(&self) -> &str;
    fn desc(&self) -> &str;
    fn brief(&self) -> String {
        self.name().to_owned() + ": " + self.desc()
    }
    // TODO: Add long descs
    // fn long_desc(&self) -> Option<&str> {
    //     None
    // }
    fn execute(&self, cfg: &AppConfig, verbs: &mut dyn Iterator<Item = &String>) -> ExecuteResult;
}

/// A predefined impl of Command that consists of a list of commands. A "subcommand"
/// e.g. docker CONTAINER ps
pub struct MetaCommand {
    name: String,
    desc: String,
    cmds: Vec<Box<dyn Command>>,
}

impl MetaCommand {
    pub fn new(name: &str, desc: &str) -> Self {
        let name = name.to_owned();
        let desc = desc.to_owned();

        Self {
            name,
            desc,
            cmds: vec![],
        }
    }

    pub fn register<C: Command + 'static>(&mut self, cmd: C) -> &mut Self {
        self.cmds.push(Box::new(cmd));
        self
    }

    // TODO: Maybe this should just be the Display trait
    fn help(&self) {
        println!("-- META COMMAND --");
        println!("-- {} --", self.name());
        println!("------------");
        println!("{}", self.desc());
        println!("------------");
        for cmd in &self.cmds {
            println!("{}", cmd.brief());
        }
    }
}

impl Command for MetaCommand {
    fn name(&self) -> &str {
        &self.name
    }
    fn desc(&self) -> &str {
        &self.desc
    }

    fn execute(&self, cfg: &AppConfig, verbs: &mut dyn Iterator<Item = &String>) -> ExecuteResult {
        let verb = verbs.next().map_or("", String::as_str);

        match self.cmds.iter().find(|&cmd| cmd.name() == verb) {
            Some(cmd) => cmd.execute(cfg, verbs),
            None => {
                self.help();
                Ok(verbs.cloned().collect())
            }
        }
    }
}
