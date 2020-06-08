// 英小文字からなる文字列 Sがあります。
// Sの長さが 
// K以下であれば、
// Sをそのまま出力してください。

// Sの長さが 
// Kを上回っているならば、先頭 
// K文字だけを切り出し、末尾に ... を付加して出力してください

fn main() {
    let k = read::<usize>();
    let s = read::<String>();

    if k >= s.len() {
        println!("{}", s);
    } else {
        println!("{}...", &s[0..k]);
    }
}


fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}