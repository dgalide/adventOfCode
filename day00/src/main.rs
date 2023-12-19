use std::fs;

fn main() {
    let file_path = "example.txt";

    let contents = fs::read_to_string(file_path);

    println!("With text:\n{:?}", contents);
}
