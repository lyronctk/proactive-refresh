use crate::ats_pr::bls::ECScalar;
use crate::ats_pr::bls::FE1;
use crate::ats_pr::bls::GE1;

use curv::BigInt;

fn lagrange_coeff_f0(n: usize, party_idx: usize) {
    let j: usize = party_idx + 1; // since quorum is 0-indexed
    let fe1_j: FE1 = ECScalar::from(&BigInt::from(j as u32));
    let mut prod: FE1 = ECScalar::from(&BigInt::from(1));
    for i in 1..=n {
        if i == j {
            continue;
        }
        let fe1_i: FE1 = ECScalar::from(&BigInt::from(i as u32));
        let diff = fe1_i.sub(&fe1_j.get_element());
        prod = prod * fe1_i * diff.invert();
    }
}
