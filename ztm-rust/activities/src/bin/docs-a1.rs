
fn main() {
    let mut v: Vec<String> = vec![String::from("Hello world")];
    let mut s = v.remove(0);
    s.push('!');
    // println!("{s}");
    v.push(String::from("Hello world"));
    println!("{:?}", v)
}   