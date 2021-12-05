use std::fmt::{self, Write};

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
    data: Vec<Vec<BoardSpace>>,
}

impl Board {
    pub fn new(data: Vec<Vec<BoardSpace>>) -> Board {
        Board { data }
    }

    pub fn sum_of_unmarked(&self) -> usize {
        let flat_iter = self.data.iter().flatten();
        flat_iter.fold(0, |acc, space| match space.is_marked() {
            true => acc,
            false => acc + space.num,
        })
    }

    /// Mutates the board data to mark the space if the number of the space
    /// matches the number provided.
    pub fn mark(&mut self, num: usize) {
        for row_index in 0..self.data.len() {
            for space_index in 0..self.data[row_index].len() {
                if self.data[row_index][space_index].num == num {
                    self.data[row_index][space_index].state = BoardSpaceState::Marked;
                }
            }
        }
    }

    pub fn is_winner(&self) -> bool {
        let has_row_winner = self.has_row_winner();

        let has_column_winner = self.has_column_winner();

        has_row_winner || has_column_winner
    }

    fn has_row_winner(&self) -> bool {
        self.data
            .iter()
            .any(|row| row.iter().all(|space| space.is_marked()))
    }

    fn has_column_winner(&self) -> bool {
        for i in 0..self.data[0].len() {
            let is_winner = self.data.iter().all(|row| row[i].is_marked());
            if is_winner {
                return true;
            }
        }
        false
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row_iter in self.data.iter() {
            for space in row_iter {
                f.write_str(&space.to_string())?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}
