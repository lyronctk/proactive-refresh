use crate::ats_pr::bls::KeyPairG2;
use crate::ats_pr::bls::FE2;
use crate::ats_pr::bls::GE2;

#[derive(Debug)]
pub struct ThresholdKeyPairs {
    pub keys: Vec<KeyPairG2>,
}

impl ThresholdKeyPairs {
    pub fn new(n: usize) -> Self {
        Self {
            keys: vec![KeyPairG2::new(); n],
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
}
