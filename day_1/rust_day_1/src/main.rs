fn main() {
    let file_contents: String = std::fs::read_to_string("input.txt").expect("Unable to find file");

    let mut counter: isize = 0;
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
            dial = move_left(&mut counter, rotation, dial);
        } else {
            dial = move_right(&mut counter, rotation, dial);
        }

        values.push(dial);

        println!("The dail is rotated {} to point at {}", line, dial);
    }

    let count = values.iter().filter(|&n| *n == 0).count() as isize;
    let final_answer = counter + count;

    println!("Count: {}", final_answer);
}

fn move_left(counter: &mut isize, left: isize, dial: isize) -> isize {
    if left <= dial {
        return dial - left;
    }

    *counter += 1;

    //something
    let new_left = left % 100;

    if new_left <= dial {
        return dial - new_left;
    }

    let movement_amount = new_left - dial;

    return 100 - movement_amount;
}

fn move_right(counter: &mut isize, right: isize, dial: isize) -> isize {
    if right + dial > 100 {
        *counter += 1;
    }

    return (dial + right) % 100;
}
