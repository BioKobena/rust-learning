// appel de la bibliothèque standard pour faire du i/o (entrée/sortie) @BioKobena
// use rand::Rng;
// use std::cmp::Ordering;
// use std::io;

// Vous pourrez trouver un tutoriel dans la doc de Rust qui vous expliquera très facilement comment prendre en main Rust


fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.

//  Les annotations utilisé ici se basent sur la doc de Rust : https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
fn main() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    //  println!("{}", s);                               // Par exemple ici s n'est plus valide

    let x = 5;                      // x comes into scope

    // makes_copy(x);                  // because i32 implements the Copy trait,
                                    // x does NOT move into the function,
    // println!("{}", x);              // so it's okay to use x afterward

    // Mais on peut utiliser x car i32 est un type qui implémente le trait Copy
    // println!("La valeur de x : {x}");
    // ownership();

    sort_array();
}

// By The Book of Rust Learn @BioKobena
// fn choice_gaming() {
//     println!("Choisir le nombre!");

//     let nombre_aleatoire = rand::thread_rng().gen_range(1..=100);

//     // println!("Le nombre secret est : {}", nombre_aleatoire);

//     // Boucle tant que : permet de déterminer si le joueur a trouvé ou pas le nombre secret
//     loop {
//         // Message pour choisir sa proposition
//         println!("Entrez votre proposition");

//         // Déclaration d'une variable pour recupérer la proposition du joueur
//         let mut proposition = String::new();

//         // Appel de la bibliothèque standard ::io pour l'entrée du joueur,
//         // gestion des erreurs avec expect
//         io::stdin()
//             .read_line(&mut proposition)
//             .expect("Echec de lecture de la ligne");

//         // Transformation de nombre élément en entier afin de le traiter et le comparer avec le nombre aléatoire généré par la bibliothèque ::Rng
//         let proposition: u32 = match proposition.trim().parse() {
//             Ok(num) => num,
//             Err(_) => continue,
//         };

//         // Affichage de la proposition du joueur
//         println!("Vous avez proposé: {proposition}");

//         // Comparaison (matching) entre le nombre secret et la proposition du joueur
//         match proposition.cmp(&nombre_aleatoire) {
//             Ordering::Less => println!("Trop petit!"),
//             Ordering::Greater => println!("Trop grand !"),
//             Ordering::Equal => {
//                 println!("Gagné !");
//                 break;
//             }
//         }
//     }
// }

// Part 3.4 from https://doc.rust-lang.org/book/ch03-05-control-flow.html my daily learning @BioKobena
// fn flow_control_if() {
//     let number: i8 = 6;

//     if number % 4 == 0 {
//         println!("number is divisible by 4");
//     } else if number % 3 == 0 {
//         println!("number is divisible by 3");
//     } else if number % 2 == 0 {
//         println!("number is divisible by 2");
//     } else {
//         println!("number is not divisible by 4, 3, or 2");
//     }

//     let condition = true;
//     let nombre = if condition { 5 } else { 2 }; // À noter que si le type de retour 5 et 6 étaient différents, il y aurait eu des problèmes
//                                                 // Par exemple : {5} else{ "6"}

//     println!("La valeur de nombre est : {nombre}");
// }

// fn flow_control_loop() {
//     let mut count = 0;

//     'counting_up: loop {
//         println!("La valeur de count est : {count}");
//         count += 1;

//         if count == 125000 {
//             println!("La fin");
//             break;
//         }
//     }

//     println!("La nouvelle valeur de count : {count}");

//     let mut count = 0;
//     'counting_up: loop {
//         println!("count = {count}");
//         let mut remaining = 10;

//         loop {
//             println!("remaining = {remaining}");
//             if remaining == 9 {
//                 break;
//             }
//             if count == 2 {
//                 break 'counting_up;
//             }
//             remaining -= 1;
//         }

//         count += 1;
//     }
//     println!("End count = {count}");

//     // Compte à rebours simple avec la boucle for
//     for nombre in (1..9).rev() {
//         println!("{nombre}");
//     }
//     println!("BOOOMMM!!!!!!!!!!!!!!!!");
// }

// Part 4.1 : Ownership
// (la plus grande différence entre Rust et les autres langages de programmation qui proposent soit des garbage collector ou des références counting) @BioKobena
// fn ownership() {
//     {
//         // s is not valid here, it’s not yet declared
//         let s = "hello"; // s is valid from this point forward
//         println!("La valeur de s : {s}");
//         // do stuff with s
//     } // this scope is now over, and s is no longer valid


//     let mut s = String::from("Hello");
//     s.push_str(", world!");
//     println!("La valeur de s : {s}");



//     let x = 5;
//     let y = x;
//     println!("La valeur de x : {x}, la valeur de y : {y}"); 

//     // let s1 = String::from("hello");
//     // let s2 = s1;
//     // println!("La valeur de s1 : {s1}, la valeur de s2 : {s2}"); // Cela va générer une erreur car s1 n'est plus valide après l'assignation à s2


//     // On va utiliser la méthode clone pour faire une copie de la valeur de s1 dans s2
//      let s1 = String::from("hello");
//     let s2 = s1.clone();
//     println!("La valeur de s1 : {s1}, la valeur de s2 : {s2}"); // Cela va générer une erreur car s1 n'est plus valide après l'assignation à s2

//     let mut s = String::from("hello");
//     s = String::from("ahoy");

//     println!("{s}, world!");
// }


// fn first_word(s:&String){
//     let bytes = s.as_string();

// }


fn sort_array(){
    let mut arr = [50, 20, 1, 4, 3, 12, 11, 5];

    let mut i = 0;

    while i<arr.len() {
        let mut j = 0;
        while j <arr.len(){
            if arr[i] < arr[j]{
                let tampon = arr[i];
                arr[i] = arr[j];
                arr[j] = tampon;
            }
            j += 1;
        }
        i += 1;
    }

    println!("{:?} tableau trié", arr);
}