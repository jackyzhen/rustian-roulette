use std::collections::HashMap;
use std::env::{self, Args};
use std::path::{Path, PathBuf};

pub struct Config {
    pub chambers: usize,
    pub path: PathBuf,
}

const DEFAULT_CHAMBERS: usize = 6;
const CHAMBER_ARG: &str = "chamber";
const PATH_ARG: &str = "path";

impl Config {
    pub fn new(args: Args) -> Config {
        let home = env::home_dir().expect("could not get home dir");
        let args = &args.collect();
        let args = parse_args(args);

        let chambers = match args.get(CHAMBER_ARG) {
            Some(val) => val.parse::<usize>()
                .expect("unable to parse chamber arg as usize, should never happen"),
            None => DEFAULT_CHAMBERS,
        };

        let path = match args.get(PATH_ARG) {
            Some(val) => Path::new(val).to_path_buf(),
            None => home,
        };

        Config { chambers, path }
    }
}

// parse_args returns a hashmap of key value arguments
// if an argument has a flag (either starts with '-' or '--')
// that flag becomes a key, and the value is either a str value of
// 'true' or the next non flag value.
// Values without a flag prefix will try to be parsed as a usize
// for the CHAMBER flag, or as a string for the PATH flag.
fn parse_args(args: &Vec<String>) -> HashMap<&str, &str> {
    let mut map = HashMap::new();
    let mut arg_iter = args.iter();
    let mut flagged = false;
    let mut flag = "";

    arg_iter.next();
    for arg in arg_iter {
        if arg.starts_with("-") {
            if flagged {
                map.entry(flag).or_insert("true");
            } else {
                flagged = true;
            }
            flag = arg.trim_matches('-');
            continue;
        }
        if flagged {
            map.entry(flag).or_insert(arg);
            flagged = false;
            continue;
        }
        if let Ok(_) = arg.parse::<usize>() {
            map.entry(CHAMBER_ARG).or_insert(arg);
        } else {
            map.entry(PATH_ARG).or_insert(arg);
        }
    }
    if flagged {
        map.entry(flag).or_insert("true");
    }
    map
}
