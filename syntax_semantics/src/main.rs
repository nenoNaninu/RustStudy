fn main() {
    let x:i32 = 17;
    {   
        println!("the value of x is {}",x);
        let x = 100;
        println!("the value of x is {}",x);
    }
    println!("the value of x is {}",x);

}
