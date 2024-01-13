use glob;
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
    let mut acc=hdc::hdv2bitarray(&hdc::Hdv::zeros());
    let mut nadd=0;
    for line in reader.lines() {
        let mut ngram = hdc::Hdv::zeros();
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
            ngram = hdc::pmultiply(&ngram, 0, bn, n-1); // forget
            let b0 = symbols.entry(c).or_insert(hdc::Hdv::new());
            block.insert(0, c);
            ngram = hdc::pmultiply(&ngram, 1, b0, 0);
            hdc::accumulate(&ngram, &mut acc);
            nadd+=1;
        }
    }
    hdc::bitarray2hdv(&acc,nadd/2)
}

fn train(n: usize) -> (HashMap<char, hdc::Hdv>, Vec<(&'static str, hdc::Hdv)>) {
    let mut symbols: HashMap<char, hdc::Hdv> = HashMap::new();
    let mut languages: Vec<(&str, hdc::Hdv)> = Vec::new();
    for (_lxx, lxxx) in LANG_MAP {
        let fname = format!("../training_texts/{lxxx}.txt");
        println!("Processing training file {fname}");
        let v = compute_sum_hv(&fname, n, &mut symbols);
        languages.push((lxxx, v));
    }
    (symbols, languages)
}

fn test(symbols: &mut HashMap<char,hdc::Hdv>, languages: &Vec<(&str,hdc::Hdv)>, n: usize) {
    let mut total = 0;
    let mut correct = 0;

    for (i, (lxx, lxxx)) in LANG_MAP.iter().enumerate() {
        println!("{i}/{}: Processing {lxx}", LANG_MAP.len());

        let pattern = format!("../testing_texts/{lxx}_*.txt");
        for fname in glob::glob(&pattern).expect("wrong glob pattern") {
            let fname = fname.unwrap();
            let v = compute_sum_hv(&fname.to_str().unwrap(), n, symbols);
            let (mut min_lang,b) = languages[0];
            let mut dmin=hdc::hamming_distance(&v,&b);
            for (lang,b) in languages.iter().skip(1) {
                let d=hdc::hamming_distance(&v,b);
                if d<dmin {
                    dmin = d;
                    min_lang=lang;
                }
            }
            if &min_lang==lxxx {
                correct+=1;
            }
            total+=1;
        }
        if total > 0 {
            println!("+{} {lxxx}: Accuracy: {correct}/{total}={}", i+1, {correct as f64/total as f64})
        }
    }
}

fn main() {
    hdc::info();
    //hdc::example_mexican_dollar();
    let n=4;
    let (mut symbols,languages) = train(n);
    test(&mut symbols,&languages,n);
    //hdc::example_bitarray();
}
