use math::round;

fn main() {
    
    let a: f64 = 5.0;
    let phi: f64 = (a.sqrt() + 1.0)/2.0;
    let mut input: Vec<u64> = Vec::new();
    let mut m: u64 = 0;
    let mut i: i32 = 1;

    while m < 4000000 {

       let mut n: f64 = (phi.powi(i) - (1.0 - phi).powi(i))/(a.sqrt());
       n = round::floor(n ,0);
       m = n as u64;
       if m%2 == 0 {
          input.push(m);
          }
       i = i+1;
    }
    let sum: u64 = input.iter().sum();

    println!("The sum of the even Fibbonacci numbers less than 4 million is {}",sum);
}
