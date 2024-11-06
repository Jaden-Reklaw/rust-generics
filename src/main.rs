use num_traits::{Float, ToPrimitive};

//Version 1
//We can pass in both f32 and f64
fn solve_hypothenuse_v1<T: Float>(a: T, b: T) -> f64 {
    
    let a_f64 = a.to_f64().unwrap();
    let b_f64 = b.to_f64().unwrap();

    (a_f64.powi(2) + b_f64.powi(2)).sqrt()
}

//Version 2
//We can pass in any type of numbers
fn solve_hypothenuse_v2<T: ToPrimitive>(a: T, b: T) -> f64 {
    
    let a_f64 = a.to_f64().unwrap();
    let b_f64 = b.to_f64().unwrap();

    (a_f64.powi(2) + b_f64.powi(2)).sqrt()
}

//What are generics
fn main() {
    let a: f32 = 3.0;
    let b: f32 = 4.0;
    println!("{}", solve_hypothenuse_v1::<f32>(a, b));

    let c: f64 = 3.0;
    let d: f64 = 4.0;
    //This is using type inference
    println!("{}", solve_hypothenuse_v1(c, d));
}
