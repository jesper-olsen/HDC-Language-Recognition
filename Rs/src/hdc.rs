//use rand::Rng;
use rand::{random, Rng};
use std::cmp::Ordering;

const N_BITS: usize = 100000;
const VEC_SIZE: usize = vec_size(N_BITS);

fn pct(n: u8) -> bool {
    random::<u8>() % 100 < n
}

#[derive(Debug, Clone, Copy)]
pub struct Hdv {
    v: [usize; VEC_SIZE],
}

pub fn info() {
    let size_of_usize = std::mem::size_of::<usize>();
    println!("usize occupies {} bytes", size_of_usize);
    println!("vec_size for 10k bits: {VEC_SIZE} ");
    println!("actual bits: {} ", VEC_SIZE * size_of_usize * 8);
}

impl Hdv {
    pub fn new() -> Self {
        assert!(VEC_SIZE % 2 == 0, "VEC_SIZE must be even");
        let mut rng = rand::thread_rng();
        let mut array: [usize; VEC_SIZE] = [0; VEC_SIZE];
        for i in 0..VEC_SIZE / 2 {
            array[i] = rng.gen_range(0..=usize::MAX);
            array[i + VEC_SIZE / 2] = array[i];
        }
        //TODO - shuffle

        Hdv { v: array }
    }

    pub fn zeros() -> Self {
        Hdv {
            v: [0; VEC_SIZE],
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
            let ones: usize = arrays
                .iter()
                .fold(0, |acc, a| acc + ((a.v[j] >> i) & 1));

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
            let ones: usize = arrays
                .iter()
                .fold(0, |acc, a| acc + ((a.v[j] >> i) & 1));

            if ones > arrays.len() / 2 {
                result.v[j] |= 1 << i;
            }
        }
    }

    result
}

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

    let ustates = add(&[
        &multiply(&name, &usa),
        &multiply(&capital, &wdc),
        &multiply(&currency, &usd),
    ]);

    let _sweden = add(&[
        &multiply(&name, &swe),
        &multiply(&capital, &stockholm),
        &multiply(&currency, &skr),
    ]);

    let mexico = add(&[
        &multiply(&name, &mex),
        &multiply(&capital, &cdmx),
        &multiply(&currency, &mpe),
    ]);

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
