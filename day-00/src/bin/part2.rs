fn main() {
    let input = include!("./input2.txt");
    let output = part1(input);
    println!("Hello, world!");
}

fn part1(input: &str) -> String {
    "todo!()".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1("");
        assert_eq!(result, "4".to_string());
    }
}
