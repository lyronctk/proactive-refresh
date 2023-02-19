mod ats_pr;

use ats_pr::pr::ProactiveRefresh;
use ats_pr::threshold::{ThresholdKeyPairs, ThresholdSignature};

use curv::arithmetic::Converter;
use curv::elliptic::curves::traits::ECScalar;

use serde_json::json;
use std::collections::HashMap;
use std::fs::File;

// Demo simulation params
const T: usize = 5;
const N: usize = 7;
const QUORUM: [usize; 3] = [0, 1, 2];
const BREACHES: [(usize, usize); 5] = [(2, 2), (5, 5), (7, 6), (10, 4), (14, 3)];
const MAX_TIME: usize = 15;

const OUT_FILE: &str = "./out/sim.json";

fn format_pkx(committee: &ProactiveRefresh) -> Vec<String> {
    Vec::from_iter(
        committee
            .tkp
            .get_x(&Vec::from_iter(0..N))
            .into_iter()
            .map(|x| "0x".to_string() + &x.to_big_int().to_str_radix(16)),
    )
}

fn main() {
    let mut committee: ProactiveRefresh = ProactiveRefresh::new(N, T);

    let mut epochs = Vec::new();
    for i in 0..MAX_TIME {
        let mut pk_status = Vec::new();
        for pkx in format_pkx(&committee) {
            let mut hm = HashMap::new();
            hm.insert("key", pkx);
            hm.insert("secure", false.to_string());
            pk_status.push(hm);
        }
        let epoch_json = json!({
            "time": i,
            "ats_ptr": {
                "breached": false,
                "pks": pk_status
            }
        });
        epochs.push(epoch_json);
        committee.refresh_all();
    }
    serde_json::to_writer(&File::create(OUT_FILE).unwrap(), &epochs).unwrap();

    // println!("=== PR");
    // let tkp = ThresholdKeyPairs::new(N, T);
    // let message_bytes: [u8; 5] = [1, 2, 3, 4, 5];
    // // let tkp = ThresholdKeyPairs::new(N, T);

    // // test all sk rotation
    // let mut pr2:ProactiveRefresh = ProactiveRefresh::new(N, T);
    // println!("all old aggregate: {:?}", pr2.tkp.quorum_x(&QUORUM.to_vec()));
    // println!("all old sks: {:?}", pr2.tkp.get_x(&QUORUM.to_vec()));
    // pr2.refresh_all();
    // println!("all new aggregate: {:?}", pr2.tkp.quorum_x(&QUORUM.to_vec()));
    // println!("all new sks: {:?}", pr2.tkp.get_x(&QUORUM.to_vec()));

    // println!("=== ATS");

    // let mut sig: ThresholdSignature =
    //     ThresholdSignature::sign(&message_bytes[..], &tkp, &QUORUM.to_vec());
    // println!("signature: {:?}", sig);

    // let adversarial_quorum = QUORUM.to_vec();
    // println!("for correct quorum: {}", sig.verify(&message_bytes[..], &tkp));

    // sig.quorum = vec![1, 4];
    // println!("for adversarial quorum: {}", sig.verify(&message_bytes[..], &tkp));
}
