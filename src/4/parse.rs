use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

use crate::board::{Board, BoardSpace};

type DrawnNumbers = Vec<usize>;

pub(crate) fn parse_file<P>(filename: P) -> io::Result<(DrawnNumbers, Vec<Board>)>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut contents: Vec<Vec<String>> = Vec::new();
    let mut i = 0;
    for line in reader.lines() {
        let line = line?;
        let line = line.trim();
        if line.len() > 0 {
            match contents.get_mut(i) {
                Some(vec) => vec.push(line.to_string()),
                None => contents.push(vec![line.to_string()]),
            }
        } else {
            i += 1;
        }
    }

    Ok(parse_file_contents(contents))
}

fn parse_file_contents(contents: Vec<Vec<String>>) -> (DrawnNumbers, Vec<Board>) {
    // special case for the first item
    let first_line = contents[0][0].clone(); // I solemnly swear this element exists
    let drawn_numbers: DrawnNumbers = first_line
        .split(',')
        .map(|num| num.parse().unwrap())
        .collect();

    // create boards from the rest of the items
    let boards: Vec<Board> = contents
        .iter()
        .skip(1) // first item is the drawn numbers
        .map(parse_row_lines)
        .map(Board::new)
        .collect();

    (drawn_numbers, boards)
}

/// Parses lines representing the board into structured rows
fn parse_row_lines(lines: &Vec<String>) -> Vec<Vec<BoardSpace>> {
    lines.iter().map(parse_row).collect()
}

/// Parse a line read from the data file into a structured row
fn parse_row(line: &String) -> Vec<BoardSpace> {
    let spaces: Vec<BoardSpace> = line
        .split_ascii_whitespace()
        .map(|num| num.parse::<usize>().unwrap()) // for this problem, ignore possibility of invalid numbers
        .map(|num| BoardSpace::new(num))
        .collect();

    spaces.into()
}

#[cfg(test)]
mod tests {
    use crate::board::BoardSpace;

    use super::parse_row;

    #[test]
    fn test_parse_row() {
        let input = "88 67 20 19 15".to_string();
        let actual = parse_row(&input);

        let expected: Vec<BoardSpace> = vec![88, 67, 20, 19, 15]
            .iter()
            .map(|i| *i as usize)
            .map(BoardSpace::new)
            .collect::<Vec<BoardSpace>>();

        assert_eq!(expected, actual);
    }
}
