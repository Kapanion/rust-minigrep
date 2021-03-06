use std::fs;
use std::error;
use std::env;

pub struct Config{
    pub query: String,
    pub file_path: String,
    pub case_sensitive: bool,
}

impl Config{
    pub fn new(mut args: env::Args) -> Result<Config, &'static str>{
        args.next();
        let query = match args.next(){
            Some(x) => x,
            None => return Err("Didn't receive the query"),
        };
        let file_path = match args.next(){
            Some(x) => x,
            None => return Err("Didn't receive the file path"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        // add: $Env:CASE_INSENSITIVE=1
        // rmv: Remove-Item Env:CASE_INSENSITIVE

        Ok(Config{query, file_path, case_sensitive})
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn error::Error>>{    
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    }else{
        search_case_insensitive(&config.query, &contents)
    };
    
    for line in results{
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
    contents.lines().filter(|line| line.contains(query)).collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
    let query = query.to_lowercase();
    let mut res: Vec<&'a str> = Vec::new();
    for line in contents.lines(){
        if line.to_lowercase().contains(&query){
            res.push(line);
        }
    }
    res
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "da";
        let contents = "\
iyo arabets rostevan, mepe gvtisagan sviani
magali, uxvi, mdabali, lashqar-mravali, ymiani,
mosamartle DA mowyale, morchmuli, gangebiani,
tvit meomari uebro, kvlav moubari wyliani";

        assert_eq!(vec!["magali, uxvi, mdabali, lashqar-mravali, ymiani,"], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rust";
        let contents = "\
RUST__??
rUsTTT!!
druSTSS
Roast
Rust";

        assert_eq!(
            vec!["RUST__??", "rUsTTT!!", "druSTSS", "Rust"],
            search_case_insensitive(query, contents)
        );
    }
}