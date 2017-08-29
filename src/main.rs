extern crate regex;
use regex::Regex;

use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let filename = String::from("doc.txt");
    let mut contents = String::new();
    // let mut map = HashMap::new();
    let mut f = File::open(filename).expect("file not found");
    
    f.read_to_string(&mut contents).expect("error reading file");

    // 1. remove punct
    let no_punct = rm_punct(contents);

    // 2. to lowercase
    let lc = no_punct.to_lowercase();

    let split = lc.split_whitespace();

    let map = create_map(split);

    let mut count_vec: Vec<_> = map.iter().collect();
    count_vec.sort_by(|a, b| b.1.cmp(a.1));

    for (word, cnt) in count_vec {
        println!("{} - {}", word, cnt);
    }
}

fn create_map(s: std::str::SplitWhitespace) -> HashMap<&str, i32> {
    let mut map = HashMap::new();
    for word in s {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    map
}

fn rm_punct(t: String) -> String {
    let re = Regex::new(r"[[:punct:][:cntrl:]]").unwrap();
    let result = re.replace_all(&*t, "");
    String::from(result)
}