fn main() {
    let a = read::<String>().parse::<i32>().unwrap();
    let bc = read::<String>();
    let b_c = bc.split(' ').map(|x| x.parse::<i32>().unwrap());
    let mut sum = a;
    for b in b_c {
        sum += b;
    }
    let s = read::<String>();
    println!("{} {}", sum, s);
}

pub fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}