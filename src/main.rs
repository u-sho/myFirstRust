use regex::Regex;

fn main() {
    let reg = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    println!("Did our date match? {}", reg.is_match("2020-08-23"));
}
