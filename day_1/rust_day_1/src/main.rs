fn main() {
    let file_contents: String = std::fs::read_to_string("input.txt").expect("Unable to find file");

    let mut dial: isize = 50;
    let mut values: Vec<isize> = Vec::new();

    println!("The dial starts by pointing at {}", dial);

    for line in file_contents.lines() {
        let chars: Vec<char> = line.chars().collect();
        let is_left = match chars.get(0) {
            Some(c) => *c == 'L',
            None => std::process::exit(1),
        };
        let rotation: isize = (&line[1..]).parse().unwrap();

        if is_left {
            dial = move_left(rotation, dial);
        } else {
            dial = move_right(rotation, dial);
        }

        values.push(dial);

        println!("The dail is rotated {} to point at {}", line, dial);
    }

    let count = values.iter().filter(|&n| *n == 0).count();
    println!("Count: {}", count);
}

fn move_left(left: isize, dial: isize) -> isize {
    if left <= dial {
        return dial - left;
    }

    let temp = left % 100;

    return dial + (100 - temp);
}

fn move_right(right: isize, dial: isize) -> isize {
    return (dial + right) % 100;
}
