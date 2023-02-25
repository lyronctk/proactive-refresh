/*
 * Accountable BLS threshold signatures. 
 */
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
    /*
     * Instantiates a threshold scheme by sampling _n new key pairs and saving
     * threshold param _t. 
     */
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

    /*
     * Constructs object from key pairs & a threshold. 
     */
    pub fn from(ks: Vec<KeyPairG2>, _t: usize) -> Self {
        Self {
            n: ks.len(),
            t: _t,
            keys: ks, 
        }
    }

    /*
     * Getter for key pair of party at idx. 
     */
    pub fn get(&self, idx: usize) -> &KeyPairG2 {
        if idx >= self.keys.len() {
            panic!("Tried to access key at idx >= n");
        }
        &self.keys[idx]
    }

    /*
     * Getter for key pairs of a given quorum. 
     */
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

    /*
     * Getter for public keys of a given quorum. 
     */
    fn get_pubs(&self, quorum: &Vec<usize>) -> Vec<GE2> {
        self.get_quorum_keys(quorum)
            .into_iter()
            .map(|key: &KeyPairG2| key.pub_key())
            .collect()
    }

    /*
     * Computes collective public key of a given quorum via lagrange 
     * interpolation. Returns f(0) of the lowest degree polynomial that passes
     * through the points {(party_idx, party_pubkey)} for all parties in the 
     * quorum.
     */
    pub fn collective_pub(&self, quorum: &Vec<usize>) -> GE2 {
        lagrange_interpolate_f0(
            &quorum
                .into_iter()
                .map(|idx: &usize| idx + 1)
                .zip(self.get_pubs(&quorum).into_iter())
                .collect(),
        )
    }

    /*
     * Update the secret key of the jth party. 
     */
    pub fn update_secret(&mut self, j: usize, upd: FE2) {
        if j >= self.keys.len() {
            panic!("Tried to update key at idx >= n");
        }
        self.keys[j].update_secret(upd);
    }

    /*
     * Threshold param. 
     */
    pub fn t(&self) -> usize {
        self.t
    }

    /*
     * Number of parties. 
     */
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
    /*
     * Creates signature shares for each party in the quorum. Collective 
     * signature is f(0) for the lowest degree polynomial passing through the
     * sig shares. See ThresholdKeyPairs::collective_pub() for an analogous 
     * computation.
     */
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

    /*
     * Signature is valid if it is 1) signed by t parties and 2) verifies 
     * under the quorum's collective public key. 
     */
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
