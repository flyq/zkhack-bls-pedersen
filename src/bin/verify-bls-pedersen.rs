use ark_bls12_381::{Fr, G1Affine};
use ark_ec::{AffineCurve, ProjectiveCurve};
use bls_pedersen::data::puzzle_data;
use bls_pedersen::PUZZLE_DESCRIPTION;
use bls_pedersen::{bls::verify, hash, matrix::a_inv};
use num_traits::identities::Zero;
use prompt::{puzzle, welcome};

fn main() {
    welcome();
    puzzle(PUZZLE_DESCRIPTION);

    let (pk, ms, sigs) = puzzle_data();
    for (m, sig) in ms.iter().zip(sigs.iter()) {
        verify(pk, m, *sig);
    }

    #[allow(unused_variables)]
    let h_m: Vec<Vec<u8>> = ms
        .iter()
        .map(|m| hash::hash_to_curve(m).0)
        .map(|i| u8_to_bits(&i))
        .collect();
    // println!("{:?}", h_m);
    // for i in 0..sigs.len() {
    //     println!("{}", sigs[i]);
    // }

    // let sig =
    /* Your solution here! */
    /*
      let sig = ...;
      let m = your username;
      verify(pk, m, sig);
    */

    let a_inv = a_inv();
    let mut q: Vec<G1Affine> = Vec::new();
    for raw in a_inv {
        let mut sum = G1Affine::zero();
        for i in 0..256 {
            sum = sum + sigs[i].mul(raw[i]).into_affine();
        }
        q.push(sum)
    }
    let msg = b"Rob";
    let hm = hash::hash_to_curve(msg).0;
    let hm = u8_to_bits(&hm);

    let mut sig = G1Affine::zero();
    for i in 0..256 {
        sig = sig + q[i].mul(Fr::from(hm[i])).into_affine();
    }

    verify(pk, msg, sig);
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
