mod commands {
    use cli_parser::{AppConfig, Command, ExecuteResult, FlagEntry};

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
            verbs: &mut dyn Iterator<Item = &String>,
        ) -> ExecuteResult {
            let version = env!("CARGO_PKG_VERSION");
            let pretty_display = match cfg.retrieve_argument("-q").map(FlagEntry::strength) {
                None => "The current version is ",
                Some(1) => "v:",
                Some(_) => "",
            };

            println!("{pretty_display}{version}");

            Ok(verbs.cloned().collect())
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
        ) -> ExecuteResult {
            for verb in verbs {
                print!("{verb}");
            }

            Ok(vec![])
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
            verbs: &mut dyn Iterator<Item = &String>,
        ) -> ExecuteResult {
            println!("hi");

            Ok(verbs.cloned().collect())
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
            ) -> ExecuteResult {
                let pretty_display = match cfg.retrieve_argument("-q") {
                    Some(_) => "",
                    None => "The sum is ",
                };
                let sum: f32 = verbs.filter_map(|x| x.parse::<f32>().ok()).sum();
                println!("{pretty_display}{sum}");

                Ok(verbs.cloned().collect())
            }
        }
    }
}

use std::env;

use cli_parser::App;
use cli_parser::Module;

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
    app.run().map(|app_success| app_success.log())
}
