use std::io;
use rand::prelude::*;

fn lsb(bb: i16) -> i8 {
   return ( (bb & -bb) - 1 ).count_ones();
}

pub struct State {
    position: [i16; 2], // = [0, 0];
    turn: i8, // = 1;
    movestack: Vec<i8>,
}

impl State {
    pub fn play(&mut self, i: i8) {
        // n ^ (1 << k) is a binary XOR where you flip the kth bit of n
        if (self.turn == 1){
            self.position[0] |= (1 << i);}
        else{
            self.position[1] |= (1 << i);}
        self.turn = -self.turn;
        self.movestack.push(i);
    }

    // do not unplay on root
    pub fn unplay(&mut self) {
        let prevmove: i8 = self.movestack.pop();
        if (self.turn == 1){
            self.position[1] &= !(1 << prevmove);}
        else{
            self.position[0] &= !(1 << prevmove);}
        self.turn = -self.turn;
    }

    pub fn pos_filled(&self, i: i8) -> bool {
        if (((self.position[0] | self.position[1]) & (1 << i)) != 0) {
            return true;
        } else {
            return false;
        }
    }

    // only valid to use if pos_filled() returns true, true = x, false = y
    pub fn player_at(&self, i: i8) -> bool {
        if ((self.position[0] & (1 << i)) != 0) {
            return true;
        } else {
            return false;
        }
    }

    pub fn is_full(&self) -> bool {
        for i in 0..9 {
            if (!self.pos_filled(i)){
                return false;}
        }
        return true;
    }

    pub fn evaluate(&self) -> i8 {
        // check first diagonal
        if (self.pos_filled(0) && self.pos_filled(4) && self.pos_filled(8)) {
            if (self.player_at(0) == self.player_at(4) && self.player_at(4) == self.player_at(8)) {
                if (self.player_at(0)){
                    return 1;}
                else{
                    return -1;}
            }
        }
        // check second diagonal
        if (self.pos_filled(2) && self.pos_filled(4) && self.pos_filled(6)) {
            if (self.player_at(2) == self.player_at(4) && self.player_at(4) == self.player_at(6)) {
                if (self.player_at(2)){
                    return 1;}
                else{
                    return -1;}
            }
        }
        // check rows
        for i in 0..3 {
            if (self.pos_filled(i * 3) && self.pos_filled(i * 3 + 1) && self.pos_filled(i * 3 + 2)) {
                if (self.player_at(i * 3) == self.player_at(i * 3 + 1) && self.player_at(i * 3 + 1) == self.player_at(i * 3 + 2)) {
                    if (self.player_at(i * 3)){
                        return 1;}
                    else{
                        return -1;}
                }
            }
        }
        // check columns
        for i in 0..3 {
            if (self.pos_filled(i) && self.pos_filled(i + 3) && self.pos_filled(i + 6)) {
                if (self.player_at(i) == self.player_at(i + 3) && self.player_at(i + 3) == self.player_at(i + 6)) {
                    if (self.player_at(i)){
                        return 1;}
                    else{
                        return -1;}
                }
            }
        }
        return 0;
    }

    pub fn pass_turn(&self) {
        self.turn = -self.turn;
    }

    pub fn show(&self) {
        for x in 0..3 {
            for y in 0..3 {
                if (self.pos_filled(x * 3 + y)) {
                    if (self.player_at(x * 3 + y)){
                        println!("X ");}
                    else{
                        println!("0 ");}
                } else{
                    println!(". ");}
            }
            println!("\n");
        }
        println!("\n");
    }

    pub fn is_game_over(&self) -> bool {
        return (self.evaluate() != 0) || self.is_full();
    }

    pub fn num_legal_moves(&self) {
        return 9 - (self.position[0] | self.position[1]).count_ones();
    }

    pub fn legal_moves(&self) -> Vec<i8> {
        let moves: Vec::<i8>;
        moves.reserve(9);
        let bb: i16 = !(self.position[0] | self.position[1]) & 0b111111111;
        while bb {
            moves.push_back(lsb(bb));
            bb &= bb - 1;  // clear the least significant bit set
        }
        return moves;
    }

    pub fn random_play(&mut self) {
        let moves: Vec::<i8> = self.legal_moves();
        self.play(moves[rand::random::<i32>() % moves.size()]);
    }

    pub fn heuristic_value(&self) -> i8 {
        return 0;
    }
}

pub struct Istus {
    node: State,
    nodes: i32,
    timeLimit: i64,
}

impl Istus {
    fn negamax(
        &mut self,
        colour: i8, 
        a: i16, 
        b: i16) -> i16  //WORKING
    {
        if (self.node.is_game_over()) {
            self.nodes += 1;
            return colour * self.node.evaluate();
        }
        let score: i16;

        // node.pass_turn();                              // MAKE A NULL MOVE
        // score = -negamax(depth - 3, -colour, -b, -a);  // PERFORM A LIMITED SEARCH
        // node.pass_turn();                              // UNMAKE NULL MOVE
        // if (score > a)
        //     a = score;
        // if (a >= b)
        //     return a;

        for m in self.node.legal_moves() {
            self.node.play(m);
            score = -self.negamax(-colour, -b, -a);
            self.node.unplay();

            if (score >= b) {
                return b;
            }
            if (score > a) {
                a = score;
            }
        }

        return a;
    }

    pub fn engine_move(&mut self)  //WORKING
    {
        let bestmove: i8;
        let bestcase: i8 = -2;
        let score: i8;
        self.reset_nodes();
        for m in self.node.legal_moves() {
            self.node.play(m);
            score = -self.negamax(self.node.turn, -2, 2);
            self.node.unplay();
            if bestcase < score {
                bestcase = score;
                bestmove = m;
            }
        }
        println!("ISTUS:\n");
        println!("{} nodes processed.\n", nodes);
        println!("Istus win prediction: {}%\n", ((1 + bestcase) * (50)));
        node.play(bestmove);
    }

    pub fn reset_nodes(&mut self) {
        self.nodes = 0;
    }

    pub fn show_result(&self) {
        let r = self.node.evaluate();
        if (r == 0){
            println!("1/2-1/2\n");}
        else if (r == 1){
            println!("1-0\n");}
        else{
            println!("0-1\n");}
    }

    pub fn get_player_move() -> i8 {
        let legals: Vec::<i8> = self.node.legal_moves();
        pos: i8;
        io::stdin()
            .read_line(&mut pos)
            .expect("Failed to read line");
        while (legals.all(|&x| x != pos + 1)) {
            node.show();
            std::cin >> pos;
        }
        return pos - 1;
    }
}

fn main(){
    println!("Hello World!");
}