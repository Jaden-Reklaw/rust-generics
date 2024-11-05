fn solve_hypothenuse(a: f64, b: f64) -> f64 {
    (a.powi(2) + b.powi(2)).sqrt()
}
//What are generics
fn main() {
    let a: f32 = 3.0;
    let b: f64 = 4.0;

    println!("{}", solve_hypothenuse(a, b));
}
