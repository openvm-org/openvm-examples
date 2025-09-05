// src/main.rs
extern crate alloc;

use openvm::io::{read, reveal_bytes32};
use openvm_algebra_guest::{moduli_macros::*, IntMod};
use openvm_sha2::sha256;

moduli_declare! {
    Mod { modulus = "23" },
}

openvm::init!();

pub fn main() {
    let alice_secret = read();
    let bob_secret = read();
    let base = Mod::from_u32(5);

    let mut A = Mod::from_u32(1);
    for _ in 0..alice_secret {
        A *= &base;
    }

    let mut B = Mod::from_u32(1);
    for _ in 0..bob_secret {
        B *= &base;
    }

    let mut shared_secret_alice = Mod::from_u32(1);
    for _ in 0..alice_secret {
        shared_secret_alice *= &B;
    }

    let mut shared_secret_bob = Mod::from_u32(1);
    for _ in 0..bob_secret {
        shared_secret_bob *= &A;
    }

    assert_eq!(shared_secret_alice, shared_secret_bob);

    let mut shared_secret: u8 = 0;
    for i in 0..23 {
        if shared_secret_alice == Mod::from_u32(i) {
            shared_secret = i as u8;
            break;
        }
    }

    println!("shared_secret: {:?}", shared_secret);

    let hashed_shared_secret = sha256(&[shared_secret]);
    reveal_bytes32(hashed_shared_secret);
}
