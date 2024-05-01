use std::io;

fn main() {
    let values: Vec<u8> = read_input();
    values.iter().for_each(|i| {
        println!("{:?}", values[usize::from(*i - 1)]);
    });
}

fn read_input() -> Vec<u8> {
    let reader: io::Stdin = io::stdin();

    let capacity: usize = read_line(&reader).parse().unwrap();
    let mut values: Vec<u8> = Vec::with_capacity(capacity);

    for item in read_line(&reader).split_whitespace() {
        values.push(item.parse().unwrap());
    }

    values
}

fn read_line(reader: &io::Stdin) -> String {
    let mut buffer = String::new();
    reader.read_line(&mut buffer).ok().unwrap();

    return buffer.trim().to_string();
}