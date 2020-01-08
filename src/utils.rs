/// Computes complete elliptic integrals of the first kind K(m) and second kind E(m).
/// Defined by:
/// K(m) = Int[0..1] 1 / sqrt( (1-x^2)*(1-m*x^2) ) dx
/// E(m) = Int[0..1] sqrt( (1-m*x^2) / (1-x^2) ) dx
/// Using polynomial approximations valid for 0 <= m < 1 with abs. error < 2e-8 from:
/// Handbook of Mathematical Functions
/// by M. Abramowitz, I. A. Stegun
/// National Bureau of Standards
/// 10ed., 1972
/// page 590ff
pub fn complete_elliptic_integrals(m: f64) -> (f64, f64) {
    let ellint_1: f64;
    let ellint_2: f64;
    if m > 1.0 || m < 0.0 {
        panic!("Complete Elliptic Integral only takes arguments between 0 <= m <= 1")
    }
    if (m - 1.0).abs() < std::f64::EPSILON {
        // if m = 1 => m1 = 0; therefore avoid case with ln(0)
        ellint_1 = std::f64::MAX;
        ellint_2 = 1.0;
    } else {
        let m1 = 1.0 - m; // complementary parameter m1 defined by m + m1 = 1
        let m1_2 = m1.powi(2);
        let m1_3 = m1 * m1_2;
        let m1_4 = m1_2.powi(2);
        let m1_ln = m1.ln();
        let a_k = 1.386_294_361_12
            + 0.096_663_442_59 * m1
            + 0.035_900_923_83 * m1_2
            + 0.037_425_637_13 * m1_3
            + 0.014_511_962_12 * m1_4;
        let b_k = 0.5
            + 0.124_985_935_97 * m1
            + 0.068_802_485_76 * m1_2
            + 0.033_283_553_46 * m1_3
            + 0.004_417_870_12 * m1_4;
        ellint_1 = a_k - b_k * m1_ln;

        let a_e = 1.0
            + 0.443_251_414_63 * m1
            + 0.062_606_012_20 * m1_2
            + 0.047_573_835_46 * m1_3
            + 0.017_365_064_51 * m1_4;
        let b_e = 0.249_983_683_10 * m1
            + 0.092_001_800_37 * m1_2
            + 0.040_696_975_26 * m1_3
            + 0.005_264_496_39 * m1_4;
        ellint_2 = a_e - b_e * m1_ln;
    }
    (ellint_1, ellint_2)
}

#[test]
fn elliptic_integrals_sanity_check() {
    use crate::utils::complete_elliptic_integrals;
    let (ellint_k, ellint_e) = complete_elliptic_integrals(0.0);
    if (ellint_k - 1.57079632679).abs() > 2e-8 {
        panic!(
            "K(0) does not evaluate to pi/2 within error bounds. Got K(0) = {}",
            ellint_k
        );
    }
    if (ellint_e - 1.57079632679).abs() > 2e-8 {
        panic!(
            "E(0) does not evaluate to pi/2 within error bounds. Got E(0) = {}",
            ellint_e
        );
    }
}
