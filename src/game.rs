use std::fmt::{Display, Formatter, Result as fmtResult};

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Color {
    Red,
    Yellow
}

//player.
pub struct Player {
    pub values: Vec<u16>,
    pub color: Color
}


impl Display for Player {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmtResult {
        write!(f, "{}", match self.color {
            Color::Red => "Red",
            Color::Yellow => "Yellow"
        })
    }
}


pub struct Game {
    pub players: Vec<Player>,
    pub board: Vec<u16>,
    pub turn: Color,
}

impl Game {
    pub fn new() -> Self {
        let players = vec![
            Player { values: vec![0, 0, 0, 0, 0, 0, 0, 0], color: Color::Red },
            Player { values: vec![0, 0, 0, 0, 0, 0, 0, 0], color: Color::Yellow }
        ];

        Self { 
            turn: Color::Red,
            players: players, 
            board: vec![0, 0, 0, 0, 0, 0, 0, 0],
        }
    }

    pub fn advance(&mut self) {
        match self.turn {
            Color::Red => self.turn = Color::Yellow,
            Color::Yellow => self.turn = Color::Red
        }
    }

    pub fn create_board(&self) -> String {
        // starting from bottom to top, the positions represent a position in the column
        let positions = vec![
            1 << 0,
            1 << 1,
            1 << 2,
            1 << 3,
            1 << 4,
            1 << 5,
            1 << 6,
            1 << 7,
        ];
        let mut string_board = String::from("  1  |  2  |  3  |  4  |  5  |  6  |  7  |  8  |\n------------------------------------------------\n");

        for i in (0..8).rev() {
            for j in 0..8 {

                if (positions[i] & self.players[0].values[j]) != 0 {
                    string_board += "  \x1B[31mⓄ\x1B[0m  |";
                }
                else if (positions[i] & self.players[1].values[j]) != 0 {
                    string_board += "  \x1B[33mⓄ\x1B[0m  |";
                }
                else {
                    string_board += "     |";
                }
            }
            string_board += "\n----- ----- ----- ----- ----- ----- ----- -----\n";
        }
        
        string_board
    }

}
