use crate::ats_pr::bls::FE1;
use crate::ats_pr::bls::FE2;

use curv::elliptic::curves::traits::ECScalar;
use curv::BigInt;

use super::threshold::ThresholdKeyPairs;

#[derive(Debug)]
pub struct ProactiveRefresh {
    pub tkp: ThresholdKeyPairs
}

/* todo: ask lyron about FE1 and ECScalar types, private . */

impl ProactiveRefresh {
    pub fn new(_n: usize, _t: usize) -> Self {
        Self {
            tkp: ThresholdKeyPairs::new(_n, _t)
        }
    }

    /* update all sk part of threshold keypairs */
    pub fn update_all(&mut self) {
        for i in 0..self.tkp.keys.len() {
            let old_sk = self.tkp.keys[i].x; 
            self.tkp.keys[i].x = self.update_one(old_sk);
        }
    }

    // producing gamma [i, 1], sends f_i(j) to signer j
    pub fn update_one(&self, sk: FE2) -> FE2 {
        let t: usize = self.tkp.t;
        let _n: usize = self.tkp.n + 1;

        let mut samples: Vec<FE2> = Vec::new();
        let mut gammas: Vec<FE2> = Vec::new();

        // sample from Zp
        for _l in 1..t {
            samples.push(ECScalar::new_random());
        }

        // compute gammas and gamma sum
        let mut gamma_sum: FE2 = ECScalar::from(&BigInt::from(0)); 
        for j in 1.._n {
            let mut gamma: FE2 = ECScalar::from(&BigInt::from(0));
            for l in 1..t {
                let j_elem: u32 = (j as u32).pow(l as u32);
                let fe1_j: FE2 = ECScalar::from(&BigInt::from(j_elem));
                gamma = gamma + samples[l - 1] * fe1_j;
            }
            gammas.push(gamma); 
            gamma_sum = gamma_sum + gamma;
        }
        
        // update secret keys
        return sk + gamma_sum
    }
}
