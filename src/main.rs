
// 
// Bienvenue sur ce magnifique projet de création d'un language en polonaise inversé
// Le nom viens du mot "polonaise" à l'envers : "esianolop"
// 
// Ce projet à été codé en rust: cela permet une plus grande rapidité et sécurité.
// Il s'utillise en ligne de commande
// 
// Si vous avez besoin d'aide avec le code: cyprien.bourotte@gmail.com
// Le dossier "exemples" contiens des fichers executable par l'interpreteur.
// Vous pouvez le faire via la commande `f exemples/1`


// J'importe mes classes
use esianolop;
// Importation de env pour les arguments quand utillisé en ligne de commande
use std::env;
// Importation de io pour les inputs
use std::io::{self, Write};
// Importation de process pour quitter le programme
use std::process;



// La fonction qui affiche l'aide
pub fn help() {
    println!(r#"Help : Esianolop v1.2b par Cyprien Bourotte
Commandes :
 - file|f <file>   : Execute le fichier <file>
 - exe|x|e [code]* : Execute le code [code]*
 - help|?          : Affiche l'aide
 - print|p         : Affiche les arbres du interpreteur
 - reset|r         : Reset les arbres du interpreteur (pas les fonctions)
 - null|n          : Reset l'interpreteur
 - quit|q          : Quitte l'invite de commande
    
Instructions :
 - add|+ : Ajoute les 2 valeurs
 - sub|- : Soustrait les 2 valeurs
 - mul|* : Multiplie les 2 valeurs
 - div|/ : Divise les 2 valeurs
 - pow|^ : Met en puissance les 2 valeurs
 - sqr|$ : Met en racine la valeure
 - dpL|< : Duplique la valeur et la place devant
 - dpR|> : Duplique la valeur et la place derrière
 - dup|~ : Duplique la valeur et la place à coté
 - del|! : Supprimer la valeur
 - <nb>  : Insère un noeu nombre

Pour chaque instruction vous pouvez spécifiez si elle se fera sur le devant du stack (avec un "<" devant, par défault) ou sur le fond (avec un ">" derrière, par défault juste pour les nombres)

Par exemple '2 3 <1' donnera '1 2 3' car le 1 à été inséré au devant du stack, tandis que '1 2 3 >+' donnera '1 Add(2,3)'

Exemples de code:
 - e 1 2 3 >~       => 1 2 3 3         (duplique la dernière valeur)
 - e 1 2 3 ~        => 1 1 2 3         (duplique la première valeur)
 - e 1 2 3 pow add  => Add(Pow(1,2),3)
 - e 1 2 3 >pow add => Add(1,Pow(2,3))
 - e 1 2 3 >        => 1 2 3 3        
 - e 1 2 3 ><       => 3 1 2 3         (duplique la dernière valeur au devant)
 - e 1 2 3 <>       => 1 2 3 1         (duplique la première valeur derrière)
 - e 2 3 <1 >!      => 1 2             (on ajoute 1 au début et supprime le 3)


Fonctions :
 - Vous pouvez définir des fonctions en utillisant les ':'. Chaque fonction porte un nom, et est assigné à une série de commandes.
   Par exemple, taper `e test:1 2 +:` définie une fonction "test" qui executera "1 2 +". Pour l'appeller, vous pouvez taper son nom ("e test test" donnera [4 2])
 - Certaine fonction sont pré-définie, par exemple la fonction "for" qui execute son code X fois, X étant la valeur dans le stack.
 - Les fonction ne se reset pas avec la commande `reset`, mais seulement avec la commande `null`.
 - Une fonction ne peut etre que défini en une ligne (pas de multiligne possible pour cette version 1.0)
Exemples :
 - e one:1: plus:+: 3 one plus => Add(1,3)
 - e 90 5 for:2 +:             => 100 (répéter 5 fois "2 +" depuis 90)
 - e 90 5 <for:2 +:            => 185 (répéter 90 fois "2 +" depuis 5)
 - e t:7:u:+:o:2: t o u        => 9   (7 2 +)
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
        "f" | "file" => { // On lit un fichier
            
            // Il y a un double match ici, gloire à rust ^^
            // Le premier retourne l'execution des arguments si ils y sont, sinon retourne une erreur
            match match input.get(1..(input.len())) {
                Some(e) => {
                    compiler.parse_file(&e.join(" ")) // le match renvoie l'execution du fichier entré ici
                },
                None => Err("Syntax: f <file_path>".to_owned()) // Si aucun code (None), retourne une erreur
            }  {
                // Deuxième match, affiche le résultat / l'erreur, que ce soit du premier match ou de l'execution du code
                Ok(()) => {
                    let res= compiler.get_result();
                    println!("results = [{}]",res.iter().map(|x| if x.is_err(){"Err".to_string()}else{x.unwrap().to_string()}).collect::<Vec<String>>().join(", "));
                    println!("{}",res.iter().filter(|x|x.is_err()).map(|x| format!("Error: {}",x.unwrap_err())).collect::<Vec<String>>().join("\n"));
                },
                Err(e) => {println!("{}",e)}
            }
        },
        "p" | "print" => println!("{:?} => {:?}",compiler.values,compiler.get_result()), // On affiche le stack / le stack compilé
        "n" | "null" => {
            // Reset tout l'interpreteur (fonctions aussi)
            compiler.clear();
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
                Ok(()) => {
                    let res= compiler.get_result();
                    println!("results = [{}]",res.iter().map(|x| if x.is_err(){"Err".to_string()}else{x.unwrap().to_string()}).collect::<Vec<String>>().join(", "));
                    println!("{}",res.iter().filter(|x|x.is_err()).map(|x| format!("Error: {}",x.unwrap_err())).collect::<Vec<String>>().join("\n"));
                },
                Err(e) => {println!("{}",e)}
            }

        }
        // Pour tout autre paterne de commande, on ne connnais pas, affichage de l'aide
        _ => {
            println!("Unknow command. Type 'help' or '?' to get help.")
        }
    }
}

// L'invite de commande (ne retourne pas, boucle infinie ou arette le programme)
pub fn command_line() -> ! {

    // Affichage du message d'introduction 
    println!("Esianolop v1.0b, par Cyprien Bourotte.\nType 'help' or '?' to get help.");
    
    // Création d'une instance du interpreteur.
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
    
    // On créé un interpreteur
    let mut compiler = esianolop::structs::Esianolop::new();

    // Découpe l'input en série d'arguments séparé par un espace (+ trimage d'espaces / tabulation / \r en trop)
    let input: Vec<&str>= args.get(1..(args.len())).unwrap().iter().filter(|x| x.to_owned().trim() != "" ).map(|x| x as &str).collect::<Vec<&str>>();
    

    execute_command(input, &mut compiler);

}

