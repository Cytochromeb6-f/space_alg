use space_alg::Multivector;

pub fn mv_testing() {
    
    let a = Multivector::new([-1.,-2.,3.,-5.,7.,11.,13.,-4.]);
    let b = Multivector::new([-7.,1.,16.,-24.,93.,-1.2,2.2,31.]);

    let c = a*b;
    let d = a^b;

    println!("({})*({}) = {}", a, b, c);
    println!("({})^({}) = {}", a, b, d);

}

fn main() {

    mv_testing();

}