fn pigify(s: &str) -> String {
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let first = s.chars().into_iter().nth(0).unwrap_or('h');
    let mut pig = String::new();
    let remainder: &str;
    if vowels.contains(&first) {
        pig.push('h');
        remainder = &s;
    } else {
        pig.push(first);
        remainder = &s[1..];
    }
    pig.push_str("ay");
    format!("{remainder}-{pig}")
}

fn main() {
    let pig1 = pigify("hello");
    let pig2 = pigify("first");
    let pig3 = pigify("art");
    println!("{pig1}\n{pig2}\n{pig3}");
}
