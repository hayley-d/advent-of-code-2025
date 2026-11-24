use dotenv::dotenv;
use fancy_regex::Regex;

#[derive(Clone, Debug)]
struct Id<'a>(&'a str, &'a str);

fn main() {
    dotenv().ok();

    let file_path = match std::env::var("FILE_PATH") {
        Ok(file_path) => file_path,
        Err(_) => panic!("No env variable found"),
    };

    let file_content = std::fs::read_to_string(file_path).expect("Unable to read file");

    let mut ids: Vec<Id> = Vec::new();

    for id in file_content.split(",") {
        let parts: Vec<&str> = id.split("-").collect();
        ids.push(Id(parts.first().unwrap(), parts.get(1).unwrap()));
    }

    let regex = Regex::new(r"^(\d+)\1+$").unwrap();

    let mut invalid_ids: Vec<usize> = Vec::new();

    ids.clone().into_iter().for_each(|id| {
        let start: usize =
            id.0.trim()
                .parse()
                .expect("Unable to convert string to usize");

        let end: usize =
            id.1.trim()
                .parse()
                .expect("Unable to convert string to usize");

        for number in start..end + 1 {
            let string_number: String = format!("{number}");

            if regex.is_match(&string_number).unwrap() {
                println!("Invalid: {}", string_number);
                invalid_ids.push(number);
            }
        }
    });

    let sum: usize = invalid_ids.into_iter().sum();

    println!("Sum: {sum}");
}
