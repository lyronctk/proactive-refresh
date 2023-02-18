use crate::ats_pr::bls::KeyPairG2;
use crate::ats_pr::bls::FE1;
use crate::ats_pr::bls::FE2;
use crate::ats_pr::bls::GE1;
use crate::ats_pr::bls::GE2;

use curv::elliptic::curves::traits::ECScalar;
use curv::BigInt;

#[derive(Debug)]
pub struct ThresholdKeyPairs {
    pub keys: Vec<KeyPairG2>,
    n: usize,
}

impl ThresholdKeyPairs {
    pub fn new(_n: usize) -> Self {
        Self {
            keys: vec![KeyPairG2::new(); _n],
            n: _n,
        }
    }

    fn get_quorum(&self, quorum: &Vec<usize>) -> Vec<&KeyPairG2> {
        let mut q: Vec<&KeyPairG2> = Vec::new();
        for idx in quorum {
            if *idx >= self.keys.len() {
                panic!("Quorum indices included party outside of available keys");
            }
            q.push(&self.keys[*idx]);
        }
        q
    }

    pub fn get_X(&self, quorum: &Vec<usize>) -> Vec<GE2> {
        self.get_quorum(quorum)
            .into_iter()
            .map(|key: &KeyPairG2| key.X)
            .collect()
    }

    pub fn get_x(&self, quorum: &Vec<usize>) -> Vec<FE2> {
        self.get_quorum(quorum)
            .into_iter()
            .map(|key: &KeyPairG2| key.x)
            .collect()
    }

    fn lagrange_coeff_f0(&self, party_idx: usize) {
        let j: usize = party_idx + 1; // since quorum is 0-indexed
        let fe1_j: FE1 = ECScalar::from(&BigInt::from(j as u32));
        let mut prod: FE1 = ECScalar::from(&BigInt::from(1));
        for i in 1..=self.n {
            if i == j {
                continue;
            }
            let fe1_i: FE1 = ECScalar::from(&BigInt::from(i as u32));
            let diff = fe1_i.sub(&fe1_j.get_element());
            prod = prod * fe1_i * diff.invert();
        }
    }

    // pub fn quorum_X(&self, quorum: &Vec<usize>) -> FE2 {
    //     for idx in quorum {}
    // }
}
