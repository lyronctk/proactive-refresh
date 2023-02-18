use crate::ats_pr::bls::ECScalar;
use crate::ats_pr::bls::KeyPairG2;
use crate::ats_pr::bls::FE2;
use crate::ats_pr::bls::GE2;
use crate::ats_pr::lagrange::lagrange_interpolate_f0;

use curv::BigInt;

#[derive(Debug)]
pub struct ThresholdKeyPairs {
    pub keys: Vec<KeyPairG2>,
    n: usize,
    t: usize,
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

    pub fn quorum_X(&self, quorum: &Vec<usize>) {
        // (1, 6)
        // (3, 8)
        // should find: (0, 5)
        // let tester: Vec<FE2> = vec![
        //     ECScalar::from(&BigInt::from(6u32)),
        //     ECScalar::from(&BigInt::from(8u32)),
        // ];

        // println!(
        //     "{:?}",
        //     lagrange_interpolate_f0(
        //         &quorum
        //             .into_iter()
        //             .map(|idx: &usize| idx + 1)
        //             .zip(tester.into_iter())
        //             .collect(),
        //     )
        // )

        let tester: Vec<usize> = vec![6, 8];

        println!(
            "{:?}",
            lagrange_interpolate_f0(
                &quorum
                    .into_iter()
                    .map(|idx: &usize| idx + 1)
                    .zip(tester.into_iter())
                    .collect(),
            )
        )
    }
}
