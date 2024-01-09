use rand::Rng;

const N_BITS: usize = 100000;
pub const VEC_SIZE: usize = vec_size(N_BITS);

pub fn multiply<const N: usize>(a1: &[usize; N], a2: &[usize; N]) -> [usize; N] {
    let mut result: [usize; N] = [0; N];

    for i in 0..N {
        result[i] = a1[i] ^ a2[i];
    }

    result
}

pub fn hamming_distance(a: &[usize], b: &[usize]) -> usize {
    assert_eq!(a.len(), b.len(), "Arrays must have the same length");

    a.iter()
        .zip(b.iter())
        .map(|(&x, &y)| (x ^ y).count_ones() as usize)
        .sum()
}

pub fn add<const N: usize>(arrays: &[&[usize; N]]) -> [usize; N] {
    assert!(!arrays.is_empty(), "Arrays slice must not be empty");

    let mut result = [0; N];

    for i in 0..usize::BITS {
        for j in 0..N {
            let ones: usize = arrays.iter().fold(0, |acc, a| acc + ((a[j] >> i) & 1));

            if ones > arrays.len() / 2 {
                result[j] |= 1 << i;
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

pub fn init_random_array<const N: usize>() -> [usize; N] {
    assert!(N % 2 == 0, "N must be even");
    let mut rng = rand::thread_rng();

    let mut array: [usize; N] = [0; N];

    for i in 0..N / 2 {
        array[i] = rng.gen_range(0..=usize::MAX);
        array[i + N / 2] = array[i];
    }

    array
}

fn example_mexican_dollar() {
    // Pentti Kanerva: What We Mean When We Say “What’s the Dollar of Mexico?”
    // https://redwood.berkeley.edu/wp-content/uploads/2020/05/kanerva2010what.pdf
    // Calculate answer: Mexican Peso - mpe
    let name: [usize; VEC_SIZE] = init_random_array();
    let capital: [usize; VEC_SIZE] = init_random_array();
    let currency: [usize; VEC_SIZE] = init_random_array();

    let swe: [usize; VEC_SIZE] = init_random_array();
    let usa: [usize; VEC_SIZE] = init_random_array();
    let mex: [usize; VEC_SIZE] = init_random_array();

    let stockholm: [usize; VEC_SIZE] = init_random_array();
    let wdc: [usize; VEC_SIZE] = init_random_array();
    let cdmx: [usize; VEC_SIZE] = init_random_array();

    let usd: [usize; VEC_SIZE] = init_random_array();
    let mpe: [usize; VEC_SIZE] = init_random_array();
    let skr: [usize; VEC_SIZE] = init_random_array();

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

fn main() {
    let size_of_usize = std::mem::size_of::<usize>();
    println!("usize occupies {} bytes", size_of_usize);
    println!("vec_size for 10k bits: {VEC_SIZE} ");
    println!("actual bits: {} ", VEC_SIZE * size_of_usize * 8);

    example_mexican_dollar();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn mexican_dollar() {
        example_mexican_dollar()
    }
}
