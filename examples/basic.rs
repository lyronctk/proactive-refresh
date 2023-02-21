use proactive_refresh::{
    pr::ProactiveRefresh,
    threshold::{ThresholdSignature},
};

const T: usize = 5;
const N: usize = 7;
const QUORUM: [usize; 5] = [1, 2, 4, 5, 6];
const MESSAGE_BYTES: [u8; 5] = [1, 2, 3, 4, 5];

fn main() {
    // test all sk rotation
    let mut parties: ProactiveRefresh = ProactiveRefresh::new(N, T);
    println!("{}", parties);    

    // println!(
    //     "all old aggregate: {:?}",
    //     pr2.tkp.quorum_x(&QUORUM.to_vec())
    // );
    // println!("all old sks: {:?}", pr2.tkp.get_x(&QUORUM.to_vec()));
    // pr2.refresh_all();
    // println!(
    //     "all new aggregate: {:?}",
    //     pr2.tkp.quorum_x(&QUORUM.to_vec())
    // );
    // println!("all new sks: {:?}", pr2.tkp.get_x(&QUORUM.to_vec()));

    // println!("=== ATS");
    // let mut sig: ThresholdSignature =
    //     ThresholdSignature::sign(&message_bytes[..], &tkp, &QUORUM.to_vec());
    // println!("signature: {:?}", sig);

    // let adversarial_quorum = QUORUM.to_vec();
    // println!(
    //     "for correct quorum: {}",
    //     sig.verify(&message_bytes[..], &tkp)
    // );

    // sig.quorum = vec![1, 4];
    // println!(
    //     "for adversarial quorum: {}",
    //     sig.verify(&message_bytes[..], &tkp)
    // );
}
