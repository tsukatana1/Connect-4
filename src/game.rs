use std::fmt::{Display, Formatter, Result as fmtResult};

//The enumeration `Color`, which holds both colours of connect four, red and yellow.
//Clone is derived to clone the current turn in the `Game` struct.
#[derive(Debug, PartialEq, Clone)]
pub enum Color {
    Red, Yellow
}

//The struct `Player`. Sort of a wrapper for `Color`, containing the positions of the player, and their colour.
#[derive(Debug, Clone)]
pub struct Player(pub Vec<u16>, pub Color);

/* The struct `Game`, which contains the current data of the game.
players: a fixed size slice containing 2 `Player` structs.
board: a vector of u16 (bitwise) of the connect 4 positions.
turn: a `Color` which decides who the current player is.
positions: a vector of bitwise of the columns.*/
#[derive(Debug, Clone)]
pub struct Game {
    pub players: [Player; 2],
    pub board: Vec<u16>,
    pub turn: Color,
    pub positions: Vec<u16>
}

//We want to implement `Display` for `Player`, to show the value of the current player.
impl Display for Player {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmtResult {
        write!(f, "Player {}, choose a column to drop your value", match self.1 {
            Color::Red => "\x1B[31mRed\x1B[0m",
            Color::Yellow => "\x1B[33mYellow\x1B[0m"
        })
    }
}

//Functions for `Game`.
impl Game {
    //new() makes a new instance of `Game`, with the default bitwise tables.
    pub fn new() -> Self {
        let players = [
            Player(vec![0, 0, 0, 0, 0, 0, 0, 0], Color::Red),
            Player(vec![0, 0, 0, 0, 0, 0, 0, 0], Color::Yellow)
        ];

        Self { 
            turn: Color::Red,
            players: players, 
            board: vec![0, 0, 0, 0, 0, 0, 0, 0],
            positions: vec![1 << 0, 1 << 1, 1 << 2, 1 << 3, 1 << 4, 1 << 5, 1 << 6, 1 << 7]
        }
    }
    
    //switch() switches the current colour of `self.turn`.
    pub fn switch(&mut self) {
        match self.turn {
            Color::Red => self.turn = Color::Yellow,
            Color::Yellow => self.turn = Color::Red
        }
    }

    //create_board() creates the board to display.
    pub fn create_board(&self) -> String {
        let mut string_board = String::new();

        for i in (0..8).rev() {
            for j in 0..8 {
                if (self.positions[i] & self.players[0].0[j]) != 0 {
                    string_board += "  \x1B[31mⓄ\x1B[0m  |";
                }
                else if (self.positions[i] & self.players[1].0[j]) != 0 {
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
