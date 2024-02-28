use std::io;

#[derive(Debug)]
enum Choice {
    ROCK,
    PAPER,
    SCISSORS,
}
impl Choice {
    fn parse(input: String) -> Result<Choice, String> {
        if input == "r".to_string() {
            return Ok(Choice::ROCK);
        } else if input == "p".to_string() {
            return Ok(Choice::PAPER);
        } else if input == "s".to_string() {
            return Ok(Choice::SCISSORS);
        } else {
            return Err(format!("{:?} Invalid input", input));
        }
    }
}

fn main() {
    let mut input: String = String::new();

    println!("[r,p,s]:");
    io::stdin().read_line(&mut input).expect("sorry");

    let x = Choice::parse(input.trim().to_string()).expect("none");
    println!("{:?}", x);
}
