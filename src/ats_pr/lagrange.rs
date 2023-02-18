use crate::ats_pr::bls::ECScalar;
use crate::ats_pr::bls::FE1;
use crate::ats_pr::bls::FE2;
use crate::ats_pr::bls::GE1;

use curv::BigInt;

pub fn lagrange_coeff_f0(points: &Vec<(usize, FE2)>, j: usize) -> FE2 {
    println!("j: {}", j);
    let fe2_xj: FE2 = ECScalar::from(&BigInt::from(points[j].0 as u32));
    let mut prod: FE2 = ECScalar::from(&BigInt::from(1));
    for (i, pi) in points.iter().enumerate() {
        if i == j {
            continue;
        }
        let fe2_xi: FE2 = ECScalar::from(&BigInt::from(points[i].0 as u32));
        let diff: FE2 = fe2_xi.sub(&fe2_xj.get_element());
        println!("fe2_xi: {:?}", fe2_xi);
        println!("fe2_xj: {:?}", fe2_xj);
        println!("diff: {:?}", diff);
        prod = prod * fe2_xi * diff.invert();
        println!("prod: {:?}", prod);
    }
    prod
}

pub fn lagrange_interpolate_f0(points: &Vec<(usize, FE2)>) -> FE2 {
    let mut summation: FE2 = ECScalar::from(&BigInt::from(0));
    for (j, p) in points.iter().enumerate() {
        summation = summation + lagrange_coeff_f0(points, j) * p.1;
    }
    summation
}
