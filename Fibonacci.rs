fn main() {
    let mut a = 0;
    let mut b = 1;

    println!("Enter the Value of N :");

    let mut n = String::new();
    std::io::stdin()
    .read_line(&mut n)
    .expect("Failed to Take Input");

    let mut n : u128 = n.trim().parse().expect("Not a Number");

    
    if n <= 1 {
        println!("{n}");
        return;
    }

    n -= 2;

    while n > 0 {
        let t = a + b;
        a = b;
        b = t;
        n-=1;
    }

    println!("{b}");
}
