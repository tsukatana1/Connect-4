mod game;
use game::Game;
use std::io;

fn main() {

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

    let mut game = Game::new();

    print!("\x1B[2J\x1B[1;1H\n");
    println!("{}", game.create_board());

    loop {
        
        let current_player = match game.turn {
            game::Color::Red => &mut game.players[0],
            game::Color::Yellow => &mut game.players[1]
        };

        println!("Player {}, choose a column to drop your piece in.", current_player);

        let mut column = String::new(); 

        io::stdin().read_line(&mut column).unwrap();

        let column = (column.trim().parse::<u16>().unwrap() - 1) as usize;

        if (game.board[column] & positions.iter().sum::<u16>()) == positions.iter().sum::<u16>() {
            println!("You cannot stack another piece in this column as it is full.");
            continue;
        }

        let mut next_left: u16 = 0;
        for i in 0..8 {
            if game.board[column] >> i == 0 {
                next_left = 1 << i;
                break;
            }
        }

        current_player.values[column] += next_left;
        game.board[column] += 1 + game.board[column];
        
        if won(&current_player.values) {
            print!("\x1B[2J\x1B[1;1H\n");
            println!("Player {} won the game!\n", current_player);
            println!("{}", game.create_board());

            return;
        }

        if tie(&game.board) {
            print!("\x1B[2J\x1B[1;1H\n");
            println!("It's a tie!\n");
            println!("{}", game.create_board());

            return;
        }

        game.advance();

        print!("\x1B[2J\x1B[1;1H\n");
        println!("{}", game.create_board());

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

fn tie(board: &Vec<u16>) -> bool {

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

    if board.iter().all(|column| *column == positions.iter().sum::<u16>()) {
        return true;
    }

    false
}