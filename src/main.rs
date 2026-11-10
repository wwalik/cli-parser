mod config {
    use std::cell::RefCell;
    use std::collections::HashMap;

    use anyhow::Context;

    #[derive(Debug)]
    struct CfgOptions(RefCell<HashMap<String, Option<String>>>); // Is this idiomatic?????????

    // TODO: FEATURE: Add a method to get unused optionks so far
    impl CfgOptions {
        fn new() -> Self {
            Self(RefCell::from(HashMap::default()))
        }

        // fn from(h: HashMap<String, Option<String>>) -> Self {
        //     Self(RefCell::from(h))
        // }

        fn register(&mut self, k: String, v: Option<String>) -> Option<Option<String>> {
            self.0.borrow_mut().insert(k, v)
        }

        fn get(&self, k: &String) -> Option<Option<String>> {
            self.0.borrow_mut().remove(k)
        }
    }

    // TODO: Doc test rust
    // Ex: doku version -q --kitten meow -> Cfg{ verbs: ["version"], options: {"-q": None, "meow": Some("Kitten")}}
    #[derive(Debug)]
    pub struct Cfg {
        verbs: Vec<String>,
        options: CfgOptions, // {"-q": None, "--ip": Some("192.168.1.25")}
    }

    impl Cfg {
        // TODO: Handle --
        // TODO: Handle double/triple flags
        pub fn parse(args: impl Iterator<Item = impl ToString>) -> anyhow::Result<Self> {
            let mut args = args.map(|s| s.to_string());

            let mut verbs: Vec<String> = vec![];
            let mut options = CfgOptions::new();

            while let Some(s) = args.next() {
                if s.starts_with("--") {
                    let complement = args
                        .next()
                        .with_context(|| format!("No complement for option {s}"))?;
                    options.register(s, Some(complement));
                } else if s.starts_with('-') {
                    for c in s.chars().skip(1) {
                        options.register(format!("-{c}"), None);
                    }
                } else if !s.is_empty() {
                    verbs.push(s);
                }
            }

            Ok(Self { verbs, options })
        }

        pub fn verbs(&self) -> impl Iterator<Item = &String> {
            self.verbs.iter()
        }

        // TODO: ARE WE SURE ABOUT RETURNING AN OPTION<BOOL> FOR THIS??????
        // TODO: its possible to pass an argument such as -vx -- we probably want to warn the caller
        // this is impossible
        pub fn get_short_option(&self, key: &str) -> Option<()> {
            let key = key.to_string();

            self.options.get(&key).map(|_| ())
        }
        pub fn get_long_option(&self, key: &str) -> Option<String> {
            let key = key.to_string();

            self.options.get(&key).map(|o_s| {
                o_s.expect(
                    "LOGIC ERROR in CfgOptions: Tried to query a short option as a long option",
                )
            })
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        fn create_test_cfg() -> Cfg {
            let cmd = "firstverb -q secondverb --long complement -vx -é".to_string();
            Cfg::parse(cmd.split(' ')).unwrap()
        }

        #[test]
        fn test_cfg_basic_parsing() {
            let cfg = create_test_cfg();

            // Test verbs
            assert_eq!(
                cfg.verbs,
                vec!["firstverb".to_string(), "secondverb".to_string()]
            );

            // Test options
            let hm: HashMap<String, Option<String>> = cfg.options.0.into_inner();
            assert_eq!(
                *hm.get("-q").expect(
                    "Could not find -q in CfgOptions when it should be there | stdlib error"
                ),
                None
            );

            // Long options
            assert_eq!(
                *hm.get("--long").expect(
                    "Could not find --long in CfgOptions when it should be there | stdlib error"
                ),
                Some("complement".to_string())
            );

            // Fake options
            assert_eq!(hm.get("-$"), None);
        }
        #[test]
        fn test_cfg_option_retrieving() {
            let cfg = create_test_cfg();
            println!("{cfg:#?}");

            // -------- Short options
            cfg.get_short_option("-q")
                .expect("Should have been able to find short option -q");

            cfg.get_short_option("-v")
                .expect("Should have been able to find short option -q");
            cfg.get_short_option("-x")
                .expect("Should have been able to find short option -q");

            // -------- Long options
            let complement = cfg
                .get_long_option("--long")
                .expect("Should have been able to find long option --long");
            assert_eq!(complement.as_str(), "complement");

            // -------- Fake options
            for param in ["$", "-$", "--$"] {
                if cfg.get_short_option("$").is_some() {
                    panic!("Expected Option::None, but got Option::Some(()");
                }
                if let Some(argument) = cfg.get_long_option("$") {
                    panic!(
                        "Expected Option::None, but got Option::Some({argument}) for parameter {param}"
                    );
                }
            }
        }
    }
}

mod app {
    use crate::config::*;

    fn version(cfg: &Cfg) {
        let version = "0.1.0";
        let pretty_display = if !cfg.get_short_option("-q").is_some() {
            "The current version is "
        } else {
            ""
        };

        println!("{pretty_display}{version}");
    }

    pub fn run(cfg: &Cfg) -> anyhow::Result<()> {
        for verb in cfg.verbs() {
            match verb.as_str() {
                "version" => version(cfg),
                _ => eprintln!("Unknown verb parameter --> {verb}"),
            }
        }

        Ok(())
    }
}

use std::env;

fn main() -> anyhow::Result<()> {
    let args = env::args().skip(1);
    let cfg = config::Cfg::parse(args)?;

    println!("{cfg:#?}");

    app::run(&cfg)
}
