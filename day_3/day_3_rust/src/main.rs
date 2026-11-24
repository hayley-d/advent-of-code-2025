use dotenv::dotenv;

struct Bank {
    batteries: Vec<char>,
    max_1: (usize, char),
    max_2: (usize, char),
    jolts: usize,
}

fn main() {
    dotenv().ok();

    let file_path = match std::env::var("FILE_PATH") {
        Ok(file_path) => file_path,
        Err(_) => panic!("Unable to find env value"),
    };

    let file_content = std::fs::read_to_string(file_path).expect("Unable to read file");

    let mut banks: Vec<Bank> = Vec::new();

    for line in file_content.lines() {
        let batteries: Vec<char> = line.chars().collect();
        let mut max_1: (usize, char) = (0, '0');
        let mut max_2: (usize, char) = (0, '0');

        for (index, battery) in batteries.clone().into_iter().enumerate() {
            if battery > max_1.1 && index != batteries.len() - 1 {
                max_1 = (index, battery);
            }
        }

        for (index, battery) in batteries.clone().into_iter().enumerate() {
            if index > max_1.0 && battery > max_2.1 {
                max_2 = (index, battery);
            }
        }

        let jolt: String = format!("{}{}", max_1.1, max_2.1);
        let jolts: usize = jolt.parse().expect("Unable to convert string to number");

        println!("In {} jolts is {}", line, jolts);

        banks.push(Bank {
            batteries,
            max_1,
            max_2,
            jolts,
        });
    }

    let sum: usize = banks.into_iter().map(|bank| bank.jolts).sum();
    println!("Sum: {}", sum);
}
