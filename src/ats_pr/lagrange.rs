use crate::ats_pr::bls::ECPoint;
use crate::ats_pr::bls::ECScalar;
use crate::ats_pr::bls::FE2;
use crate::ats_pr::bls::GE2;

use curv::BigInt;

pub fn lagrange_coeff_f0<T>(points: &Vec<(usize, T)>, j: usize) -> FE2 {
    let fe2_xj: FE2 = ECScalar::from(&BigInt::from(points[j].0 as u32));
    let mut prod: FE2 = ECScalar::from(&BigInt::from(1));
    for (i, pi) in points.iter().enumerate() {
        if i == j {
            continue;
        }
        let fe2_xi: FE2 = ECScalar::from(&BigInt::from(points[i].0 as u32));
        let diff: FE2 = fe2_xi.sub(&fe2_xj.get_element());
        prod = prod * fe2_xi * diff.invert();
    }
    prod
}

// [TODO] Combine both functions below into one generic
pub fn lagrange_interpolate_f0_X(points: &Vec<(usize, GE2)>) -> GE2 {
    println!("POINTS FOR X: {:?}", points);
    let mut summation: GE2 = GE2::generator().sub_point(&GE2::generator().get_element());
    for (j, p) in points.iter().enumerate() {
        summation = summation + p.1 * lagrange_coeff_f0(points, j);
    }
    summation
}

pub fn lagrange_interpolate_f0_x(points: &Vec<(usize, FE2)>) -> FE2 {
    println!("POINTS FOR x: {:?}", points);
    let mut summation: FE2 = FE2::zero();
    for (j, p) in points.iter().enumerate() {
        summation = summation + p.1 * lagrange_coeff_f0(points, j);
    }
    summation
}
