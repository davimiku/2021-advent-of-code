use std::fmt::{self, Write};

use array2d::Array2D;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum BoardSpaceState {
    Marked,
    NotMarked,
}

impl Default for BoardSpaceState {
    fn default() -> Self {
        Self::NotMarked
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct BoardSpace {
    num: usize,
    state: BoardSpaceState,
}

impl BoardSpace {
    pub fn new<N>(num: N) -> BoardSpace
    where
        N: Into<BoardSpace>,
    {
        num.into()
    }

    fn is_marked(&self) -> bool {
        match self.state {
            BoardSpaceState::Marked => true,
            BoardSpaceState::NotMarked => false,
        }
    }
}

impl fmt::Display for BoardSpace {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_marked() {
            f.write_fmt(format_args!("*{} ", self.num))
        } else {
            f.write_fmt(format_args!(" {} ", self.num))
        }
    }
}

impl Into<BoardSpace> for usize {
    fn into(self) -> BoardSpace {
        BoardSpace {
            num: self,
            state: BoardSpaceState::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Board {
    data: Array2D<BoardSpace>,
}

impl Board {
    pub fn new(spaces: Vec<Vec<BoardSpace>>) -> Board {
        let data = Array2D::from_rows(&spaces);

        Board { data }
    }

    pub fn sum_of_unmarked(&self) -> usize {
        self.data
            .elements_row_major_iter()
            .fold(0, |acc, space| match space.is_marked() {
                true => acc,
                false => acc + space.num,
            })
    }

    /// Mutates the board data to mark the space if the number of the space
    /// matches the number provided.
    pub fn mark(&mut self, num: usize) {
        let rows = self.data.as_rows();

        for (row_index, row) in rows.iter().enumerate() {
            for (space_index, space) in row.iter().enumerate() {
                if space.num == num {
                    self.data[(row_index, space_index)].state = BoardSpaceState::Marked;
                }
            }
        }
    }

    pub fn is_winner(&self) -> bool {
        let has_row_winner = is_winner(self.data.rows_iter());
        let has_column_winner = is_winner(self.data.columns_iter());

        has_row_winner || has_column_winner
    }
}

fn is_winner<'a>(mut iter: impl Iterator<Item = impl Iterator<Item = &'a BoardSpace>>) -> bool {
    iter.any(|mut space_iter| space_iter.all(|space| space.is_marked()))
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row_iter in self.data.rows_iter() {
            for space in row_iter {
                f.write_str(&space.to_string())?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}
