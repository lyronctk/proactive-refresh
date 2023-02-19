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
    println!("all old: {:?}", pr2.tkp.get_x(&QUORUM.to_vec()));
    pr2.update_all();
    println!("all new: {:?}", pr2.tkp.get_x(&QUORUM.to_vec()));
    //

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
