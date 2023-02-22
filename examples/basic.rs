use proactive_refresh::{bls::KeyPairG2, pr::ProactiveRefresh, threshold::ThresholdSignature};

const T: usize = 5;
const N: usize = 7;
const REFRESH_COUNT: usize = 2;
const QUORUM: [usize; 5] = [1, 2, 4, 5, 6];
const MESSAGE_BYTES: [u8; 5] = [1, 2, 3, 4, 5];

fn main() {
    let mut parties: ProactiveRefresh = ProactiveRefresh::new(N, T);

    let sig1: ThresholdSignature =
        ThresholdSignature::sign(&MESSAGE_BYTES[..], &parties.getKeys(), &QUORUM.to_vec());

    // Pretend that adversary compromises keys for party 1 & 5 
    let stolen1: KeyPairG2 = parties.getKeys().getParty(1);
    let stolen5: KeyPairG2 = parties.getKeys().getParty(5);

    // Run refresh protocol a few times
    for i in 0..=REFRESH_COUNT {
        println!("== Refresh {}", i);
        parties.refresh_all();
        println!("- {}", parties);
        println!(
            "- Collective pubkey for {:?}: {:?}",
            QUORUM,
            parties.getKeys().quorum_X(&QUORUM.to_vec())
        );
        println!("==");
    }

    // Demonstrate threshold public key unchanged
    println!("== Signature signed before refresh still validates");
    println!(
        "- {} validates? {}",
        sig1,
        sig1.verify(&MESSAGE_BYTES[..], parties.getKeys())
    );
    println!("==");

    // Demonstrate that collective signature of quorum unchanged
    println!("== Signature from before refresh same as after");
    println!("- Before: {}", sig1);
    println!(
        "- After: {}",
        ThresholdSignature::sign(&MESSAGE_BYTES[..], &parties.getKeys(), &QUORUM.to_vec())
    );
    println!("==");

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
