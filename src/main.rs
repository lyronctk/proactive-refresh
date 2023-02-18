mod ats_pr;

use ats_pr::bls::BLSSignature;
use ats_pr::pr::ProactiveRefresh;
use ats_pr::threshold::ThresholdKeyPairs;



// Sample params for T-of-N threshold sig
const T: usize = 2;
const N: usize = 5;
const QUORUM: [usize; 2] = [0, 3];

fn main() {
    // let tkp = ThresholdKeyPairs::new(N, T);

    // test one sk rotation
    let pr1 = ProactiveRefresh::new(N, T);
    let sk_old = pr1.tkp.keys[0].x;
    println!("single old sk: {:?}", sk_old);
    let sk_new = pr1.update_one(sk_old);
    println!("single new sk: {:?}", sk_new);

    // test all sk rotation
    let mut pr2:ProactiveRefresh = ProactiveRefresh::new(N, T);
    println!("al old: {:?}", pr2.tkp.get_x(&QUORUM.to_vec()));
    pr2.update_all();
    println!("all new: {:?}", pr2.tkp.get_x(&QUORUM.to_vec())); 
    // 

    // let tk = tkp.keys[0] + tkp.keys[1];
    // let message_bytes: [u8; 5] = [1, 2, 3, 4, 5];
    // let sig: BLSSignature = BLSSignature::sign(&message_bytes[..], &tk.x);
    // println!("{}", sig.verify(&message_bytes[..], &tkp.keys[0].X));
}

