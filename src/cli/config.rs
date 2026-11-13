//! A module for parsing command line arguments
//!
//! This crate contains the Config struct
use std::cell::Cell;
use std::collections::HashMap;

use crate::AppSuccess;

/// Represents an entry into the flag database.
#[derive(Debug)]
pub struct FlagEntry {
    operand: Option<String>,
    strength: u8,
    /// How many times the entry has been retrieved
    /// This is used to find unused arguments
    retrieve_count: Cell<usize>,
}

impl FlagEntry {
    fn new(operand: Option<String>) -> Self {
        Self {
            operand,
            strength: 1,
            retrieve_count: Cell::from(0),
        }
    }

    /// Returns the operand
    ///
    /// # Examples
    ///
    /// ```
    /// # use doku::cli::AppConfig;
    /// # use std::fmt::format;
    /// let operand = "fish";
    /// let cfg = AppConfig::parse(format!("--param {operand}").split(" "));
    /// assert_eq!(cfg.retrieve_argument("--param").unwrap().operand().unwrap(), "{operand}");
    /// ```
    pub fn operand(&self) -> Option<&String> {
        self.operand.as_ref()
    }

    /// Returns how many times the matching parameter was used
    ///
    /// # Examples
    ///
    /// ```
    /// # use doku::cli::AppConfig;
    /// let cfg = AppConfig::parse("-q -q".split(" "));
    /// assert_eq!(cfg.retrieve_argument("-q").unwrap().strength(), 2);
    /// ```
    pub fn strength(&self) -> u8 {
        self.strength
    }

    /// Returns how many times the parameter was retrieved
    ///
    /// # Examples
    ///
    /// ```
    /// # use doku::cli::AppConfig;
    /// let cfg = AppConfig::parse("-q".split(" "));
    /// cfg.retrieve_argument("-q");
    /// cfg.retrieve_argument("-q");
    /// let arg = cfg.retrieve_argument("-q").unwrap();
    /// assert_eq!(arg.retrieve_count(), 3);
    /// ```
    pub fn retrieve_count(&self) -> usize {
        self.retrieve_count.get()
    }

    fn increment_count(&self) -> &Self {
        let count = self.retrieve_count();
        self.retrieve_count.set(count + 1);
        self
    }
}

// Implemented as a std::collections::HashMap
// It's really just a wrapper around insert and get with some extra logic for retrieve_count() and strength()
#[derive(Debug, Default)]
struct FlagsDb {
    hashmap: HashMap<String, FlagEntry>,
}

impl FlagsDb {
    fn new() -> Self {
        Default::default()
    }

    fn register(&mut self, k: String, v: Option<String>) {
        match self.hashmap.get_mut(&k) {
            Some(entry) => {
                assert!(entry.operand().is_none() && v.is_none()); // Make sure that these are
                // short arguments
                entry.strength += 1;
            }
            None => {
                self.hashmap.insert(k, FlagEntry::new(v));
            }
        }
    }

    fn get(&self, k: &String) -> Option<&FlagEntry> {
        self.hashmap.get(k).map(FlagEntry::increment_count)
    }
}

pub type VerbsDb = Vec<String>;

// Ex: doku version -q --kitten meow -> Cfg{ verbs: ["version"], options: {"-q": None, "meow": Some("Kitten")}}
#[derive(Debug)]
pub struct AppConfig {
    /// Every word in the arguments list that doesn't start with a - or --
    verbs: VerbsDb,
    flags_db: FlagsDb, // {"-q": None, "--ip": Some("192.168.1.25")}
}

pub enum Parameter {
    Short(String),
    Long(String),
    Verb(String),
    /// "--" i.e. End of options delimiter
    EoO,
}

impl Parameter {
    // Returns a Vec because -vx is effectively multiple parameters
    fn create(value: String) -> Vec<Self> {
        use Parameter::*;
        if value.as_str() == "--" {
            vec![EoO]
        } else if value.starts_with("--") {
            vec![Long(value)]
        } else if value.starts_with("-") {
            value
                .chars()
                .skip(1) // Skip -
                .map(|flag| Short(format!("-{flag}")))
                .collect()
        } else {
            vec![Verb(value)]
        }
    }
}

impl AppConfig {
    pub fn parse(args: impl Iterator<Item = impl ToString>) -> Self {
        let mut flags_db = FlagsDb::new();
        let mut verbs = VerbsDb::new();

        use Parameter::*;
        let mut args = args.map(|s| s.to_string());
        while let Some(param) = args.next() {
            for flag in Parameter::create(param) {
                match flag {
                    Short(flag_name) => flags_db.register(flag_name, None),
                    Long(flag_name) => flags_db.register(flag_name, args.next()),
                    Verb(verb) => verbs.push(verb),
                    EoO => verbs.extend(args.by_ref()),
                }
            }
        }

        let cfg = Self { verbs, flags_db };

        log::debug!("Just built a config:\n{cfg:#?}\n---");
        cfg
    }

    pub fn verbs(&self) -> impl Iterator<Item = &String> {
        self.verbs.iter()
    }

    // TODO: its possible to pass an argument such as -vx -- we probably want to warn the caller
    // that this is an invalid key value
    pub fn retrieve_argument(&self, param: &str) -> Option<&FlagEntry> {
        let param = param.to_string();
        self.flags_db.get(&param)
    }

    pub fn transform_app_result(self, remaining_verbs: Vec<String>) -> AppSuccess {
        let useless_flags = self
            .flags_db
            .hashmap
            .into_values()
            .filter(|flag| flag.retrieve_count() == 0)
            .collect();
        AppSuccess {
            remaining_verbs,
            useless_flags,
        }
    }
}
