use std::cell::RefCell;
use std::option;
use std::{collections::HashMap, env};

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
    pub fn parse(args: impl Iterator<Item = impl ToString>) -> Self {
        let args = args.map(|s| s.to_string());

        let mut verbs = vec![];
        let options = CfgOptions::new();

        for (_index, s) in args.enumerate() {
            // if Options [
            // if --
            // get next s
            // if -
            // register option
            // ] else
            // if * (verb)
            // add to verbs
            if s.as_str().starts_with("--") {
                todo!()
            } else if s.starts_with('-') {
                options.register(s, None);
            } else {
                verbs.push(s);
            }
        }

        Self { verbs, options }
    }

    pub fn get_option(&mut self, key: &String) -> Option<Option<String>> {
        self.options.get(key)
    }
}

fn main() -> anyhow::Result<()> {
    let args = env::args().skip(1);

    let cfg = Cfg::parse(args);

    println!("{cfg:?}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {}
}
