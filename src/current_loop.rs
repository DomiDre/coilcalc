use crate::utils::complete_elliptic_integrals;

pub struct CurrentLoop {
    r_center: (f64, f64, f64),
    radius: f64,
    current: f64,
}

impl CurrentLoop {
    /// Create new current loop assumed to flow in a circle around the z-axis
    /// # Arguments
    /// * `x_center` - x position of the center-point [in mm]
    /// * `y_center` - y position of the center-point [in mm]
    /// * `z_center` - z position of the center-point [in mm]
    /// * `radius` - radius [in mm]
    /// * `current` - current [in A]
    pub fn new(
        x_center: f64,
        y_center: f64,
        z_center: f64,
        radius: f64,
        current: f64,
    ) -> CurrentLoop {
        CurrentLoop {
            r_center: (x_center, y_center, z_center),
            radius,
            current,
        }
    }

    /// Calculate the magnetic field vector at position (x,y,z) produced by the current loop
    /// # Arguments
    /// * `x` - x-position of probed area [in mm]
    /// * `y` - y-position of probed area [in mm]
    /// * `z` - z-position of probed area [in mm]
    pub fn magnetic_field(&self, x: f64, y: f64, z: f64) -> (f64, f64, f64) {
        let x = x - self.r_center.0;
        let y = y - self.r_center.1;
        let z = z - self.r_center.2;

        // transform x,y to rho, phi
        let rho = (x.powi(2) + y.powi(2)).sqrt();
        let phi = y.atan2(x);
        let z2 = z.powi(2);
        let r2 = self.radius.powi(2);
        let rho2 = rho.powi(2);

        let d = ((rho + self.radius).powi(2) + z2).sqrt();
        let m = 4.0 * rho * self.radius / d.powi(2);
        let (ell_k, ell_e) = complete_elliptic_integrals(m);
        let nominator = (self.radius - rho).powi(2) + z2;
        let b_r;
        if (rho == 0.0) || (nominator == 0.0) {
            b_r = 0.0;
        } else {
            b_r = 0.2 * self.current * z / (rho * d)
                * (-ell_k + ell_e * (r2 + rho2 + z2) / nominator);
        }

        let b_z;
        if nominator == 0.0 {
            b_z = 0.0;
        } else {
            b_z = 0.2 * self.current / d * (ell_k + ell_e * (r2 - rho2 - z2) / nominator);
        }
        (b_r * phi.cos(), b_r * phi.sin(), b_z)
    }
}

/// Calculate magnetic field produced by current loop(s) within the field of view
pub fn calculate_magnetic_field(
    current_loops: &Vec<CurrentLoop>,
    x_range: &(f64, f64, usize),
    z_range: &(f64, f64, usize),
) -> Vec<Vec<(f64, f64, f64)>> {
    let dx = (x_range.1 - x_range.0) / ((x_range.2 - 1) as f64);
    let dz = (z_range.1 - z_range.0) / ((z_range.2 - 1) as f64);
    let mut magnetic_field = Vec::new();
    for i_z in 0..z_range.2 {
        let z = z_range.0 + (i_z as f64) * dz;
        let mut row = Vec::new();
        for i_x in 0..x_range.2 {
            let x = x_range.0 + (i_x as f64) * dx;
            let mut b_field = (0.0, 0.0, 0.0);
            for current_loop in current_loops.iter() {
                let add_field = current_loop.magnetic_field(x, 0.0, z);
                b_field.0 += add_field.0;
                b_field.1 += add_field.1;
                b_field.2 += add_field.2;
            }
            row.push(b_field);
        }
        magnetic_field.push(row);
    }
    magnetic_field
}

#[test]
///Test the create function of the current loop
fn create_current_loop() {
    use crate::current_loop::CurrentLoop;
    let current_loop = CurrentLoop::new(1.0, 2.0, 3.0, 12.0, 1.23);
    assert_eq!(current_loop.r_center.0, 1.0);
    assert_eq!(current_loop.r_center.1, 2.0);
    assert_eq!(current_loop.r_center.2, 3.0);
    assert_eq!(current_loop.radius, 12.0);
    assert_eq!(current_loop.current, 1.23);
}
