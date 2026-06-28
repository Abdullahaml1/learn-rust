use std::{env, error, fs};

pub struct Config {
    pub query: String,
    pub filepath: String,
    pub case_insensitive: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            case_insensitive: false, // creating default for Config
            query: String::new(),
            filepath: String::new(),
        }
    }
}

impl Config {
    pub fn new(args: &[String]) -> Self {
        if args.len() < 3 {
            panic!("No enought args to produce search");
        }
        let query = args[1].clone();
        let filepath = args[2].clone();

        Self {
            query,
            filepath,
            ..Default::default()
        }
    }
    pub fn build(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("No enough args to produce search");
        }
        let query = args[1].clone();
        let filepath = args[2].clone();

        // NOTE: using env variables
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Self {
            query,
            filepath,
            ..Default::default()
        })
    }
}

fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut out_vec = Vec::new();
    for line in content.lines() {
        if line.to_lowercase().contains(&query) {
            out_vec.push(line);
        }
    }
    out_vec
}

fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut out_vec = Vec::new();
    for line in content.lines() {
        if line.contains(query) {
            out_vec.push(line);
        }
    }
    out_vec
}

pub fn run(config: Config) -> Result<(), Box<dyn error::Error>> {
    let content = fs::read_to_string(config.filepath)?;
    // String implements Deref<Target = str>, so &String auto-coerces to &str everywhere a &str is expected.
    let search_fn = match config.case_insensitive {
        true => search_case_insensitive,
        false => search,
    };
    for line in search_fn(&config.query, &content) {
        println!("{line}");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
