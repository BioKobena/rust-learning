// appel de la bibliothèque standard pour faire du i/o (entrée/sortie) @BioKobena
// use rand::Rng;
// use std::cmp::Ordering;
// use std::io;

// Vous pourrez trouver un tutoriel dans la doc de Rust qui vous expliquera très facilement comment prendre en main Rust

//  Les annotations utilisé ici se basent sur la doc de Rust : https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
fn main() {
    // println!("Choisir le nombre!");

    // let nombre_aleatoire = rand::thread_rng().gen_range(1..=100);

    // // println!("Le nombre secret est : {}", nombre_aleatoire);

    // // Boucle tant que : permet de déterminer si le joueur a trouvé ou pas le nombre secret
    // loop {
    //     // Message pour choisir sa proposition
    //     println!("Entrez votre proposition");

    //     // Déclaration d'une variable pour recupérer la proposition du joueur
    //     let mut proposition = String::new();

    //     // Appel de la bibliothèque standard ::io pour l'entrée du joueur,
    //     // gestion des erreurs avec expect
    //     io::stdin()
    //         .read_line(&mut proposition)
    //         .expect("Echec de lecture de la ligne");

    //     // Transformation de nombre élément en entier afin de le traiter et le comparer avec le nombre aléatoire généré par la bibliothèque ::Rng
    //     let proposition: u32 = match proposition.trim().parse() {
    //         Ok(num) => num,
    //         Err(_) => continue,
    //     };

    //     // Affichage de la proposition du joueur
    //     println!("Vous avez proposé: {proposition}");

    //     // Comparaison (matching) entre le nombre secret et la proposition du joueur
    //     match proposition.cmp(&nombre_aleatoire) {
    //         Ordering::Less => println!("Trop petit!"),
    //         Ordering::Greater => println!("Trop grand !"),
    //         Ordering::Equal => {
    //             println!("Gagné !");
    //             break;
    //         }
    //     }
    // }
    // flow_control_if();
    flow_control_loop();
}

// Part 3.4 from https://doc.rust-lang.org/book/ch03-05-control-flow.html my daily learning @BioKobena
fn flow_control_if() {
    let number: i8 = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let nombre = if condition { 5 } else { 2 }; // À noter que si le type de retour 5 et 6 étaient différents, il y aurait eu des problèmes
                                                // Par exemple : {5} else{ "6"}

    println!("La valeur de nombre est : {nombre}");
}

fn flow_control_loop() {
    // let mut count = 0;

    // 'counting_up: loop {
    //     println!("La valeur de count est : {count}");
    //     count += 1;

    //     if count == 125000 {
    //         println!("La fin");
    //         break;
    //     }
    // }

    // println!("La nouvelle valeur de count : {count}");

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");


    // Compte à rebours simple avec la boucle for
    for nombre in (1..9).rev() {
        println!("{nombre}");
    }
    println!("BOOOMMM!!!!!!!!!!!!!!!!");
}
