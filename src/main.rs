use std::io;


fn main() {
    let mut board: [char; 9] = [' '; 9];
    let mut player : char = 'X';
    let mut input : usize;
    let mut moves : u8 = 0;
    loop {
        println!("{} to play", player);
        outputboard(board);
        input = readlines();

        if  input > 8 {
            println!("error above or below limit");
            continue;
        }
        
        if checkinput(player, board, input) {
            board[input] = player;
            if wincheck(player,board) {
                outputboard(board);
                board = win(player);
                moves = 0;
            } else {
                if moves >=8 { board = win('D'); moves = 0; continue; } 
                if player == 'X' { player = 'O' } else { player = 'X' }
                moves += 1;
            }

        } else {
            readlines();
        }
}
   

}



fn readlines() -> usize {
    //no error handling (☞ﾟヮﾟ)☞
    let mut strinput = String::new(); 
    println!("Input");
    io::stdin().read_line(&mut strinput); 
    let input: usize = strinput.trim().parse().unwrap();
    return input;
}

fn checkinput(player: char, board: [char;9], input: usize) -> bool {
    if board[input] != ' ' {
        println!("already taken square");
        return false;
    }
    return true;
}


fn wincheck(player: char, board: [char;9]) -> bool {


    if board[4] == player { 
        if(board[0] == board[4] && board[0] == board[8]) 
        ||(board[2] == board[4] && board[2] == board[6]) {
            return true;
        }

    }

    for i in 0..3 {
        if board[i] == player {
            if board[i] == board[i+3] && board[i] == board[i+6] {
                return true; 
            }
        }

        let ii = i*3;
        if board[ii] == player {
            if board[ii] == board[ii+1] && board[ii] == board[ii+2] { 
                return true; 
            }
        }
    }

    return false;
}

fn win(player: char) -> [char;9] {
    if player != 'D' {
        println!("{} wins",player);
    } else {  println!("DRAW"); }

    let board: [char; 9] = [' '; 9];
    return board;
}



fn outputboard(board: [char;9]) {
    for i in 0..3 {
        println!("{} | {} | {}",
        board[(i+i*2)  ],
        board[(i+i*2)+1],
        board[(i+i*2)+2]);
        println!("---------");
    }
}

