use crate::bls::FE2;
use crate::threshold::ThresholdKeyPairs;

use curv::elliptic::curves::traits::ECScalar;
use curv::BigInt;

use std::fmt;

#[derive(Debug)]
pub struct ProactiveRefresh {
    tkp: ThresholdKeyPairs,
    gammas: Vec<Vec<FE2>>,
}

impl ProactiveRefresh {
    pub fn new(_n: usize, _t: usize) -> Self {
        let mut g: Vec<Vec<FE2>> = Vec::with_capacity(_n);
        for _i in 0.._n {
            g.push(Vec::with_capacity(_n));
        }
        Self {
            tkp: ThresholdKeyPairs::new(_n, _t),
            gammas: g,
        }
    }

    pub fn threshold_keys(&self) -> &ThresholdKeyPairs {
        &self.tkp
    }

    /* update all sk part of threshold keypairs */
    pub fn refresh_all(&mut self) {
        // generate gammas
        for i in 0..self.tkp.n_keys() {
            self.gammas[i] = self.update_0();
        }
        let mut updated_keys: Vec<FE2> = Vec::new();
        for j in 0..self.tkp.n_keys() {
            updated_keys.push(self.update_1(self.tkp.get(j + 1).priv_key(), j));
        }
        for j in 0..self.tkp.n_keys() {
            self.tkp.update_secret(j + 1, updated_keys[j]);
        }
    }

    // producing gamma [i, 1], sends f_i(j) to signer j
    pub fn update_0(&self) -> Vec<FE2> {
        let mut samples: Vec<FE2> = Vec::new();
        let mut gammas: Vec<FE2> = Vec::new();

        // sample from Zp
        for _l in 1..self.tkp.t() {
            samples.push(ECScalar::new_random());
        }

        // compute gammas
        for j in 1..=self.tkp.n() {
            let mut gamma: FE2 = ECScalar::from(&BigInt::from(0));
            for l in 1..self.tkp.t() {
                let j_elem: u32 = (j as u32).pow(l as u32);
                let fe1_j: FE2 = ECScalar::from(&BigInt::from(j_elem));
                gamma = gamma + samples[l - 1] * fe1_j;
            }
            gammas.push(gamma);
        }
        return gammas;
    }

    pub fn update_1(&self, sk: FE2, j: usize) -> FE2 {
        let mut gamma_sum: FE2 = ECScalar::from(&BigInt::from(0));
        for i in 1..=self.tkp.n() {
            gamma_sum = gamma_sum + self.gammas[i - 1][j];
        }
        return sk + gamma_sum;
    }
}

impl fmt::Display for ProactiveRefresh {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.tkp)
    }
}
