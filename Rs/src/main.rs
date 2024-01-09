use std::collections::hash_map::{Entry, HashMap};
use std::fs::File;
use std::io::{self, BufRead};

pub mod hdc;

const LANG_MAP: [(&str, &str); 22] = [
    ("af", "afr"),
    ("bg", "bul"),
    ("cs", "ces"),
    ("da", "dan"),
    ("de", "deu"),
    ("el", "ell"),
    ("en", "eng"),
    ("es", "spa"),
    ("et", "est"),
    ("fi", "fin"),
    ("fr", "fra"),
    ("hu", "hun"),
    ("it", "ita"),
    ("lt", "lit"),
    ("lv", "lav"),
    ("nl", "nld"),
    ("pl", "pol"),
    ("pt", "por"),
    ("ro", "ron"),
    ("sk", "slk"),
    ("sl", "slv"),
    ("sv", "swe"),
];

fn compute_sum_hv(fname: &str, n: usize, symbols: &mut HashMap<char,[usize;hdc::VEC_SIZE]>) -> [usize; hdc::VEC_SIZE] {
    let file = File::open(fname).unwrap();
    let reader = io::BufReader::new(file);
    for line in reader.lines() {
        let line = line.unwrap();
        for c in line.chars().take(n) {
            print!("_{c}");
        }
        for c in line.chars().skip(n) {
            print!("-{c}");
        }
        println!();
    }
    hdc::init_random_array()
}

fn train(n: usize) {
    let mut symbols: HashMap<char, [usize; hdc::VEC_SIZE]> = HashMap::new();
    let mut languages: Vec<(&str,[usize;hdc::VEC_SIZE])> = Vec::new();
    for (lxx, lxxx) in LANG_MAP {
        let fname = format!("../training_texts/{lxxx}.txt");
        println!("Processing training file {fname}");
        let v=compute_sum_hv(&fname, n, &mut symbols);
        languages.push((lxxx,v));
    }
}

fn main() {
    train(2);
}
