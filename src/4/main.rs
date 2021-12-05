use std::io;

use board::Board;

use crate::parse::parse_file;

mod board;
mod parse;

fn main() -> io::Result<()> {
    let (drawn_numbers, mut boards) = parse_file("src/4/data.txt")?;

    let (last_num, winning_board) = get_last_winning_board(drawn_numbers, &mut boards);

    let final_result = last_num * winning_board.sum_of_unmarked();
    println!("Final result: {}\n", final_result);
    println!("{}", winning_board);

    Ok(())
}

fn get_last_winning_board(drawn_numbers: Vec<usize>, boards: &mut Vec<Board>) -> (usize, Board) {
    for num in drawn_numbers {
        let mut board_index = 0;
        let mut num_boards = boards.len();

        while board_index < num_boards {
            boards[board_index].mark(num);

            if boards[board_index].is_winner() {
                // in-place mutation of the vector and associate bookeeping
                let winning_board = boards.remove(board_index);
                num_boards -= 1;

                if boards.is_empty() {
                    return (num, winning_board);
                }
            } else {
                board_index += 1;
            }
        }
    }

    unreachable!();
}
