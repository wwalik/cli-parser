mod app;
mod config;
mod modcmd;

mod commands {
    use std::env;

    use super::config::{ArgEntry, Cfg};
    use super::modcmd::Command;

    pub struct Version;
    impl Command for Version {
        fn name(&self) -> &str {
            "version"
        }
        fn desc(&self) -> &str {
            "version"
        }
        fn execute(&self, cfg: &Cfg, _verbs: &mut dyn Iterator<Item = &String>) {
            let version = env!("CARGO_PKG_VERSION");
            let pretty_display = match cfg.retrieve_argument("-q").map(ArgEntry::strength) {
                None => "The current version is ",
                Some(1) => "v:",
                Some(_) => "",
            };

            println!("{pretty_display}{version}");
        }
    }

    pub struct Echo;
    impl Command for Echo {
        fn name(&self) -> &str {
            "echo"
        }
        fn desc(&self) -> &str {
            "echo its arguments"
        }
        fn execute(&self, _cfg: &Cfg, verbs: &mut dyn Iterator<Item = &String>) {
            for verb in verbs {
                print!("{verb}");
            }
        }
    }

    pub struct Hi;
    impl Command for Hi {
        fn name(&self) -> &str {
            "hi"
        }
        fn desc(&self) -> &str {
            "says hi"
        }
        fn execute(&self, _cfg: &Cfg, _verbs: &mut dyn Iterator<Item = &String>) {
            println!("hi");
        }
    }

    pub struct MathSum;
    impl Command for MathSum {
        fn name(&self) -> &str {
            "sum"
        }
        fn desc(&self) -> &str {
            "sum all its arguments"
        }
        fn execute(&self, cfg: &Cfg, verbs: &mut dyn Iterator<Item = &String>) {
            let pretty_display = match cfg.retrieve_argument("-q") {
                Some(_) => "",
                None => "The sum is ",
            };
            let sum: f32 = verbs.filter_map(|x| x.parse::<f32>().ok()).sum();
            println!("{pretty_display}{sum}");
        }
    }
}

use std::env;

use app::App;
use modcmd::Module;

// TODO: Create tests for the modules and commands
fn main() {
    env_logger::init();

    let mut topmod = Module::new("App", "App description"); // Maybe? why not?
    topmod.register(Box::from(commands::Version)); // TODO: I hate this syntax :(
    topmod.register(Box::from(commands::Echo));
    topmod.register(Box::from(commands::Hi));

    let mut submod = Module::new("math", "do math");
    submod.register(Box::from(commands::MathSum));

    topmod.register(Box::from(submod));

    let args = env::args().skip(1);
    let app = App::new(args, topmod);

    // TODO: This needs to be a result
    let app_result = app.run();
    for r_verb in app_result.remaining_verbs() {
        eprintln!("Remaining_verbs (good): {r_verb}");
    }

    for (key, arg) in app_result.unused_arguments().iter() {
        eprintln!("Useless parameter: ({key})");
        log::debug!("Useless ArgEntry:\n{arg:?}\n---");
    }
}
