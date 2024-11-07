use num_traits::{Float, ToPrimitive};

//Version 1
//We can pass in both two f32 or two f64
fn solve_hypothenuse_v1<T: Float>(a: T, b: T) -> f64 {
    
    let a_f64 = a.to_f64().unwrap();
    let b_f64 = b.to_f64().unwrap();

    (a_f64.powi(2) + b_f64.powi(2)).sqrt()
}

//Version 1
//We can pass in both f32 and f64
//You have to have two generic types
fn solve_hypothenuse_v2<T: Float, U: Float>(a: T, b: U) -> f64 {
    
    let a_f64 = a.to_f64().unwrap();
    let b_f64 = b.to_f64().unwrap();

    (a_f64.powi(2) + b_f64.powi(2)).sqrt()
}

//Version 3
//We can pass in any type of numbers
fn solve_hypothenuse_v3<T: ToPrimitive, U: ToPrimitive>(a: T, b: U) -> f64 {
    
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

    let e:i32 = 3;
    let f:i32 = 4;

    //This is using type inference
    println!("{}", solve_hypothenuse_v1(c, d));

    //Passing in two different types
    println!("{}", solve_hypothenuse_v2(a, d));
    println!("{}", solve_hypothenuse_v2(c, b));

    //Passing in any type of numbers
    println!("{}", solve_hypothenuse_v3(a, b));
    println!("{}", solve_hypothenuse_v3(a, c));
    println!("{}", solve_hypothenuse_v3(e, f));
    println!("{}", solve_hypothenuse_v3(e, c));
}
