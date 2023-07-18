use std::fmt;   
use std::ops::{Add, Sub, Mul, Neg};


type Real = f32; 

#[derive(PartialEq, Clone, Copy)]
pub struct Multivector {
    blades: [Real; 8]
}

impl Multivector {
    fn new(blades: [Real; 8]) -> Multivector {
        Multivector {blades: blades}
    }
    
    fn new_grade1(comp: [Real; 3]) -> Multivector {
        Multivector::new([0., comp[0], comp[1], comp[2], 0., 0., 0., 0.])
    }

    fn scaled(self, scalar: Real) -> Multivector {
        Multivector::new([
            scalar*self.blades[0], scalar*self.blades[1], scalar*self.blades[2], scalar*self.blades[3],
            scalar*self.blades[4], scalar*self.blades[5], scalar*self.blades[6], scalar*self.blades[7]
        ])
    }
    
    fn complement(self) -> Multivector {
        // The orthogonal complement of a multivector
        Multivector::new([
            -self.blades[7], -self.blades[6], self.blades[5], -self.blades[4],
            self.blades[3], -self.blades[2], self.blades[1], self.blades[0]
        ])
    }
}


impl Add for Multivector {
    // Sum of two multivectors
    type Output = Multivector;

    fn add(self, right: Multivector) -> Multivector {
        Multivector::new([
            self.blades[0]+right.blades[0], self.blades[1]+right.blades[1],
            self.blades[2]+right.blades[2], self.blades[3]+right.blades[3],
            self.blades[4]+right.blades[4], self.blades[5]+right.blades[5],
            self.blades[6]+right.blades[6], self.blades[7]+right.blades[7]
        ])
    }
}

impl Sub for Multivector {
    // Difference between two multivectors
    type Output = Multivector;

    fn sub(self, right: Multivector) -> Multivector {
        Multivector::new([
            self.blades[0]-right.blades[0], self.blades[1]-right.blades[1],
            self.blades[2]-right.blades[2], self.blades[3]-right.blades[3],
            self.blades[4]-right.blades[4], self.blades[5]-right.blades[5],
            self.blades[6]-right.blades[6], self.blades[7]-right.blades[7]
        ])
    }
}
// impl AddAssign

impl Mul for Multivector {
    // Clifford product in R^{3,0}
    type Output = Multivector;

    fn mul(self, right: Multivector) -> Multivector {
        let a = self.blades;
        let b = right.blades;

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

impl Neg for Multivector {
    type Output = Multivector;

    fn neg(self) -> Multivector {
        let a = self.blades;
        let new_blades: [Real; 8] = [
            -a[0], -a[1], -a[2], -a[3], -a[4], -a[5], -a[6], -a[7] 
        ];
        
        Multivector::new(new_blades)
    }
}

impl fmt::Display for Multivector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        
        let blade_nms: [&str; 8] = ["", "e_x", "e_y","e_z", "e_xe_y", "e_xe_z", "e_ye_z", "e_xe_ye_z"];

        let mut is_zero = true;

        let mut output: String = String::from("");

        let coeff = self.blades[0];

        if coeff != 0. {
            output = format!("{}{}", output, coeff);
            is_zero = false
        }

        for i in 1..=7 {
            let coeff = self.blades[i];
            let blade = blade_nms[i];
            
            if coeff == 0. {
                continue;
            } else if is_zero {
                
                // If the first term isn't constant
                if coeff == -1. {
                    output = format!("{}-{}", output, blade);
                } else if coeff == 1. {
                    output = format!("{}{}", output, blade);
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

fn cliff_testing() {
    
    let a = Multivector::new([-1.,-2.,3.,-5.,7.,11.,13.,-4.]);
    let b = Multivector::new([-7.,1.,16.,-24.,93.,-1.2,2.2,31.]);
    let c = a*b;

    println!("({})*({}) = {}", a, b, c)

}

fn main() {

    cliff_testing();
}