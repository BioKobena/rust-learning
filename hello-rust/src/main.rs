// appel de la bibliothèque standard pour faire du i/o (entrée/sortie) @BioKobena
use std::io;

fn main() {
    println!("--------------Jeu de dévinette--------------");

    println!("Veuillez saissir un nombre");

    let mut guess = String::new();

    // Cette fonction nous permet de gérer les entrées utilisateurs
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("Vous avez entré : {}", guess);

}
