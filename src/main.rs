mod ats_pr;

use ats_pr::bls::BLSSignature;
use ats_pr::bls::GE2;
use ats_pr::threshold::ThresholdKeyPairs;
use curv::elliptic::curves::traits::ECPoint;

// Sample params for T-of-N threshold sig
const T: usize = 2;
const N: usize = 5;
const QUORUM: [usize; 2] = [1, 3];

fn main() {
    let tkp = ThresholdKeyPairs::new(N, T);
    let qx = tkp.quorum_x(&QUORUM.to_vec());
    let qX = tkp.quorum_X(&QUORUM.to_vec());

    let message_bytes: [u8; 5] = [1, 2, 3, 4, 5];
    let sig: BLSSignature = BLSSignature::sign(&message_bytes[..], &qx);
    println!("{}", sig.verify(&message_bytes[..], &qX));
}
