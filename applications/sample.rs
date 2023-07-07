fn main() {
    let a = vec![1,2,3,5];
    for v in a {
        println!("Good morning{}", match v>1 {
            true => format!(", for the {}th time", v),
            false => "".to_string()
        })
    };
}
