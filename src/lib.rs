use std::fmt;   
use std::ops::{Add, Sub, AddAssign, Mul, BitXor, Neg};


type Real = f32; 
#[derive(PartialEq, Clone, Copy)]
pub struct Multivector {
    comps: [Real; 8]
}

#[allow(dead_code)]
impl Multivector {
    pub fn new(comps: [Real; 8]) -> Multivector {
        Multivector {comps: comps}
    }
    
    pub fn comps(self) -> [Real; 8] {
        self.comps
    }
    
    pub fn new_grade1(comp: [Real; 3]) -> Multivector {
        Multivector::new([0., comp[0], comp[1], comp[2], 0., 0., 0., 0.])
    }

    pub fn new_grade2(comp: [Real; 3]) -> Multivector {
        Multivector::new([0., 0., 0., 0., comp[0], comp[1], comp[2], 0.])
    }

    pub fn scaled(self, scalar: Real) -> Multivector {
        Multivector::new([
            scalar*self.comps[0], scalar*self.comps[1], scalar*self.comps[2], scalar*self.comps[3],
            scalar*self.comps[4], scalar*self.comps[5], scalar*self.comps[6], scalar*self.comps[7]
        ])
    }
    
    pub fn complement(self) -> Multivector {
        // The orthogonal complement of a multivector
        Multivector::new([
            -self.comps[7], -self.comps[6], self.comps[5], -self.comps[4],
            self.comps[3], -self.comps[2], self.comps[1], self.comps[0]
        ])
    }

    pub fn unit_vector(self) -> Multivector {
        // This multivector when normalized to unit length (first, non-vector components are set to zero)
        let inv_len: Real = (self.comps[1].powi(2) + self.comps[2].powi(2) + self.comps[3].powi(2)).powf(-0.5);

        Multivector::new([0., inv_len*self.comps[1], inv_len*self.comps[2], inv_len*self.comps[3], 0., 0., 0., 0.])
    }

    pub fn unit_bivector(self) -> Multivector {
        // This multivector when normalized to unit length (first, non-bivector components are set to zero)
        let inv_area: Real = (self.comps[4].powi(2) + self.comps[5].powi(2) + self.comps[6].powi(2)).powf(-0.5);

        Multivector::new([0., 0., 0., 0., inv_area*self.comps[4], inv_area*self.comps[5], inv_area*self.comps[6], 0.])
    }
}


impl Add for Multivector {
    // Sum of two multivectors
    type Output = Multivector;

    fn add(self, right: Multivector) -> Multivector {
        Multivector::new([
            self.comps[0]+right.comps[0], self.comps[1]+right.comps[1],
            self.comps[2]+right.comps[2], self.comps[3]+right.comps[3],
            self.comps[4]+right.comps[4], self.comps[5]+right.comps[5],
            self.comps[6]+right.comps[6], self.comps[7]+right.comps[7]
        ])
    }
}

impl Sub for Multivector {
    // Difference between two multivectors
    type Output = Multivector;

    fn sub(self, right: Multivector) -> Multivector {
        Multivector::new([
            self.comps[0] - right.comps[0], self.comps[1] - right.comps[1],
            self.comps[2] - right.comps[2], self.comps[3] - right.comps[3],
            self.comps[4] - right.comps[4], self.comps[5] - right.comps[5],
            self.comps[6] - right.comps[6], self.comps[7] - right.comps[7]
        ])
    }
}
impl AddAssign for Multivector {
    fn add_assign(&mut self, right: Multivector) {
        *self = Multivector::new([
            self.comps[0] + right.comps[0], self.comps[1] + right.comps[1],
            self.comps[2] + right.comps[2], self.comps[3] + right.comps[3],
            self.comps[4] + right.comps[4], self.comps[5] + right.comps[5],
            self.comps[6] + right.comps[6], self.comps[7] + right.comps[7]
        ])
    }
}

impl Mul for Multivector {
    // Clifford product in R^{3,0}
    type Output = Multivector;

    fn mul(self, right: Multivector) -> Multivector {
        let a = self.comps;
        let b = right.comps;

        // This is from the multiplication table
        let ab: [Real; 8] = [
            a[0]*b[0] + a[1]*b[1] + a[2]*b[2] + a[3]*b[3] - a[4]*b[4] - a[5]*b[5] - a[6]*b[6] - a[7]*b[7],
            a[1]*b[0] + a[0]*b[1] + a[4]*b[2] - a[2]*b[4] + a[5]*b[3] - a[3]*b[5] - a[7]*b[6] - a[6]*b[7],
            a[2]*b[0] + a[0]*b[2] - a[4]*b[1] + a[1]*b[4] + a[6]*b[3] - a[3]*b[6] + a[7]*b[5] + a[5]*b[7],
            a[3]*b[0] + a[0]*b[3] - a[5]*b[1] + a[1]*b[5] - a[6]*b[2] + a[2]*b[6] - a[7]*b[4] - a[4]*b[7],
            a[4]*b[0] + a[0]*b[4] - a[2]*b[1] + a[1]*b[2] + a[7]*b[3] + a[3]*b[7] + a[6]*b[5] - a[5]*b[6],
            a[5]*b[0] + a[0]*b[5] - a[3]*b[1] + a[1]*b[3] - a[7]*b[2] - a[2]*b[7] - a[6]*b[4] + a[4]*b[6],
            a[6]*b[0] + a[0]*b[6] + a[7]*b[1] + a[1]*b[7] - a[3]*b[2] + a[2]*b[3] + a[5]*b[4] - a[4]*b[5],
            a[7]*b[0] + a[0]*b[7] + a[6]*b[1] + a[1]*b[6] - a[5]*b[2] - a[2]*b[5] + a[4]*b[3] + a[3]*b[4]
        ];
        
        Multivector::new(ab)
    }
}

impl BitXor for Multivector {
    // Exterior product in R^{3,0}
    type Output = Multivector;

    fn bitxor(self, right: Multivector) -> Multivector {
        let a = self.comps;
        let b = right.comps;

        // This is from the multiplication table
        let ab: [Real; 8] = [
            a[0]*b[0],
            a[1]*b[0] + a[0]*b[1],
            a[2]*b[0] + a[0]*b[2],
            a[3]*b[0] + a[0]*b[3],
            a[4]*b[0] + a[0]*b[4] - a[2]*b[1] + a[1]*b[2],
            a[5]*b[0] + a[0]*b[5] - a[3]*b[1] + a[1]*b[3],
            a[6]*b[0] + a[0]*b[6] - a[3]*b[2] + a[2]*b[3],
            a[7]*b[0] + a[0]*b[7] + a[6]*b[1] + a[1]*b[6] - a[5]*b[2] - a[2]*b[5] + a[4]*b[3] + a[3]*b[4]
        ];
        
        Multivector::new(ab)
    }
}

impl Neg for Multivector {
    type Output = Multivector;

    fn neg(self) -> Multivector {
        let a = self.comps;
        let new_comps: [Real; 8] = [
            -a[0], -a[1], -a[2], -a[3], -a[4], -a[5], -a[6], -a[7] 
        ];
        
        Multivector::new(new_comps)
    }
}

impl fmt::Display for Multivector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        
        let blade_nms: [&str; 8] = ["", "e_x", "e_y","e_z", "e_xe_y", "e_xe_z", "e_ye_z", "e_xe_ye_z"];

        let mut is_zero = true;

        let mut output: String = String::from("");

        let coeff = self.comps[0];

        if coeff != 0. {
            output = format!("{}{}", output, coeff);
            is_zero = false
        }

        for i in 1..=7 {
            let coeff = self.comps[i];
            let blade = blade_nms[i];
            
            if coeff == 0. {
                continue;
            } else if is_zero {
                
                // If the first term isn't constant
                if coeff == -1. {
                    output = format!("{}-{}", output, blade);
                } else if coeff == 1. {
                    output = format!("{}{}", output, blade);
                } else {
                    output = format!("{}{}{}", output, coeff, blade);
                }
                is_zero = false
            } else {
            
                // Subsequent terms
                if coeff < 0. {
                    if coeff == -1. {
                        output = format!("{} - {}", output, blade);
                    } else {
                        output = format!("{} - {}{}", output, -coeff, blade);
                    }
                } else if coeff > 0. {
                    if coeff == 1. {
                        output = format!("{} + {}", output, blade);
                    } else {
                        output = format!("{} + {}{}", output, coeff, blade);
                    }
                } 
            }
        }
        if is_zero {
            output = 0.to_string()
        }

        write!(f, "{output}")
            
    }
}

