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

        // TODO: FEATURE: Add a method to get unused optionks so far
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
        // TODO: I hate this function so much REFACTOR
        // TODO: Handle negative numbers -5 etc...
        // TODO: Handle --
        pub fn parse(args: impl Iterator<Item = impl ToString>) -> anyhow::Result<Self> {
            let args: Vec<String> = args.map(|s| s.to_string()).collect();

            let mut verbs: Vec<String> = vec![];
            let mut options = CfgOptions::new();

            let mut index = 0;
            while let Some(s) = args.get(index) {
                if s.as_str().starts_with("--") {
                    let complement = args
                        .get(index + 1)
                        .context(format!("No complement for option {s}"))?;
                    options.register(s.clone(), Some(complement.clone()));
                    index += 1;
                } else if s.starts_with('-') {
                    for c in s.chars().skip(1) {
                        options.register(format!("-{c}"), None);
                    }
                } else if !s.is_empty() {
                    verbs.push(s.clone());
                }

                index += 1;
            }

            Ok(Self { verbs, options })
        }

        // TODO: ARE WE SURE ABOUT RETURNING AN OPTION<BOOL> FOR THIS??????
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
            let cmd = "firstverb -q secondverb --long complement -vx ".to_string();
            Cfg::parse(cmd.split(' ')).unwrap()
        }

        #[test]
        fn test_cfg_parsing() {
            let cfg = create_test_cfg();

            // Test verbs
            assert_eq!(
                cfg.verbs,
                vec!["firstverb".to_string(), "secondverb".to_string()]
            );

            // Test options
            let hm = cfg.options.0.into_inner();
            assert_eq!(
                *hm.get("-q")
                    .expect("Could not find -q in CfgOptions when it should be there"),
                None
            );

            assert_eq!(
                *hm.get("-v")
                    .expect("Could not find -v in CfgOptions when it should be there"),
                None
            );
            assert_eq!(
                *hm.get("-x")
                    .expect("Could not find -x in CfgOptions when it should be there"),
                None
            );

            assert_eq!(
                *hm.get("--long")
                    .expect("Could not find --long in CfgOptions when it should be there"),
                Some("complement".to_string())
            )
        }
        #[test]
        fn test_cfg_options() {
            let cfg = create_test_cfg();
            println!("{cfg:#?}");

            cfg.get_short_option("-q")
                .expect("Should have been able to find short option -q");

            let complement = cfg
                .get_long_option("--long")
                .expect("Should have been able to find long option --long");
            assert_eq!(complement.as_str(), "complement")
        }
    }
}

use std::env;

fn main() -> anyhow::Result<()> {
    let args = env::args().skip(1);
    let cfg = config::Cfg::parse(args)?;

    println!("{cfg:#?}");

    Ok(())
}
