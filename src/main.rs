use std::io::Read;
use std::{fs::File, path::Path};

fn main() {
    println!("Part 1: {}", part_1(get_input()));
}

fn get_input() -> String {
    let path = Path::new("input.txt");
    let mut file = File::open(&path).expect(&format!("Cannot open file: {:?}", &path));
    let mut input = String::new();

    file.read_to_string(&mut input)
        .expect(&format!("Cannot read file: {:?}", &path));

    input
}

fn part_1(_input: String) -> String {
    42.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_is_resolved() -> () {
        assert_eq!("42", part_1(get_input()));
    }
}
