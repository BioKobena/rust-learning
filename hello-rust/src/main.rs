// appel de la bibliothèque standard pour faire du i/o (entrée/sortie) @BioKobena
use rand::Rng;
use std::cmp::Ordering;
use std::io;

// Vous pourrez trouver un tutoriel dans la doc de Rust qui vous expliquera très facilement comment prendre en main Rust


//  Les annotations utilisé ici se basent sur la doc de Rust : https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
fn main() {

    
    println!("Choisir le nombre!");

    let nombre_aleatoire = rand::thread_rng().gen_range(1..=100);

    // println!("Le nombre secret est : {}", nombre_aleatoire);

    // Boucle tant que : permet de déterminer si le joueur a trouvé ou pas le nombre secret
    loop {
        // Message pour choisir sa proposition
        println!("Entrez votre proposition");

        // Déclaration d'une variable pour recupérer la proposition du joueur
        let mut proposition = String::new();

        // Appel de la bibliothèque standard ::io pour l'entrée du joueur,
        // gestion des erreurs avec expect
        io::stdin()
            .read_line(&mut proposition)
            .expect("Echec de lecture de la ligne");

        // Transformation de nombre élément en entier afin de le traiter et le comparer avec le nombre aléatoire généré par la bibliothèque ::Rng
        let proposition: u32 = match proposition.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // Affichage de la proposition du joueur
        println!("Vous avez proposé: {proposition}");


        // Comparaison (matching) entre le nombre secret et la proposition du joueur
        match proposition.cmp(&nombre_aleatoire) {

            Ordering::Less => println!("Trop petit!"),
            Ordering::Greater => println!("Trop grand !"),
            Ordering::Equal => {
                println!("Gagné !");
                break;
            }
        }
    }
}
