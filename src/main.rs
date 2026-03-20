fn main() {
    let mut v = vec![100, 37, 52];
    for i in &mut v {
        println!("{i}");
    }
    use std::collections::HashMap;
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    for (key,value) in &scores {
        println!("{key}: {value}");
        
    }
}
