use dotenv::dotenv;

pub struct Matrix {
    matrix: Vec<Vec<char>>,
}

impl Matrix {
    pub fn push(&mut self, values: Vec<char>) {
        self.matrix.push(values);
    }

    pub fn print(&self) -> String {
        let mut output = String::new();

        for row in &self.matrix {
            let line: String = row.iter().collect();
            output.push_str(&line);
            output.push('\n');
        }

        output
    }

    pub fn check_neighbours(&self, row: usize, col: usize) -> bool {
        let kernel: Vec<(isize, isize)> = vec![
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 0),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];

        let mut paper_count: usize = 0;

        for coord in kernel.into_iter() {
            let row = row as isize + coord.0;
            let col = col as isize + coord.1;

            let character: char = if row < 0 || col < 0 {
                '.'
            } else {
                match self.matrix.get(row as usize) {
                    Some(matrix_row) => match matrix_row.get(col as usize) {
                        Some(character) => *character,
                        None => '.',
                    },
                    None => '.',
                }
            };

            if character == '@' || character == 'x' {
                paper_count += 1
            }
        }

        paper_count <= 4
    }

    pub fn evaluate(&mut self) {
        for (x, row) in self.matrix.clone().into_iter().enumerate() {
            for (y, character) in row.into_iter().enumerate() {
                if character == '@' && self.check_neighbours(x, y) {
                    self.matrix[x][y] = 'x';
                }
            }
        }
    }

    pub fn count_character(&self, target: char) -> usize {
        let mut count: usize = 0;
        for row in &self.matrix {
            for character in row {
                if *character == target {
                    count += 1;
                }
            }
        }

        count
    }

    pub fn convert_character(&mut self, target: char, replacement: char) {
        for (x, row) in self.matrix.clone().into_iter().enumerate() {
            for (y, character) in row.into_iter().enumerate() {
                if character == target {
                    self.matrix[x][y] = replacement;
                }
            }
        }
    }
}

fn main() {
    dotenv().ok();

    let file_path = match std::env::var("FILE_PATH") {
        Ok(file_path) => file_path,
        Err(_) => panic!("Unable to find FILE_PATH"),
    };

    let file_content: String = std::fs::read_to_string(file_path).expect("unable to read file");

    let mut matrix: Matrix = parse_file(&file_content);

    let mut changes: usize = 1;
    let mut total: usize = 0;

    while changes != 0 {
        matrix.evaluate();
        changes = matrix.count_character('x');
        total += changes;
        //println!("{}", matrix.print());
        matrix.convert_character('x', '.');
    }

    println!("Count: {total}");
}

pub fn parse_file(file_content: &str) -> Matrix {
    let mut matrix: Matrix = Matrix { matrix: Vec::new() };

    for line in file_content.lines() {
        let line_chars: Vec<char> = line.trim().chars().collect();
        matrix.push(line_chars);
    }

    matrix
}
