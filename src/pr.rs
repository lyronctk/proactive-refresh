/*
 * Proactive refresh. Updates all secret keys in a threshold scheme while 
 * leaving collective public keys unchanged. Based on the paper 
 * "Proactive Refresh for Accountable Threshold Signatures" by D Boneh, 
 * A Partap, and L Rotem. 
 */
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
    /*
     * Instantiates new threshold signature scheme with functionality for 
     * proactive refresh. 
     */
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

    /*
     * Getter for all the key pairs. 
     */
    pub fn threshold_keys(&self) -> &ThresholdKeyPairs {
        &self.tkp
    }

    /*
     * Updates secret keys with one party's contribution. Note that a realistic 
     * implementation of would do this step for every party since everyone 
     * must contribute. 
     */
    pub fn refresh_all(&mut self) {
        for i in 0..self.tkp.n() {
            self.gammas[i] = self.update_0();
        }
        let mut updated_keys: Vec<FE2> = Vec::new();
        for j in 0..self.tkp.n() {
            updated_keys.push(self.update_1(self.tkp.get(j).priv_key(), j));
        }
        for j in 0..self.tkp.n() {
            self.tkp.update_secret(j, updated_keys[j]);
        }
    }

    /*
     * Implements update_0 for one party as defined in the paper.
     */
    fn update_0(&self) -> Vec<FE2> {
        let mut samples: Vec<FE2> = Vec::new();
        let mut gammas: Vec<FE2> = Vec::new();

        // Coefficients of the new polynomal f
        for _l in 1..self.tkp.t() {
            samples.push(ECScalar::new_random());
        }

        // Compute {f(j)} and send to all signers  
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

    /*
     * Implements update_1 for party j as defined in the paper. 
     */
    fn update_1(&self, sk: FE2, j: usize) -> FE2 {
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
