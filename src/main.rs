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

        fn from(h: HashMap<String, Option<String>>) -> Self {
            Self(RefCell::from(h))
        }

        fn register(&self, k: String, v: Option<String>) -> Option<Option<String>> {
            self.0.borrow_mut().insert(k, v)
        }

        // TODO: FEATURE: Add a method to get unused optionks so far WITHOUT REMOVING
        /// consumes the option
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
        pub fn parse(args: impl Iterator<Item = impl ToString>) -> anyhow::Result<Self> {
            let args: Vec<String> = args.map(|s| s.to_string()).collect();

            let mut verbs: Vec<String> = vec![];
            let options = CfgOptions::new();

            let mut index = 0;
            while let Some(s) = args.get(index) {
                if s.as_str().starts_with("--") {
                    let complement = args
                        .get(index + 1)
                        .context("No complement for option {s}")?;
                    options.register(s.clone(), Some(complement.clone()));
                    index += 1;
                    continue;
                } else if s.starts_with('-') {
                    options.register(s.clone(), None);
                } else {
                    verbs.push(s.clone());
                }

                index += 1;
            }

            Ok(Self { verbs, options })
        }

        pub fn get_option(&mut self, key: &String) -> Option<Option<String>> {
            self.options.get(key)
        }
    }

    // #[cfg(test)]
    // mod tests {
    //     use super::*;

    //     #[test]
    //     fn test1() {}
    // }
}

use std::env;

fn main() -> anyhow::Result<()> {
    let args = env::args().skip(1);

    let cfg = config::Cfg::parse(args);

    println!("{cfg:?}");

    Ok(())
}
