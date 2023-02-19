use crate::ats_pr::bls::{BLSSignature, ECPoint, ECScalar, KeyPairG2, FE1, FE2, GE1, GE2};
use crate::ats_pr::lagrange::{
    lagrange_interpolate_f0_X, lagrange_interpolate_f0_sig, lagrange_interpolate_f0_x,
};

#[derive(Debug)]
pub struct ThresholdKeyPairs {
    pub keys: Vec<KeyPairG2>,
    n: usize,
    t: usize,
}

#[derive(Debug)]
pub struct ThresholdSignature {
    sigma: GE1,
}

impl ThresholdKeyPairs {
    pub fn new(_n: usize, _t: usize) -> Self {
        Self {
            keys: vec![KeyPairG2::new(); _n],
            n: _n,
            t: _t,
        }
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
        ThresholdSignature { sigma: sigma }
    }
}
