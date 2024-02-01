use std::{io::{BufRead, BufReader, BufWriter, Read, Write}, net::{TcpListener, TcpStream}, ptr::null, thread, vec};

use core::num;
use std::io;
use rand::Rng;
use std::cmp::Ordering;

/*
struct GamePlayer{
    player1: Option<TcpStream>,
    player2: Option<TcpStream>,
    playerTurn: bool
}

impl GamePlayer {

    fn wait_for_another_player(&self){

        match self.player2 {
            
        }

    }

    fn GameStart(&mut self){

        self.write_to_player(&self.player1, String::from("\n"));
        self.write_to_player(&self.player1, String::from("Welcome to this epic game, player 1!"));
        self.write_to_player(&self.player1, String::from("In this game you will guess a number between 1 and 100\n"));
        self.write_to_player(&self.player1, String::from("Each time you guess, you will be notified if it is too low or too high\n"));
        self.write_to_player(&self.player1, String::from("Good luck gamer!\n\n\n"));

        self.write_to_player(&self.player2, String::from("Welcome to this epic game, player 2!"));
        self.write_to_player(&self.player2, String::from("In this game you will guess a number between 1 and 100\n"));
        self.write_to_player(&self.player2, String::from("Each time you guess, you will be notified if it is too low or too high\n"));
        self.write_to_player(&self.player2, String::from("Good luck gamer!\n\n\n"));
    }

    fn write_to_player(&self, mut player: &TcpStream, message: String) {
        player.write(String::from(message + "\n").as_bytes());
    }
}
*/
fn handle_connection(mut stream: TcpStream) {

    let _random_number: u16 = rand::thread_rng().gen_range(1..=100);
    println!("Random number {}", _random_number);


    stream.write(b"\n\n");
    stream.write(b"Welcome to this epic game\n");
    stream.write(b"In this game you will guess a number between 1 and 100\n");
    stream.write(b"Each time you guess, you will be notified if it is too low or too high\n");
    stream.write(b"Good luck gamer!\n\n\n");


    loop{

        stream.write(b"Input your guess: ");

        let mut buf_reader = BufReader::new(stream.try_clone().expect("a"));
        
        let mut input: String = String::new(); 
        buf_reader.read_line(&mut input);
    
        println!("Request: {:#?}", input);
    

        if input == ""{
            stream.write(b"FUCK YOU!!");
            continue;
        }


        let guess_Number: u16 = match input.trim().parse::<u16>(){
            Ok(num) => num,
            Err(_) => continue,
        };
    
        match guess_Number.cmp(&_random_number) {
            Ordering::Less => stream.write(b"WEAK!\n\n"),
            Ordering::Equal => {stream.write(b"Just nice :3\n\n"); break;},
            Ordering::Greater => stream.write(b"TOO STRONG!!!!\n\n"),
        };
        
    }


    
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    //let games: Vec<GamePlayer> = Vec::new();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        thread::spawn(|| {
            println!("New client connected");

            //let mut gameplayer: GamePlayer = GamePlayer {player1: stream.try_clone().expect("a"), player2: stream.try_clone().expect("a"), playerTurn: false};

            //gameplayer.GameStart();
            
            handle_connection(stream);
        });
    }
}
