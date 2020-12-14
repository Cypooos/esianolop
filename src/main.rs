use esianolop;
use std::env;
use std::io::{self, Write};
use std::process;

pub fn help() {
    println!(r#"Help : Esianolop v1.0b par Cyprien Bourotte
Commandes :
 - file|f <file>   : Execute le fichier <file>
 - exe|x|e [code]* : Execute le code [code]*
 - help|?          : Affiche l'aide
 - print|p         : Affiche les arbres du compilers
 - reset|r         : Reset le compilateur
 - quit|q          : Quitte l'invite de commande
    
Instructions :
 - add|+ : ajoute les 2 valeurs
 - sub|- : soustrait les 2 valeurs
 - mul|* : multiplie les 2 valeurs
 - div|/ : divise les 2 valeurs
 - pow|^ : met en puissance les 2 valeurs
 - dpL|< : duplique la valeur et la place devant
 - dpR|> : duplique la valeur et la place derrière
 - dup|~ : duplique la valeur et la place à coté
 - <nb>  : Insère un noeu nombre

Pour chaque instruction vous pouvez spécifiez si elle se fera sur le devant du stack (avec un "<" devant, par défault) ou sur le fond -par défault juste pour les nombres- (avec un ">" devant)

Par exemple '2 3 <1' donnera '1 2 3' car le 1 à été inséré au devant du stack, tandis que '1 2 3 >+' donnera '1 Add(2,3)'
    
Exemples :
 - '1 2 3 >~'       => '1 2 3 3' (duplique la dernière valeur)
 - '1 2 3 pow add'  => 'Add(Pow(1,2),3)'
 - '1 2 3 >pow add' => 'Add(1,Pow(2,3))'
 - '1 2 3 >'        => '1 2 3 3'
 - '1 2 3 ><'       => '3 1 2 3' (duplique la dernière valeur au devant)
 - '1 2 3 <>'       => '1 2 3 1' (duplique la première valeur derrière)
"#
)
}

pub fn command_line() {
    println!("Esianolop v1.0b, par Cyprien Bourotte.\nType 'help' or '?' to get help.");
    
    let mut compiler = esianolop::structs::Esianolop{
        values:Vec::new()
    };

    loop {
        print!(">> ");
        io::stdout().flush().unwrap();
        let mut input_str = String::new();
        io::stdin().read_line(&mut input_str).expect("error: unable to read user input");
        let mut input: Vec<&str>= input_str.trim().split(" ").filter(|x| x.to_owned().trim() != "" ).collect::<Vec<&str>>();
        if input.len() == 0 {
            println!("Empty command."); continue;
        }
        match input[0] {
            "?" | "help" => {
                help()
            }
            "q" | "quit" => {process::exit(1);}
            "r" | "reset" => {
                compiler.values = Vec::new();
            }
            "p" | "print" => {println!("{:?}",compiler.values);}
            "e" | "exe" | "x" => {
                match match input.get(1..(input.len())) {
                    Some(e) => {
                        println!("{:?}",&e.join(" "));
                        compiler.parse_text(&e.join(" "))
                    },
                    None => Err("Syntax: e *[code]".to_owned())
                }{
                    Ok(()) => {},
                    Err(e) => {println!("{} \n for \"{:?}\"",e,input.get(1..(input.len())))}
                }

            }
            _ => {
                println!("Unknow command. Type 'help' or '?' to get help.")
            }
        }
    }

}

fn main() {


    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        println!("Starting command-line use, because no arguments has been given");
        command_line();
    }
    println!("{:?}",args);

    let mut compiler = esianolop::structs::Esianolop{
        values:Vec::new()
    };

    let tests = [
        "1 2 3 >~","1 2 3 pow add","1 2 3 >pow add","1 2 3 >","1 2 3 ><","1 2 3 <>"
    ];

    for x in tests.iter() {
        compiler.values = Vec::new();
        println!("{:?}",compiler.parse_text(x));
        println!("{:?}",compiler.values);

    }

}
