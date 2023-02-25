/*
 * Basic example of using proactive refresh on BLS threshold signatures. Goes
 * through generating the keys, refreshing shares, and signing messages.
 */
use proactive_refresh::{
    bls::KeyPairG2, pr::ProactiveRefresh, threshold::ThresholdKeyPairs,
    threshold::ThresholdSignature,
};

const T: usize = 5;
const N: usize = 7;
const REFRESH_COUNT: usize = 2;
const QUORUM: [usize; 5] = [0, 1, 3, 4, 5];
const MESSAGE_BYTES: [u8; 5] = [9, 9, 9, 9, 9];

fn main() {
    let mut parties: ProactiveRefresh = ProactiveRefresh::new(N, T);

    let sig1: ThresholdSignature =
        ThresholdSignature::sign(&MESSAGE_BYTES[..], &parties.threshold_keys(), &QUORUM.to_vec());

    // Pretend that adversary compromises keys for party 0 & 4
    let stolen0: KeyPairG2 = parties.threshold_keys().get(0).clone();
    let stolen4: KeyPairG2 = parties.threshold_keys().get(4).clone();

    // Run refresh protocol a few times
    for i in 0..=REFRESH_COUNT {
        println!("== Refresh {}", i);
        parties.refresh_all();
        println!("- {}", parties);
        println!(
            "- Collective pubkey for {:?}: {:?}",
            QUORUM,
            parties.threshold_keys().collective_pub(&QUORUM.to_vec())
        );
        println!("==");
    }

    // Demonstrate threshold public key unchanged
    println!("== Signature signed before refresh still validates");
    println!("- {}", sig1);
    println!(
        "- validates? {}",
        sig1.verify(&MESSAGE_BYTES[..], parties.threshold_keys())
    );
    println!("==");

    // Demonstrate that collective signature of quorum unchanged
    println!("== Signature from before refresh same as after");
    println!("- Before: {}", sig1);
    println!(
        "- After: {}",
        ThresholdSignature::sign(&MESSAGE_BYTES[..], &parties.threshold_keys(), &QUORUM.to_vec())
    );
    println!("==");

    // Demonstrate that keys from different refresh intervals cannot be
    // combined to create a valid signature under the same public key
    println!("== Stolen keys from diff rounds don't create valid signatures");
    let advers_recon: ThresholdKeyPairs = ThresholdKeyPairs::from(
        vec![
            stolen0,
            parties.threshold_keys().get(1).clone(),
            parties.threshold_keys().get(2).clone(),
            parties.threshold_keys().get(3).clone(),
            stolen4,
            parties.threshold_keys().get(4).clone(),
            parties.threshold_keys().get(5).clone(),
        ],
        T,
    );
    let sig2: ThresholdSignature =
        ThresholdSignature::sign(&MESSAGE_BYTES[..], &advers_recon, &QUORUM.to_vec());
    println!(
        "- validates? {}",
        sig2.verify(&MESSAGE_BYTES[..], &parties.threshold_keys())
    );
    println!("==");
}
