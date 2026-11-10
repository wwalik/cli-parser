mod config {
    use std::cell::RefCell;
    use std::collections::HashMap;

    use anyhow::Context;

    #[derive(Debug)]
    pub struct ArgEntry {
        value: Option<String>,
        strength: u8,
        retrieve_count: usize,
    }

    impl ArgEntry {
        pub fn value(&self) -> Option<&String> {
            self.value.as_ref()
        }

        pub fn strength(&self) -> u8 {
            self.strength
        }

        fn retrieve_count(&self) -> usize {
            self.retrieve_count
        }
    }

    #[derive(Debug)]
    struct ArgsDb(RefCell<HashMap<String, ArgEntry>>); // Is this idiomatic?????????

    // TODO: FEATURE: Add a method to get unused optionks so far
    impl ArgsDb {
        fn new() -> Self {
            Self(RefCell::from(HashMap::default()))
        }

        // fn from(h: HashMap<String, Option<String>>) -> Self {
        //     Self(RefCell::from(h))
        // }

        fn register(&mut self, k: String, v: Option<String>) -> Option<ArgEntry> {
            let arg = ArgEntry {
                value: v,
                strength: 1,
                retrieve_count: 0,
            };

            self.0.borrow_mut().insert(k, arg)
        }

        fn get(&self, k: &String) -> Option<ArgEntry> {
            self.0.borrow_mut().remove(k)
        }
    }

    // TODO: Doc test rust
    // Ex: doku version -q --kitten meow -> Cfg{ verbs: ["version"], options: {"-q": None, "meow": Some("Kitten")}}
    #[derive(Debug)]
    pub struct Cfg {
        verbs: Vec<String>,
        options: ArgsDb, // {"-q": None, "--ip": Some("192.168.1.25")}
    }

    impl Cfg {
        // TODO: Handle double/triple flags
        // TODO: Make this function less evil
        pub fn parse(args: impl Iterator<Item = impl ToString>) -> anyhow::Result<Self> {
            let mut args = args.map(|s| s.to_string());

            let mut verbs: Vec<String> = vec![];
            let mut options = ArgsDb::new();

            while let Some(s) = args.next() {
                if s.as_str() == "--" {
                    break;
                }

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
        // TODO: its possible to pass an argument such as -vx -- we probably want to warn the caller
        // this is impossible
        pub fn retrieve_argument(&self, param: &str) -> Option<ArgEntry> {
            let param = param.to_string();
            self.options.get(&param)
        }
    }

    #[cfg(test)]
    mod tests {
        use rand::seq::SliceRandom;

        use super::*;

        fn gen_verbs() -> Vec<String> {
            let v = ["firstverb", "secondverb"];

            v.iter().map(|s| s.to_string()).collect()
        }
        fn gen_shorts() -> Vec<(String, Option<String>)> {
            let v = ["-q", "-vx", "-é"];

            v.iter().map(|s| (s.to_string(), None)).collect()
        }
        fn gen_longs() -> Vec<(String, Option<String>)> {
            let v = [("--long", "complement"), ("--kitten", "meow")];

            v.iter()
                .map(|s| (s.0.to_string(), Some(s.1.to_string())))
                .collect()
        }

        #[test]
        fn general_config_parsing() {
            let verbs = gen_verbs();
            let shorts = gen_shorts();
            let longs = gen_longs();

            let mut params: Vec<String> = [shorts.clone(), longs.clone()]
                .concat()
                .iter()
                .map(|s| {
                    let s0 = s.0.clone();
                    match &s.1 {
                        Some(s1) => s0 + " " + s1.as_str(),
                        None => s0,
                    }
                })
                .collect();
            params.shuffle(&mut rand::rng());

            let cmd: Vec<String> = [verbs.clone(), params]
                .concat()
                .join(" ")
                .split(" ")
                .map(str::to_string)
                .collect();
            println!("The config will parse this command: \n{cmd:?}\n---");

            let cfg = Cfg::parse(cmd.iter()).expect("Error parsing");
            println!("This is the parsed config: \n{cfg:#?}\n---");

            // Verify verbs
            verbs
                .iter()
                .zip(cfg.verbs.iter())
                .for_each(|(v, v_parsed)| {
                    assert_eq!(*v, *v_parsed);
                });

            // Verify shorts
            for (param, _) in shorts {
                println!("{param}");
                for c in param.chars().skip(1) {
                    println!("-{c}");
                    assert!(cfg.retrieve_argument(format!("-{c}").as_str()).is_some());
                }
            }

            // Verify longs
            for (param, complement_wrapped) in longs {
                let complement = complement_wrapped.expect("Should have complement");

                let entry = cfg
                    .retrieve_argument(param.as_str())
                    .expect("Should have found ArgEntry for {param}");
                assert_eq!(
                    entry
                        .value()
                        .expect("Long options should have values")
                        .as_str(),
                    complement.as_str()
                );
            }
        }
        #[ignore = "Unimplemented"]
        #[test]
        fn general_config_parsing_failure() {
            unimplemented!()
        }
    }
}

mod app {
    use crate::config::*;

    fn version(cfg: &Cfg) {
        let version = "0.1.0";
        let pretty_display = if !cfg.retrieve_argument("-q").is_some() {
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
