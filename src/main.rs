use space_alg::Multivector;

fn mv_testing() {
    
    let a = Multivector::new([-1.,-2.,3.,-5.,7.,11.,13.,-4.]);
    let b = Multivector::new([-7.,1.,16.,-24.,93.,-1.2,2.2,31.]);

    let c = a*b;
    let d = a^b;

    println!("({})*({}) = {}", a, b, c);
    println!("({})^({}) = {}", a, b, d);

}

fn unitvector_testing() {
    let a = Multivector::new_grade1([1., 2., 5.]);
    let b = a.unit_vector();

    println!("{}",a);
    println!("{}",b);
}

fn main() {

    // mv_testing();

    unitvector_testing();

}