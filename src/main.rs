use std::io::{Write, stdout, stdin};

struct Game {
    grid: [char; 9],
    players: [char; 2]
}

impl Game {
    fn draw(&self) {
        println!("");
        for i in 0..3 {
            let offset = 3 * i;
            println!("  {} │ {} │ {} ", self.grid[offset], self.grid[offset + 1], self.grid[offset + 2]);
            if i != 2 {
                println!(" ───┼───┼───");
            } else {
                println!("");
            }
        }
    }

    fn is_available (&self, position: usize) -> bool {
        if position < 1 || position > 9 {
            return false;
        }
        let token: char = self.grid[position - 1];
        return !(token == self.players[0] || token == self.players[1])
    }
    
    fn is_won(&self) -> bool {
        for token in self.players {
            for row in 0..3 {
                if self.grid[row] == token && self.grid[row + 1] == token && self.grid[row + 2] == token {return true;}
            }
            for col in 0..3 {
                if self.grid[0] == token && self.grid[3 + col] == token && self.grid[6 + col] == token {return true;}
            }
            if self.grid[0] == token && self.grid[4] == token && self.grid[8] == token ||  self.grid[2] == token && self.grid[4] == token && self.grid[6] == token {return true;}
        }
        return false;
    }

    fn take_turn(&mut self, player: usize) {
        println!("It's now player {}'s turn", player + 1);

        let position: usize = loop {
            print!("Where do you want to play? >>> ");
            let _ = stdout().flush();

            let mut input = String::new();
            match stdin().read_line(&mut input) {
                Ok(_n) => {}
                Err(_error) => {continue;}
            }
            let num: usize = match input.trim().parse() {
                Ok(value) => value,
                Err(_) => {
                    println!(" X: Please enter a valid number");
                    continue;
                }
            };

            if self.is_available(num) {
                break num;
            } else {
                println!(" X: Please choose a available position to play");
            }

        };

        self.grid[position - 1] = self.players[player];
        self.draw();
    }

    fn win(&self, player: usize) {
        println!("Player {} has won the game", player + 1);
    }
}

fn main() {
    let mut game = Game {
        grid : ['1', '2', '3', '4', '5', '6', '7', '8', '9'],
        players : ['X', 'O']
    };
    game.draw();
    let mut player = 0;
    loop {
        game.take_turn(player);
        player = if player == 0 {1} else {0};

        if game.is_won() {break;}
    }
    game.win(player)
}
