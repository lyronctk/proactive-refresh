use crate::ats_pr::bls::ECScalar;
use crate::ats_pr::bls::FE1;
use crate::ats_pr::bls::FE2;
use crate::ats_pr::bls::GE1;

use curv::BigInt;

pub fn lagrange_coeff_f0(n: usize, j: usize) -> FE2 {
    let fe1_j: FE2 = ECScalar::from(&BigInt::from(j as u32));
    let mut prod: FE2 = ECScalar::from(&BigInt::from(1));
    for i in 1..=n {
        if i == j {
            continue;
        }
        let fe1_i: FE2 = ECScalar::from(&BigInt::from(i as u32));
        let diff = fe1_i.sub(&fe1_j.get_element());
        prod = prod * fe1_i * diff.invert();
    }
    prod
}

pub fn lagrange_interpolate_f0(points: &Vec<(usize, usize)>) -> BigInt {
    println!("POINTS {:?}", points);
    // let mut summation: FE2 = ECScalar::from(&BigInt::from(0));
    let mut summation: BigInt = BigInt::from(0);
    for p in points {
        summation = summation + lagrange_coeff_f0(points.len(), p.0).to_big_int() * BigInt::from(p.1 as u32);
    }
    summation
}
