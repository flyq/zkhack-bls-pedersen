use bls_pedersen::data::puzzle_data;
use bls_pedersen::PUZZLE_DESCRIPTION;
use bls_pedersen::{bls::verify, hash};
use prompt::{puzzle, welcome};

fn main() {
    welcome();
    puzzle(PUZZLE_DESCRIPTION);
    println!("{:?}", u8_to_bits(&[5, 15]));
    let (pk, ms, sigs) = puzzle_data();
    for (m, sig) in ms.iter().zip(sigs.iter()) {
        verify(pk, m, *sig);
    }

    // let a = matrix![2., 3.;
    //             1., 2.];
    // let inv = a
    //     .clone()
    //     .inverse()
    //     .expect("This matrix should have an inverse!");

    // let I = a * inv;

    /* Your solution here! */
    /*
      let sig = ...;
      let m = your username;
      verify(pk, m, sig);
    */

    let h_m: Vec<Vec<u8>> = ms
        .iter()
        .map(|m| hash::hash_to_curve(m).0)
        // .map(|i| u8_to_bits(&i))
        .collect();

    // let sig =
    // let m = b"Rob".to_vec();
}

fn u8_to_bits(inputs: &[u8]) -> Vec<u8> {
    let mut bits = Vec::with_capacity(inputs.len() * 8);

    for input in inputs {
        for i in 0..8 {
            let bit = (input >> i) & 1;
            bits.push(bit);
        }
    }
    bits
}
