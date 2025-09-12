// Final polynomial evaluation with SHA256 using JSON input
extern crate alloc;
use alloc::vec::Vec;
use openvm as _;

use core::hint::black_box;
use openvm_algebra_guest::{moduli_macros::*, IntMod};
use openvm_sha2::sha256;

moduli_declare! {
    Mod1 { modulus = "998244353" }
}

use openvm::io::{read, reveal_u32};

openvm::init!();

fn eval_polynomial(coeffs: Vec<Mod1>, x: Mod1) -> (Mod1, Vec<Mod1>) {
    let mut result = Mod1::ZERO;
    let mut steps: Vec<Mod1> = Vec::new();

    for coeff in coeffs.iter().rev() {
        result = result * x.clone() + coeff.clone();
        steps.push(result.clone());
    }

    (result, steps)
}

fn main() {
    let n: u32 = read();
    let mut coeffs: Vec<Mod1> = Vec::new();

    for _ in 0..n {
        let coeff_val: u32 = read();
        coeffs.push(Mod1::from_u32(coeff_val));
    }

    let x_val: u32 = read();
    let x = Mod1::from_u32(x_val);

    let (result, steps) = eval_polynomial(coeffs, x);

    let result_bytes = result.0;
    let result_val = u32::from_le_bytes([
        result_bytes[0],
        result_bytes[1],
        result_bytes[2],
        result_bytes[3],
    ]);

    reveal_u32(result_val, 0); // 4 at a time

    for (i, step) in steps.iter().enumerate() {
        let step_bytes = step.0;
        let step_val =
            u32::from_le_bytes([step_bytes[0], step_bytes[1], step_bytes[2], step_bytes[3]]); //convert into u32 from little endian, from underlying bytes of Mod1
        let hash = sha256(&black_box(step.0));

        reveal_u32(step_val, 2 * i + 1);
        let hash_val = u32::from_le_bytes([hash[0], hash[1], hash[2], hash[3]]);
        reveal_u32(hash_val, 2 * i + 2);
    }
}
