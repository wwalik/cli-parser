use super::config::*;

pub trait Command {
    fn name(&self) -> &str;
    fn desc(&self) -> &str;
    // TODO: Add long descs
    // fn long_desc(&self) -> Option<&str> {
    //     None
    // }
    fn help(&self) {
        println!("{} : {}", self.name(), self.desc());
    }
    fn execute(&self, cfg: &Cfg, verbs: &mut dyn Iterator<Item = &String>) -> anyhow::Result<()>;
}

pub struct Module {
    name: String,
    desc: String,
    // TODO: long_desc: String
    cmds: Vec<Box<dyn Command>>,
}
impl Module {
    pub fn new(name: &str, desc: &str) -> Self {
        let name = name.to_owned();
        let desc = desc.to_owned();

        Self {
            name,
            desc,
            cmds: vec![],
        }
    }

    pub fn register(&mut self, cmd: Box<dyn Command>) {
        self.cmds.push(cmd);
    }

    // TODO: Maybe this should just be the Display trait
    fn help(&self) {
        println!("-- MODULE --");
        println!("-- {} --", self.name());
        println!("------------");
        println!("{}", self.desc());
        println!("------------");
        for mod_cmd in &self.cmds {
            mod_cmd.help();
        }
    }
}

impl Command for Module {
    fn name(&self) -> &str {
        &self.name
    }
    fn desc(&self) -> &str {
        &self.desc
    }

    fn execute(&self, cfg: &Cfg, verbs: &mut dyn Iterator<Item = &String>) -> anyhow::Result<()> {
        let verb = verbs.next().map_or("", String::as_str);

        match self.cmds.iter().find(|&cmd| cmd.name() == verb) {
            Some(cmd) => cmd.execute(cfg, verbs),
            None => {
                self.help();
                Ok(()) // TODO: Should this be an Err? self.help() already handles this..?
            }
        }
    }
}
