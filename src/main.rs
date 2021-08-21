use std::char;
use std::fs::OpenOptions;
use std::io::prelude::*;

fn main() {
    //1 ♔ Rei
    //1 ♛ Dama
    //2 ♝ Bispos
    //2 ♞ Cavalos
    //2 ♜ Torres
    //8 ♟ Peões.
    let rei = 1 as char;
    let dama = 2 as char;
    let bispo = 3 as char;
    let bispo2 = 4 as char;
    let cavalo = 5 as char;
    let cavalo2 = 6 as char;
    let torre = 7 as char;
    let torre2 = 8 as char;
    let peao = 9 as char;
    let peao2 = 10 as char;
    let peao3 = 11 as char;
    let peao4 = 12 as char;
    let peao5 = 13 as char;
    let peao6 = 14 as char;
    let peao7 = 15 as char;
    let peao8 = 16 as char;

    let reiB = 17 as char;
    let damaB = 18 as char;
    let bispoB = 19 as char;
    let bispo2B = 20 as char;
    let cavaloB = 21 as char;
    let cavalo2B = 22 as char;
    let torreB = 23 as char;
    let torre2B = 24 as char;
    let peaoB = 25 as char;
    let peao2B = 26 as char;
    let peao3B = 27 as char;
    let peao4B = 28 as char;
    let peao5B = 29 as char;
    let peao6B = 30 as char;
    let peao7B = 31 as char;
    let peao8B = 32 as char;

    let turno = 1 as char;
    let bk = 33 as char;

    let map = [
        bk, bk, bk, bk, bk, bk, bk, bk, bk, bk, bk, bk, bk, bk, bk, bk, bk, bk, bk, bk, bk, bk, bk,
        bk, bk, bk, bk, bk, bk, bk, bk, bk, bk, bk, bk, bk, bk, bk, bk, bk, bk, bk, bk, bk, bk, bk,
        bk, bk, bk, bk, bk, bk, bk, bk, bk, bk, bk, bk, bk, bk, bk, bk, bk, bk,
    ];

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("database.hex")
        .unwrap();

    let mut result = String::from("");

    for i in 0..64 {
      result.push(map[i]);
    }
    println!("{}", result);
    write!(file, "{}", result);
}
