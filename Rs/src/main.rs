use std::collections::hash_map::{Entry, HashMap};
use std::fs::File;
use std::io::{self, BufRead};

pub mod hdc;
pub mod hdc0;

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

//def computeSumHV(fname, letters, N, D):
//    sum_hv = np.zeros(D)
//    for line in open(fname):
//        block = np.ones((N, D))
//        ngram = np.ones(D)
//        for c in line[:N]:
//            block = np.roll(block, shift=(1, 1), axis=(0, 1))
//            block[0] = letters.setdefault(c, init_hv(D))
//            ngram = np.multiply(np.roll(ngram, shift=1), block[0])  # Hadamard
//
//        for c in line[N:]:
//            ngram = np.multiply(ngram, block[N - 1])  # forget - Hadamard
//            block = np.roll(block, shift=(1, 1), axis=(0, 1))
//            block[0] = letters.setdefault(c, init_hv(D))
//            ngram = np.multiply(np.roll(ngram, shift=1), block[0])  # Hadamard
//            sum_hv += ngram
//    return sum_hv

fn compute_sum_hv(fname: &str, n: usize, symbols: &mut HashMap<char, hdc::Hdv>) -> hdc::Hdv {
    let file = File::open(fname).unwrap();
    let reader = io::BufReader::new(file);
    let mut sum_hv = hdc::Hdv::zeros();
    let mut ngram = hdc::Hdv::zeros();
    let mut l = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let mut block: Vec<char> = Vec::new();
        for c in line.chars().take(n) {
            let b0 = symbols.entry(c).or_insert(hdc::Hdv::new());
            block.insert(0, c);
            ngram = hdc::pmultiply(&ngram, 1, b0, 0);
        }
        for c in line.chars().skip(n) {
            let cn = block.pop().unwrap();
            let bn = symbols.get(&cn).unwrap();
            ngram = hdc::pmultiply(&ngram, 0, bn, n); // forget
            let b0 = symbols.entry(c).or_insert(hdc::Hdv::new());
            block.insert(0, c);
            ngram = hdc::pmultiply(&ngram, 1, b0, 0);
            l.push(ngram);
            //sum_hv=hdc::add(&[&sumHV,&ngram]);
            //            ngram = np.multiply(ngram, block[N - 1])  # forget - Hadamard
            //            block = np.roll(block, shift=(1, 1), axis=(0, 1))
            //            block[0] = letters.setdefault(c, init_hv(D))
            //            ngram = np.multiply(np.roll(ngram, shift=1), block[0])  # Hadamard
            //            sum_hv += ngram
        }
    }
    hdc::add2(&l)
}

fn train(n: usize) {
    let mut symbols: HashMap<char, hdc::Hdv> = HashMap::new();
    let mut languages: Vec<(&str, hdc::Hdv)> = Vec::new();
    for (lxx, lxxx) in LANG_MAP {
        let fname = format!("../training_texts/{lxxx}.txt");
        println!("Processing training file {fname}");
        let v = compute_sum_hv(&fname, n, &mut symbols);
        languages.push((lxxx, v));
    }
}

fn main() {
    hdc::info();
    hdc::example_mexican_dollar();
    train(2);
}
