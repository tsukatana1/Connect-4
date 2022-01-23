mod game;

use std::io;

use game::{Game, Color, Player};

//This function is to get the current player using a mutable Player slice.
fn get_player(players: &mut [Player], color: Color) -> &mut Player {
    let pos = players.iter().position(|x| x.1 == color).unwrap();

    players.get_mut(pos).unwrap()
}

//A wrapper for `Game#create_board` because of borrowing issues.
fn create_board(game: &Game) -> String {
    game.create_board()
}

//Where all the magic happens.
fn main() {
    let mut game = Game::new();

    print!("\x1B[2J\x1B[1;1H\n");
    println!("{}", create_board(&game));

    loop {
        let turn = game.turn.clone();
        let current_player = get_player(&mut game.players, turn);

        println!("{}", current_player);

        let mut column = String::new();
        io::stdin().read_line(&mut column).unwrap();

        let i = (column.trim().parse::<u16>().unwrap() - 1) as usize;

        if (game.board[i] & game.positions.iter().sum::<u16>()) == game.positions.iter().sum::<u16>() {
            println!("Cannot stack, since the column is full.");
            continue;
        }

        let mut next_left: u16 = 0;
        for x in 0..8 {
            if game.board[i] >> x == 0 {
                next_left = 1 << x;
                break;
            }
        } 

        //println!("{}", game.create_board());

        current_player.0[i] += next_left;
        game.board[i] += 1 + game.board[i];

        if won(&current_player.0) {
            print!("\x1B[2J\x1B[1;1H\n");
            println!("Player {} won the game!\n", current_player);
            println!("{}", create_board(&game));

            return;
        }

        if tie(&game.positions, &game.board) {
            print!("\x1B[2J\x1B[1;1H\n");
            println!("It's a tie!\n");
            println!("{}", create_board(&game));

            return;
        }

        game.switch();

        print!("\x1B[2J\x1B[1;1H\n");
        println!("{}", create_board(&game));
    }
}

fn won(color: &Vec<u16>) -> bool {
    for i in 0..7 {
        for j in 0..4 {
            if (color[i] >> j) == 15 {
                return true;
            }
            else if i < 5 && ((color[i] >> j) & 1) != 0 && ((color[i + 1] >> j) & 1) != 0 && ((color[i + 2] >> j) & 1) != 0 && ((color[i + 3] >> j) & 1) != 0 {
                return true;
            }
            else if i < 5 && (((color[i] >> j) & 1) != 0 && ((color[i + 1] >> j) & 2) != 0 && ((color[i + 2] >> j) & 4) != 0 && ((color[i + 3] >> j) & 8) != 0 ||
            ((color[i] >> j) & 8) != 0 && ((color[i + 1] >> j) & 4) != 0 && ((color[i + 2] >> j) & 2) != 0 && ((color[i + 3] >> j) & 1) != 0) {
                return true;
            }
        }
    }

    false
}

fn tie(positions: &Vec<u16>, board: &Vec<u16>) -> bool {

    if board.iter().all(|column| *column == positions.iter().sum::<u16>()) {
        return true;
    }

    false
}
