fn main() {
    let mut s = String::from("hello ");
    s.push_str(", world");
    println!("s: {}", s);

    let mut x = 5;
    let y = x;
    x = 8;
    println!("x: {}, y: {}", x, y);

    let s1 = String::from("hi");
    // s1将会释放
    let s2 = s1;

    // clone会复制栈的数据和对应堆中的数据
    let s3 = s2.clone();
    println!("s2: {},s3: {}", s2, s3);
    let m  = calculate_length(&s3);
    println!("the length of {}", m);
    println!("s3: {}", s3);
    let mut t = String::from("abc");
    let z = &t[0..2];
    let k = &t[..];

    println!("z: {}, k: {}", z, k);

}
fn calculate_length(s:&String) -> usize {
    let len = s.len();
    len
}