use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub is_ignore_case: bool,
}

impl Config {
    pub fn build(args: &Vec<String>) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not Enough Arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        let is_ignore_case = env::var("IGNORE_CASE").is_ok();
        dbg!(is_ignore_case);

        Ok(Config {
            query,
            file_path,
            is_ignore_case,
        })
    }
}

pub fn search_sensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    //iterate through the content
    let mut result: Vec<&str> = Vec::new();
    for line in content.lines() {
        //search is query is present in content
        if line.contains(query) {
            //if present -> add to the vector
            result.push(line);
        }
    }
    //if not then do nothing
    //return vector
    result
}

pub fn search_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    let query = query.to_lowercase();

    for line in content.lines() {
        if line.to_lowercase().contains(&query) {
            result.push(line.trim());
        }
    }
    result
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    //read and store the content of the file of the provided file path
    let content = fs::read_to_string(config.file_path)?;
    let result = if config.is_ignore_case {
        search_insensitive(&config.query, &content)
    } else {
        search_sensitive(&config.query, &content)
    };

    if result.len() <= 0 {
        println!("No line found !");
    } else {
        for line in result {
            println!("{}\n", line);
        }
    }
    Ok(())
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_search_sensitive() {
        let query = "consensus";
        let content = "ess called consensus.
         Blockchain protocols are mechanisms that enable,
         protocols like p2p
         ";

        assert_eq!(
            vec!["ess called consensus."],
            search_sensitive(query, content)
        )
    }
    #[test]
    fn test_search_insensitive() {
        let query: &str = "P2P";
        let content: &str = "ess called consensus.
         Blockchain protocols are mechanisms that enable,
         protocols like p2p";
        assert_eq!(
            vec!["protocols like p2p"],
            search_insensitive(query, content)
        );
    }
}
