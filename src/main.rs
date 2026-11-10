use std::cell::RefCell;
use std::{collections::HashMap, env};

#[derive(Debug)]
struct CfgOptions(RefCell<HashMap<String, Option<String>>>); // Is this idiomatic?????????

// TODO: FEATURE: Add a method to get unused optionks so far
impl CfgOptions {
    fn new(h: HashMap<String, Option<String>>) -> Self {
        Self(RefCell::from(h))
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
    // TODO: Rewrite this to handle long options
    pub fn parse(args: impl Iterator<Item = impl ToString>) -> Self {
        let args = args.map(|s| s.to_string());

        let mut options = HashMap::<String, Option<String>>::new();
        let verbs = args
            .filter_map(|s| {
                if s.starts_with('-') {
                    options.insert(s, None);
                    None
                } else {
                    Some(s)
                }
            })
            .collect();
        let options = CfgOptions::new(options);

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
