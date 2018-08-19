// fn main() {
//     let x:i32 = 17;
//     {
//         println!("the value of x is {}",x);
//         let x = 100;
//         println!("the value of x is {}",x);
//     }
//     println!("the value of x is {}",x);
//     print_number(x);
//     let two_hearts = '💕';
//     println!("{}",two_hearts);
// }

// fn print_number(x:i32){
//     println!("x is {}",x);
// }

fn main() {
    let mut a = [0, 1, 2, 3, 4];
    println!("{}", a[1]);
    a[1] = 4;
    println!("{}", a[1]);
}
