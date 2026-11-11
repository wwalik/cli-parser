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
        fn execute(&self, cfg: &Cfg, verbs: &mut dyn Iterator<Item = &String>);
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

        fn execute(&self, cfg: &Cfg, verbs: &mut dyn Iterator<Item = &String>) {
            let verb = match verbs.next() {
                Some(s) => s,
                None => {
                    self.help();
                    ""
                }
            };

            for cmd in &self.cmds {
                if verb == cmd.name() {
                    cmd.execute(cfg, verbs);
                }
            }
        }
    }
