use std::fs;

fn main(){
    let input = String::from_utf8(fs::read("input.txt").unwrap()).unwrap();
    let (mut max_calories, mut current_calories) = (0, 0);

    print!("{}", max_calories);
}
