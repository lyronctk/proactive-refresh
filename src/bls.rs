// inspired by https://github.com/ZenGo-X/multi-party-bls/blob/main/src/basic_bls.rs
#![allow(non_snake_case)]

use curv::arithmetic::Converter;
pub use curv::elliptic::curves::bls12_381::{
    g1::FE as FE1,
    g1::GE as GE1,
    g2::FE as FE2,
    g2::GE as GE2,
    Pair,
};
pub use curv::elliptic::curves::traits::{ECPoint, ECScalar};

use std::ops::Add;
use std::fmt;

#[derive(Clone, Copy, Debug)]
pub struct KeyPairG2 {
    pub X: GE2,
    pub x: FE2,
}

#[derive(Debug)]
pub struct BLSSignature {
    pub sigma: GE1,
}

impl KeyPairG2 {
    pub fn new() -> Self {
        let x: FE2 = ECScalar::new_random();
        let X: GE2 = GE2::generator() * &x;
        KeyPairG2 { x, X }
    }
}

impl fmt::Display for KeyPairG2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x{}", self.x.to_big_int().to_str_radix(16))
    }
}

impl Add for KeyPairG2 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            X: self.X + other.X,
        }
    }
}

impl BLSSignature {
    pub fn sign(message: &[u8], x: &FE2) -> Self {
        let H_m: GE1 = GE1::hash_to_curve(message);
        let fe1_x: FE1 = ECScalar::from(&ECScalar::to_big_int(x));
        BLSSignature { sigma: H_m * fe1_x }
    }

    pub fn verify(&self, message: &[u8], X: &GE2) -> bool {
        let H_m: GE1 = GE1::hash_to_curve(message);
        let lhs: Pair = Pair::compute_pairing(&H_m, X);
        let rhs: Pair = Pair::compute_pairing(&self.sigma, &GE2::generator());
        lhs == rhs
    }
}
