use crate::bls::{BLSSignature, KeyPairG2, FE2, GE1, GE2};
use crate::lagrange::{
    lagrange_interpolate_f0,
};
use std::fmt;

#[derive(Debug)]
pub struct ThresholdKeyPairs {
    keys: Vec<KeyPairG2>,
    n: usize,
    t: usize,
}

#[derive(Debug)]
pub struct ThresholdSignature {
    sig: BLSSignature,
    quorum: Vec<usize>
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

    /*
     * Getter for keypair of party at idx. 
     */
    pub fn get(&self, idx: usize) -> &KeyPairG2 {
        if idx >= self.keys.len() {
            panic!("Tried to access key at idx >= n");
        }
        &self.keys[idx]
    }

    pub fn get_quorum_keys(&self, quorum: &Vec<usize>) -> Vec<&KeyPairG2> {
        let mut q: Vec<&KeyPairG2> = Vec::new();
        for idx in quorum {
            if *idx > self.keys.len() {
                panic!("Quorum indices included party outside of available keys");
            }
            q.push(&self.keys[*idx]);
        }
        q
    }

    fn get_pubs(&self, quorum: &Vec<usize>) -> Vec<GE2> {
        self.get_quorum_keys(quorum)
            .into_iter()
            .map(|key: &KeyPairG2| key.pub_key())
            .collect()
    }

    pub fn collective_pub(&self, quorum: &Vec<usize>) -> GE2 {
        lagrange_interpolate_f0(
            &quorum
                .into_iter()
                .map(|idx: &usize| idx + 1)
                .zip(self.get_pubs(&quorum).into_iter())
                .collect(),
        )
    }

    pub fn n_keys(&self) -> usize {
        self.keys.len()
    }

    pub fn update_secret(&mut self, j: usize, upd: FE2) {
        if j >= self.keys.len() {
            panic!("Tried to update key at idx >= n");
        }
        self.keys[j].update_secret(upd);
    }

    pub fn t(&self) -> usize {
        self.t
    }

    pub fn n(&self) -> usize {
        self.n
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
        for key in tkps.get_quorum_keys(quorum) {
            sigmas.push(BLSSignature::sign(message, &key.priv_key()).sigma())
        }
        let sigma = lagrange_interpolate_f0(
            &quorum
                .into_iter()
                .map(|idx: &usize| idx + 1)
                .zip(sigmas.into_iter())
                .collect(),
        );
        ThresholdSignature { sig: BLSSignature::from(sigma), quorum: quorum.clone() }
    }

    pub fn verify(&self, message: &[u8], tkps: &ThresholdKeyPairs) -> bool {
        if self.quorum.len() < tkps.t {
            println!("- Verification failed. Quorum has fewer than t participants.");
            return false;
        }
        return self.sig.verify(message, &tkps.collective_pub(&self.quorum));
    }
}

impl fmt::Display for ThresholdSignature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(sigma: {}, quorum: {:?})", self.sig, self.quorum)
    }
}
