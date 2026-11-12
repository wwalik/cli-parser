mod commands {
    use anyhow::Result;

    use doku::cli::{AppConfig, Command, FlagEntry};

    pub struct Version;
    impl Command for Version {
        fn name(&self) -> &str {
            "version"
        }
        fn desc(&self) -> &str {
            "version"
        }
        fn execute(
            &self,
            cfg: &AppConfig,
            _verbs: &mut dyn Iterator<Item = &String>,
        ) -> Result<()> {
            let version = env!("CARGO_PKG_VERSION");
            let pretty_display = match cfg.retrieve_argument("-q").map(FlagEntry::strength) {
                None => "The current version is ",
                Some(1) => "v:",
                Some(_) => "",
            };

            println!("{pretty_display}{version}");
            Ok(())
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
        fn execute(
            &self,
            _cfg: &AppConfig,
            verbs: &mut dyn Iterator<Item = &String>,
        ) -> Result<()> {
            for verb in verbs {
                print!("{verb}");
            }

            Ok(())
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
        fn execute(
            &self,
            _cfg: &AppConfig,
            _verbs: &mut dyn Iterator<Item = &String>,
        ) -> anyhow::Result<()> {
            println!("hi");
            Ok(())
        }
    }

    pub mod math {
        use super::*;
        pub struct Sum;
        impl Command for Sum {
            fn name(&self) -> &str {
                "sum"
            }
            fn desc(&self) -> &str {
                "sum all its arguments"
            }
            fn execute(
                &self,
                cfg: &AppConfig,
                verbs: &mut dyn Iterator<Item = &String>,
            ) -> Result<()> {
                let pretty_display = match cfg.retrieve_argument("-q") {
                    Some(_) => "",
                    None => "The sum is ",
                };
                let sum: f32 = verbs.filter_map(|x| x.parse::<f32>().ok()).sum();
                println!("{pretty_display}{sum}");

                Ok(())
            }
        }
    }
}

use std::env;

use doku::cli::App;
use doku::cli::Module;

// TODO: This should really be automated by macros...
fn create_topmod() -> Module {
    let mut submod = Module::new("math", "do math");
    submod.register(commands::math::Sum);

    let mut topmod = Module::new("App", "App description");
    // TODO: This should also have a macro...
    topmod
        .register(commands::Version)
        .register(commands::Echo)
        .register(commands::Hi)
        .register(submod);

    topmod
}

fn main() -> anyhow::Result<()> {
    env_logger::init();

    let app = App::new(env::args().skip(1), create_topmod());
    let app_result = app.run()?;

    for (key, arg) in app_result.useless_flags.iter() {
        eprintln!("Useless parameter: ({key})");
        log::debug!("Useless ArgEntry:\n{arg:?}\n---");
    }

    Ok(())
}
