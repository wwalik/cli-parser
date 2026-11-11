mod config {
    use std::cell::Cell;
    use std::collections::HashMap;

    use anyhow::Context;

    #[derive(Debug)]
    pub struct ArgEntry {
        value: Option<String>,
        strength: u8,
        retrieve_count: Cell<usize>,
    }

    impl ArgEntry {
        fn new(value: Option<String>) -> Self {
            Self {
                value,
                strength: 1,
                retrieve_count: Cell::from(0),
            }
        }

        pub fn value(&self) -> Option<&String> {
            self.value.as_ref()
        }

        pub fn strength(&self) -> u8 {
            self.strength
        }

        pub fn retrieve_count(&self) -> usize {
            self.retrieve_count.get()
        }

        fn increment_count(&self) -> &Self {
            let count = self.retrieve_count();
            self.retrieve_count.set(count + 1);
            self
        }
    }

    /// Implemented as a std::collections::HashMap
    #[derive(Debug, Default)]
    struct ArgsDb {
        hashmap: HashMap<String, ArgEntry>,
    }

    impl ArgsDb {
        fn new() -> Self {
            Default::default()
        }

        fn register(&mut self, k: String, v: Option<String>) {
            match self.hashmap.get_mut(&k) {
                Some(entry) => {
                    assert!(entry.value().is_none() && v.is_none()); // Make sure that these are
                    // short arguments
                    entry.strength += 1;
                }
                None => {
                    self.hashmap.insert(k, ArgEntry::new(v));
                }
            }
        }

        fn get(&self, k: &String) -> Option<&ArgEntry> {
            self.hashmap.get(k).map(ArgEntry::increment_count)
        }

        fn unused(&self) -> Vec<(&String, &ArgEntry)> {
            self.hashmap
                .iter()
                .filter(|(_key, value)| value.retrieve_count() == 0)
                .collect()
        }
    }

    // Ex: doku version -q --kitten meow -> Cfg{ verbs: ["version"], options: {"-q": None, "meow": Some("Kitten")}}
    #[derive(Debug)]
    pub struct Cfg {
        verbs: Vec<String>,
        args_db: ArgsDb, // {"-q": None, "--ip": Some("192.168.1.25")}
    }

    impl Cfg {
        // TODO: Handle double/triple flags
        // TODO: Make this function less evil
        // TODO: Handle -- better
        // TODO: Get rid of the anyhow::Result
        pub fn parse(args: impl Iterator<Item = impl ToString>) -> anyhow::Result<Self> {
            let mut args = args.map(|s| s.to_string());

            let mut verbs: Vec<String> = vec![];
            let mut args_db = ArgsDb::new();

            while let Some(s) = args.next() {
                if s.as_str() == "--" {
                    break;
                }

                if s.starts_with("--") {
                    let complement = args
                        .next()
                        .with_context(|| format!("No complement for option {s}"))?;
                    args_db.register(s, Some(complement));
                } else if s.starts_with('-') {
                    for c in s.chars().skip(1) {
                        args_db.register(format!("-{c}"), None);
                    }
                } else if !s.is_empty() {
                    verbs.push(s);
                }
            }

            Ok(Self { verbs, args_db })
        }

        pub fn verbs(&self) -> impl Iterator<Item = &String> {
            self.verbs.iter()
        }
        // TODO: its possible to pass an argument such as -vx -- we probably want to warn the caller
        // that this is an invalid key value
        pub fn retrieve_argument(&self, param: &str) -> Option<&ArgEntry> {
            let param = param.to_string();
            self.args_db.get(&param)
        }

        // I should probably have a general function for both args and verbs at this abstraction level
        pub fn retrieve_unused_arguments(&self) -> Vec<(&String, &ArgEntry)> {
            self.args_db.unused()
        }
    }

    #[cfg(test)]
    mod tests {
        use rand::seq::SliceRandom;

        use super::*;

        // ------------------------ Retrieving -------------------------
        #[test]
        fn unused_arguments_test() {
            let cmd = "blabla -q";
        }
        // ------------------------ Retrieving -------------------------
        #[test]
        fn retrieve_count_test() {
            let cmd = "blabla -q";

            let cfg = Cfg::parse(cmd.split(" ")).unwrap();

            cfg.retrieve_argument("-q");
            cfg.retrieve_argument("-q");

            let entry = cfg.retrieve_argument("-q").unwrap();
            assert_eq!(entry.retrieve_count(), 3)
        }

        // ------------------------ PARSING -------------------------
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
        fn general_config_parsing_test() {
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
        let pretty_display = match cfg.retrieve_argument("-q").map(ArgEntry::strength) {
            None => "The current version is ",
            Some(1) => "v:",
            Some(_) => "",
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

        for (key, arg) in cfg.retrieve_unused_arguments().into_iter() {
            eprintln!("Unused parameter: ({key}) with argument:\n({arg:#?})\n---");
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
