mod ats_pr;

use ats_pr::bls::KeyPairG2;
use ats_pr::pr::ProactiveRefresh;
use ats_pr::threshold::{ThresholdKeyPairs, ThresholdSignature};


// Sample params for T-of-N threshold sig
const T: usize = 2;
const N: usize = 5;
const QUORUM: [usize; 2] = [1, 3];

fn main() {
    println!("=== PR");
    let tkp = ThresholdKeyPairs::new(N, T);
    let message_bytes: [u8; 5] = [1, 2, 3, 4, 5];
    // let tkp = ThresholdKeyPairs::new(N, T);

    // test one sk rotation
    let pr1 = ProactiveRefresh::new(N, T);
    let sk_old = pr1.tkp.keys[0].x;
    println!("single old sk: {:?}", sk_old);
    let sk_new = pr1.update_one(sk_old);
    println!("single new sk: {:?}", sk_new);


    // test all sk rotation
    let mut pr2:ProactiveRefresh = ProactiveRefresh::new(N, T);
    println!("all old aggregate: {:?}", pr2.tkp.quorum_X(&QUORUM.to_vec()));
    println!("all old sk: {:?}", pr2.tkp.get_x(&QUORUM.to_vec()));
    pr2.update_all();
    println!("all new aggregate: {:?}", pr2.tkp.quorum_X(&QUORUM.to_vec()));
    println!("all new sk: {:?}", pr2.tkp.get_x(&QUORUM.to_vec())); 

    println!("=== ATS");

    let mut sig: ThresholdSignature =
        ThresholdSignature::sign(&message_bytes[..], &tkp, &QUORUM.to_vec());
    println!("signature: {:?}", sig);

    let adversarial_quorum = QUORUM.to_vec();
    println!("for correct quorum: {}", sig.verify(&message_bytes[..], &tkp));
    
    sig.quorum = vec![1, 4];
    println!("for adversarial quorum: {}", sig.verify(&message_bytes[..], &tkp));
}
