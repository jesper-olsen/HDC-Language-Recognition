// Copyright (c) 2024 Jesper Olsen
//
//References:
//
//[1] "What We Mean When We Say 'What’s the Dollar of Mexico?': Prototypes and Mapping in Concept Space" by Pentti Kanerva, Quantum Informatics for Cognitive, Social, and Semantic Processes: Papers from the AAAI Fall Symposium (FS-10-08), 2010
//[2] "Language Geometry using Random Indexing" Aditya Joshi1, Johan T. Halseth2, and Pentti Kanerva, 2016
//[3] "A Robust and Energy-Efficient Classifier Using Brain-Inspired Hyperdimensional Computing" Abbas Rahimi, Pentti Kanerva, Jan M. Rabaey, 2016
//[4] "Hyperdimensional Computing: An Algebra for Computing with Vectors", Pentti Kanerva, 2022

use rand::{random, Rng};
use std::cmp::Ordering;

const N_BITS: usize = 1000000;
const SIZE_OF_USIZE: usize = std::mem::size_of::<usize>();
const VEC_SIZE: usize = vec_size(N_BITS);
const N_HDV_BITS: usize = VEC_SIZE * SIZE_OF_USIZE * 8;

const fn vec_size(n_bits: usize) -> usize {
    let bits_per_usize = 8 * std::mem::size_of::<usize>();
    let n = if n_bits / bits_per_usize == 0 {
        n_bits / bits_per_usize
    } else {
        n_bits / bits_per_usize + 1
    };
    if n % 2 == 0 {
        n
    } else {
        n + 1
    }
}

fn pct(n: u8) -> bool {
    random::<u8>() % 100 < n
}

#[derive(Debug)]
pub struct Hdv {
    //v: [usize; VEC_SIZE],
    v: Vec<usize>,
}

pub fn info() {
    let size_of_usize = std::mem::size_of::<usize>();
    println!("usize occupies {} bytes", size_of_usize);
    println!("vec_size for {N_BITS} bits: {VEC_SIZE} ");
    println!("actual bits: {} ", VEC_SIZE * size_of_usize * 8);
}

pub fn accumulate(hdv: &Hdv, a: &mut [usize]) {
    for (i, &x) in hdv.v.iter().enumerate() {
        for j in 0..SIZE_OF_USIZE * 8 {
            a[i * SIZE_OF_USIZE * 8 + j] += (x >> j) & 1;
        }
    }
}

pub fn hdv2bitarray(hdv: &Hdv) -> Vec<usize> {
    let mut a = Vec::with_capacity(N_HDV_BITS);

    for x in hdv.v.iter() {
        for j in 0..SIZE_OF_USIZE * 8 {
            a.push((x >> j) & 1);
        }
    }
    a
}

pub fn bitarray2hdv(a: &[usize], thr: usize) -> Hdv {
    let mut hdv = Hdv::zeros();

    for (i, &b) in a.iter().enumerate() {
        let idx = i / (SIZE_OF_USIZE * 8);
        if b > thr {
            hdv.v[idx] |= 1 << (i % (SIZE_OF_USIZE * 8));
        }
    }
    hdv
}

impl Hdv {
    pub fn new() -> Self {
        assert!(VEC_SIZE % 2 == 0, "VEC_SIZE must be even");

        let mut rng = rand::thread_rng();
        //let mut v = Vec::with_capacity(VEC_SIZE);
        //for _ in 0..VEC_SIZE/ 2 {
        //    let random_value = rng.gen_range(0..=usize::MAX);
        //    v.push(random_value);
        //    v.push(!random_value & usize::MAX); // Flip the bits to ensure balance
        //}
        //v.shuffle(&mut rng);

        let v = (0..VEC_SIZE)
            .map(|_| rng.gen_range(0..=usize::MAX))
            .collect();
        Hdv { v  }
    }

    pub fn zeros() -> Self {
        //Hdv { v: [0; VEC_SIZE] }
        Hdv {
            v: vec![0; VEC_SIZE],
        }
    }
}

pub fn multiply(a: &Hdv, b: &Hdv) -> Hdv {
    let mut result = Hdv::zeros();

    for i in 0..VEC_SIZE {
        result.v[i] = a.v[i] ^ b.v[i];
    }

    result
}

pub fn pmultiply(a: &Hdv, pa: usize, b: &Hdv, pb: usize) -> Hdv {
    //permute and multiply
    let mut result = Hdv::zeros();

    for i in 0..VEC_SIZE {
        result.v[i] = a.v[(i + pa) % VEC_SIZE] ^ b.v[(i + pb) % VEC_SIZE];
    }

    result
}

pub fn hamming_distance(a: &Hdv, b: &Hdv) -> usize {
    assert_eq!(a.v.len(), b.v.len(), "Arrays must have the same length");

    a.v.iter()
        .zip(b.v.iter())
        .map(|(&x, &y)| (x ^ y).count_ones() as usize)
        .sum()
}

pub fn add(arrays: &[&Hdv]) -> Hdv {
    assert!(!arrays.is_empty(), "Arrays slice must not be empty");

    let mut result = Hdv::zeros();

    for i in 0..usize::BITS {
        for j in 0..VEC_SIZE {
            let ones: usize = arrays.iter().fold(0, |acc, a| acc + ((a.v[j] >> i) & 1));

            //match ones.cmp(&(arrays.len() / 2)) {
            //    Ordering::Less => result.v[j] |= 1 << i,
            //    Ordering::Equal if pct(50) => result.v[j] |= 1 << i,
            //    Ordering::Equal => (),
            //    Ordering::Greater => (),
            //}

            if ones > arrays.len() / 2 {
                result.v[j] |= 1 << i;
            }
        }
    }

    result
}

pub fn add2(arrays: &[Hdv]) -> Hdv {
    assert!(!arrays.is_empty(), "Arrays slice must not be empty");

    let mut result = Hdv::zeros();

    for i in 0..usize::BITS {
        for j in 0..VEC_SIZE {
            let ones: usize = arrays.iter().fold(0, |acc, a| acc + ((a.v[j] >> i) & 1));

            if ones > arrays.len() / 2 {
                result.v[j] |= 1 << i;
            }
        }
    }

    result
}

pub fn example_bitarray() {
    let mut hdv = Hdv::zeros();
    hdv.v[0] = 1;
    hdv.v[1] = 2;
    hdv.v[2] = 255;

    println!("    v: {:?}", hdv.v);
    let a = hdv2bitarray(&hdv);
    println!("    a: {:?}", a);
    let h = bitarray2hdv(&a, 0);
    println!("    h: {:?}", h);

    hdv.v[3] = 1;
    let mut a = hdv2bitarray(&hdv);
    accumulate(&hdv, &mut a);

    let h = bitarray2hdv(&a, 0);
    println!("acc h: {:?}", h);
}

pub fn example_mexican_dollar() {
    // Pentti Kanerva: What We Mean When We Say “What’s the Dollar of Mexico?”
    // https://redwood.berkeley.edu/wp-content/uploads/2020/05/kanerva2010what.pdf
    // Calculate answer: Mexican Peso - mpe
    let name = Hdv::new();
    let capital = Hdv::new();
    let currency = Hdv::new();

    let swe = Hdv::new();
    let usa = Hdv::new();
    let mex = Hdv::new();

    let stockholm = Hdv::new();
    let wdc = Hdv::new();
    let cdmx = Hdv::new();

    let usd = Hdv::new();
    let mpe = Hdv::new();
    let skr = Hdv::new();

    let mut a = hdv2bitarray(&Hdv::zeros());
    accumulate(&multiply(&name, &usa), &mut a);
    accumulate(&multiply(&capital, &wdc), &mut a);
    accumulate(&multiply(&currency, &usd), &mut a);
    let ustates = bitarray2hdv(&a, 3 / 2);

    //let ustates = add(&[
    //    &multiply(&name, &usa),
    //    &multiply(&capital, &wdc),
    //    &multiply(&currency, &usd),
    //]);

    let _sweden = add(&[
        &multiply(&name, &swe),
        &multiply(&capital, &stockholm),
        &multiply(&currency, &skr),
    ]);

    let mut a = hdv2bitarray(&Hdv::zeros());
    accumulate(&multiply(&name, &mex), &mut a);
    accumulate(&multiply(&capital, &cdmx), &mut a);
    accumulate(&multiply(&currency, &mpe), &mut a);
    let mexico = bitarray2hdv(&a, 3 / 2);

    //let mexico = add(&[
    //    &multiply(&name, &mex),
    //    &multiply(&capital, &cdmx),
    //    &multiply(&currency, &mpe),
    //]);

    let fmu = multiply(&mexico, &ustates);
    let x = multiply(&fmu, &usd);

    let vocab = [
        ("swe", swe),
        ("usa", usa),
        ("mex", mex),
        ("stockholm", stockholm),
        ("wdc", wdc),
        ("cdmx", cdmx),
        ("usd", usd),
        ("mpe", mpe),
        ("skr", skr),
    ];
    let mut ml = vocab[0].0;
    let mut md = hamming_distance(&x, &vocab[0].1);
    for (label, v) in vocab.iter().skip(1) {
        let d = hamming_distance(&x, v);
        println!("{label} {d:?}");
        if d < md {
            md = d;
            ml = label;
        }
    }
    println!("Min is: {ml}");
    assert_eq!(ml, "mpe", "Expected mpe");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn mexican_dollar() {
        example_mexican_dollar()
    }
}
