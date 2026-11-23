use std::env;
use std::fs;

struct Dial {
    position: i32,
    zero_hits: u32,
}

impl Dial {
    pub fn new() -> Self {
        Dial {
            position: 50,
            zero_hits: 0,
        }
    }

    pub fn rotate_right(&mut self, amount: i32) {
        self.zero_hits += ((self.position + amount) / 100) as u32;
        self.position = (self.position + amount).rem_euclid(100);
    }

    pub fn rotate_left(&mut self, amount: i32) {
        if (self.position > 0 || amount >= 100) && self.position - amount <= 0 {
            let mut initial_value = -(self.position - amount) as u32;
            let mut count = if self.position == 0 { 0 } else { 1 };

            if initial_value >= 100 {
                initial_value /= 100_u32;
                count += initial_value;
            }

            println!("Add {}", count);
            self.zero_hits += count;
        }

        self.position = (self.position - amount).rem_euclid(100);

        println!("The dail is rotated L{amount} to {}", self.position);
    }
}

fn main() {
    dotenv::dotenv().ok();

    let file_path = match env::var("FILE_NAME") {
        Ok(file_path) => file_path,
        Err(_) => panic!("File not found"),
    };

    let file_content = fs::read_to_string(file_path).expect("Unable to read the file");

    let mut dial = Dial::new();

    parse_file(&mut dial, &file_content);

    println!("Final result: {}", dial.zero_hits);
}

fn parse_file(dial: &mut Dial, file_content: &str) {
    for line in file_content.lines() {
        let mut buffer: Vec<char> = Vec::new();
        let mut function: Option<fn(&mut Dial, i32)> = None;
        for (index, character) in line.chars().enumerate() {
            match index {
                0 => function = Some(decide_direction(&character)),
                _ => {
                    buffer.push(character);
                }
            };
        }

        let amount_string: String = buffer.into_iter().collect();

        let amount = amount_string
            .parse::<i32>()
            .expect("Unable to convert string to integer");

        if let Some(function) = function {
            function(dial, amount)
        };
    }
}

fn decide_direction(direction: &char) -> fn(&mut Dial, i32) {
    match direction {
        'L' => Dial::rotate_left,
        'R' => Dial::rotate_right,
        _ => panic!("unknown direction {direction}"),
    }
}
