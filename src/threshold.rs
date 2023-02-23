#![allow(non_snake_case)]

use crate::bls::{BLSSignature, KeyPairG2, FE2, GE1, GE2};
use crate::lagrange::{
    lagrange_interpolate_f0_X, lagrange_interpolate_f0_sig, lagrange_interpolate_f0_x,
};

use std::fmt;

#[derive(Debug)]
pub struct ThresholdKeyPairs {
    pub keys: Vec<KeyPairG2>,
    pub n: usize,
    pub t: usize,
}

#[derive(Debug)]
pub struct ThresholdSignature {
    sig: BLSSignature,
    pub quorum: Vec<usize>
}

impl ThresholdKeyPairs {
    pub fn new(_n: usize, _t: usize) -> Self {
        let mut k: Vec<KeyPairG2> = Vec::new();
        for _ in 0.._n {
            k.push(KeyPairG2::new());
        }
        Self {
            keys: k,
            n: _n,
            t: _t,
        }
    }

    pub fn from(ks: Vec<KeyPairG2>, _t: usize) -> Self {
        Self {
            n: ks.len(),
            t: _t,
            keys: ks, 
        }
    }

    pub fn getParty(&self, idx: usize) -> KeyPairG2 {
        if idx - 1 >= self.keys.len() {
            panic!("Tried to access key at idx > n");
        }
        self.keys[idx - 1]
    }

    fn get_quorum_keys(&self, quorum: &Vec<usize>) -> Vec<&KeyPairG2> {
        let mut q: Vec<&KeyPairG2> = Vec::new();
        for idx in quorum {
            if *idx >= self.keys.len() {
                panic!("Quorum indices included party outside of available keys");
            }
            q.push(&self.keys[*idx]);
        }
        q
    }

    // [TODO] Convert functions below for generics that accept GE2 or FE2
    pub fn get_X(&self, quorum: &Vec<usize>) -> Vec<GE2> {
        self.get_quorum_keys(quorum)
            .into_iter()
            .map(|key: &KeyPairG2| key.X)
            .collect()
    }

    pub fn get_x(&self, quorum: &Vec<usize>) -> Vec<FE2> {
        self.get_quorum_keys(quorum)
            .into_iter()
            .map(|key: &KeyPairG2| key.x)
            .collect()
    }

    pub fn quorum_X(&self, quorum: &Vec<usize>) -> GE2 {
        lagrange_interpolate_f0_X(
            &quorum
                .into_iter()
                .map(|idx: &usize| idx + 1)
                .zip(self.get_X(&quorum).into_iter())
                .collect(),
        )
    }

    pub fn quorum_x(&self, quorum: &Vec<usize>) -> FE2 {
        lagrange_interpolate_f0_x(
            &quorum
                .into_iter()
                .map(|idx: &usize| idx + 1)
                .zip(self.get_x(&quorum).into_iter())
                .collect(),
        )
    }
}

impl fmt::Display for ThresholdKeyPairs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Private keys for {}-of-{}\n", self.t, self.n)?;
        for (i, k) in self.keys.iter().enumerate() {
            write!(f, "  [{}] {}", i.to_string(), k)?;
            if i != self.keys.len() - 1 {
                write!(f, "\n")?;
            }
        }
        Ok(())
    }
}

impl ThresholdSignature {
    pub fn sign(message: &[u8], tkps: &ThresholdKeyPairs, quorum: &Vec<usize>) -> Self {
        let mut sigmas: Vec<GE1> = Vec::new();
        for x in tkps.get_x(quorum) {
            sigmas.push(BLSSignature::sign(message, &x).sigma);
        }
        let sigma = lagrange_interpolate_f0_sig(
            &quorum
                .into_iter()
                .map(|idx: &usize| idx + 1)
                .zip(sigmas.into_iter())
                .collect(),
        );
        ThresholdSignature { sig: BLSSignature { sigma: sigma }, quorum: quorum.clone() }
    }

    pub fn verify(&self, message: &[u8], tkps: &ThresholdKeyPairs) -> bool {
        if self.quorum.len() < tkps.t {
            println!("- Verification failed. Quorum has fewer than t participants.");
            return false;
        }
        let X: GE2 = tkps.quorum_X(&self.quorum);
        return self.sig.verify(message, &X);
    }
}

impl fmt::Display for ThresholdSignature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(sigma: {}, quorum: {:?})", self.sig, self.quorum)
    }
}
