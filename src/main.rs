use esianolop;
use std::env;
use std::io::{self, Write};
use std::process;


// La fonction qui affiche l'aide
pub fn help() {
    println!(r#"Help : Esianolop v1.0b par Cyprien Bourotte
Commandes :
 - file|f <file>   : Execute le fichier <file>
 - exe|x|e [code]* : Execute le code [code]*
 - help|?          : Affiche l'aide
 - print|p         : Affiche les arbres du compilers
 - reset|r         : Reset les arbres du compilateur
 - null|n          : Reset le compilateur
 - quit|q          : Quitte l'invite de commande
    
Instructions :
 - add|+ : Ajoute les 2 valeurs
 - sub|- : Soustrait les 2 valeurs
 - mul|* : Multiplie les 2 valeurs
 - div|/ : Divise les 2 valeurs
 - pow|^ : Met en puissance les 2 valeurs
 - dpL|< : Duplique la valeur et la place devant
 - dpR|> : Duplique la valeur et la place derrière
 - dup|~ : Duplique la valeur et la place à coté
 - del|! : Supprimer la valeur
 - <nb>  : Insère un noeu nombre

Pour chaque instruction vous pouvez spécifiez si elle se fera sur le devant du stack (avec un "<" devant, par défault) ou sur le fond -par défault juste pour les nombres- (avec un ">" devant)

Par exemple '2 3 <1' donnera '1 2 3' car le 1 à été inséré au devant du stack, tandis que '1 2 3 >+' donnera '1 Add(2,3)'
    
Exemples :
 - '1 2 3 >~'       => '1 2 3 3' (duplique la dernière valeur)
 - '1 2 3 ~'        => '1 1 2 3' (duplique la première valeur)
 - '1 2 3 pow add'  => 'Add(Pow(1,2),3)'
 - '1 2 3 >pow add' => 'Add(1,Pow(2,3))'
 - '1 2 3 >'        => '1 2 3 3'
 - '1 2 3 ><'       => '3 1 2 3' (duplique la dernière valeur au devant)
 - '1 2 3 <>'       => '1 2 3 1' (duplique la première valeur derrière)
 - 'e 2 3 <1 >!'    => '1 2' (on ajoute 1 au début et supprime le 3)
"#
)
}

// Fonction qui execute du code en ligne de commande. Ne retourne rien, affiche directement
fn execute_command(input:Vec<&str>,mut compiler:&mut esianolop::structs::Esianolop) {

    // On regarde le permier argument
    match input[0] {

        "?" | "help" => help(), // Affichage de l'aide
        "q" | "quit" => process::exit(0), // Quitter l'application
        "r" | "reset" => compiler.values = Vec::new(), // On reset le stack
        "p" | "print" => println!("{:?} => {:?}",compiler.values,compiler.get_result()), // On affiche le stack / le stack compilé
        "n" | "null" => {
            // Reset tout le compilateur (fonctions aussi)
            *compiler = esianolop::structs::Esianolop::new();
            println!("Interpréteur reset.");
        }, 
        "e" | "exe" | "x" => { 
            
            // Il y a un double match ici, gloire à rust ^^
            // Le premier retourne l'execution des arguments si ils y sont, sinon retourne une erreur
            match match input.get(1..(input.len())) {

                Some(e) => {
                    compiler.parse_text(&e.join(" ")) // le match renvoie l'execution du code entré ici
                },
                None => Err("Syntax: e *[code]".to_owned()) // Si aucun code (None), retourne une erreur
            } {
                // Deuxième match, affiche le résultat / l'erreur, que ce soit du premier match ou de l'execution du code
                Ok(()) => {println!("{:?}",compiler.get_result());},
                Err(e) => {println!("{}",e)}
            }

        }
        // Pour tout autre paterne de commande, on ne connnais pas, affichage de l'aide
        _ => {
            println!("Unknow command. Type 'help' or '?' to get help.")
        }
    }
}

// L'invite de commande
pub fn command_line() {

    // Affichage du message d'introduction 
    println!("Esianolop v1.0b, par Cyprien Bourotte.\nType 'help' or '?' to get help.");
    
    // Création d'une instance d'un compilateur.
    let mut compiler = esianolop::structs::Esianolop::new();

    loop { // Boucle infinie

        // Affichage du curseur
        print!(">> ");io::stdout().flush().unwrap();
        
        // L'équivalent d'un input (prend le stdin et le met dans input_str)
        let mut input_str = String::new();
        io::stdin().read_line(&mut input_str).expect("error: unable to read user input");

        // Découpe l'input en série d'arguments séparé par un espace (+ trimage d'espaces / tabulation / \r en trop)
        let mut input: Vec<&str>= input_str.trim().split(" ").filter(|x| x.to_owned().trim() != "" ).collect::<Vec<&str>>();
        
        // Si la commande est vide, on recommence la loop
        if input.len() == 0 {
            println!("Empty command."); continue;
        }

        // On execute notre fonction
        execute_command(input,&mut compiler);
    }

}

// Le point d'entré de notre programme.
fn main() {

    // Récupération des arguments (args[0] correspond au chemin de l'executable)
    let args: Vec<String> = env::args().collect();

    // Si il n'y a pas d'arguments (autre que le chemin de l'executable)
    if args.len() <= 1 {
        println!("Starting command-line use, because no arguments has been given");
        command_line();
    }
    
    // On créé un compilateur
    let mut compiler = esianolop::structs::Esianolop::new();

    // Découpe l'input en série d'arguments séparé par un espace (+ trimage d'espaces / tabulation / \r en trop)
    let mut input: Vec<&str>= args.get(1..(args.len())).unwrap().iter().filter(|x| x.to_owned().trim() != "" ).map(|x| x as &str).collect::<Vec<&str>>();
    

    execute_command(input, &mut compiler);

}


// Big tests :
// e test:7:uwu:+:owo:2: test owo uwu
