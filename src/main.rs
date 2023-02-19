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
const QUORUM: [usize; 5] = [0, 1, 2, 3, 4];
const BREACHES: [(usize, usize); 5] = [(2, 1), (5, 6), (7, 5), (10, 3), (14, 2)];
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
    let mut committee_pr: ProactiveRefresh = ProactiveRefresh::new(N, T);

    let mut secure: Vec<bool> = vec![true; N];
    let mut secure_pr: Vec<bool> = vec![true; N];

    let mut is_breached = false;
    let mut is_breached_pr = false;

    let mut breach_ctr = 0; 
    let mut epochs = Vec::new();
    for i in 0..MAX_TIME {

        if breach_ctr < BREACHES.len() && BREACHES[breach_ctr].0 == i {
            secure[BREACHES[breach_ctr].1] = false;
            secure_pr[BREACHES[breach_ctr].1] = false;
        }
        if breach_ctr == BREACHES.len() - 1 {
            is_breached = true;
        }

        let mut pk_status = Vec::new();
        for (j, pkx) in format_pkx(&committee).iter().enumerate() {
            let mut hm = HashMap::new();
            hm.insert("key", pkx.clone());
            hm.insert("secure", secure[j].to_string());
            pk_status.push(hm);
        }
        let cx = ECScalar::to_big_int(&committee.tkp.quorum_x(&QUORUM.to_vec()));

        let mut pk_status_pr = Vec::new();
        for (j, pkx) in format_pkx(&committee_pr).iter().enumerate() {
            let mut hm = HashMap::new();
            hm.insert("key", pkx.clone());
            hm.insert("secure", secure_pr[j].to_string());
            pk_status_pr.push(hm);
        }        
        let cx_pr = ECScalar::to_big_int(&committee_pr.tkp.quorum_x(&QUORUM.to_vec()));

        let epoch_json = json!({
            "time": i,
            "ats": {
                "breached": is_breached.to_string(),
                "collective_pk": "0x".to_string() + &cx.to_str_radix(16),
                "pks": pk_status      
            },
            "ats_pr": {
                "breached": is_breached_pr.to_string(),
                "collective_pk": "0x".to_string() + &cx_pr.to_str_radix(16),
                "pks": pk_status_pr
            }
        });
        epochs.push(epoch_json);

        if BREACHES[breach_ctr].0 == i {
            secure_pr[BREACHES[breach_ctr].1] = true;
            breach_ctr += 1;
        }
        committee_pr.refresh_all();
    }
    // serde_json::to_writer(&File::create(OUT_FILE).unwrap(), &epochs).unwrap();

    // println!("=== PR");
    // let tkp = ThresholdKeyPairs::new(N, T);
    // let message_bytes: [u8; 5] = [1, 2, 3, 4, 5];
    // // let tkp = ThresholdKeyPairs::new(N, T);

    // test all sk rotation
    println!(); 
    println!();
    println!();  
    println!("==================================== PRE-PROACTIVE REFRESH ===================================="); 
    let mut pr2:ProactiveRefresh = ProactiveRefresh::new(N, T);
    println!("old aggregate sk: {:?}", ECScalar::to_big_int(&pr2.tkp.quorum_x(&QUORUM.to_vec())));
    // println!();  
    // println!("all old aggregate pk: {:?}", pr2.tkp.quorum_X(&QUORUM.to_vec()));
    println!(); 
    println!("all old sks in quorum:");
    for i in 0..QUORUM.len() {
        println!("{}) {:?}", i + 1, ECScalar::to_big_int(&pr2.tkp.get_x(&QUORUM.to_vec())[i]));
    }
    println!();
    println!(); 
    println!("==================================== POST-PROACTIVE REFRESH ===================================="); 
    // ROTATE KEYS
    pr2.refresh_all();
    println!("new aggregate sk: {:?}", ECScalar::to_big_int(&pr2.tkp.quorum_x(&QUORUM.to_vec())));
    // println!(); 
    // println!("all new aggregate pk: {:?}", pr2.tkp.quorum_X(&QUORUM.to_vec()));
    println!();
    println!("all new sks in quorum:");
    for i in 0..QUORUM.len() {
        println!("{}) {:?}", i + 1, ECScalar::to_big_int(&pr2.tkp.get_x(&QUORUM.to_vec())[i]));
    }
    

    // println!("=== ATS");
    // let mut sig: ThresholdSignature =
    //     ThresholdSignature::sign(&message_bytes[..], &tkp, &QUORUM.to_vec());
    // println!("signature: {:?}", sig);

    // let adversarial_quorum = QUORUM.to_vec();
    // println!("for correct quorum: {}", sig.verify(&message_bytes[..], &tkp));

    // sig.quorum = vec![1, 4];
    // println!("for adversarial quorum: {}", sig.verify(&message_bytes[..], &tkp));
}
