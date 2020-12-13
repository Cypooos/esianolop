use esianolop;

fn main() {
    let mut compiler = esianolop::structs::Esianolop{
        values:Vec::new()
    };

    println!("{:?}",compiler.parse_text("1 2 3 <+"));
    println!("{:?}",compiler.values);
    compiler.values = Vec::new();
    
    println!("{:?}",compiler.parse_text("1 2 3 >+"));
    println!("{:?}",compiler.values);
    compiler.values = Vec::new();
    
    println!("{:?}",compiler.parse_text("1 2 3 <~ $ +"));
    println!("{:?}",compiler.values);
    compiler.values = Vec::new();
    
    println!("{:?}",compiler.parse_text("1 2 3 ~"));
    println!("{:?}",compiler.values);
    compiler.values = Vec::new();
    
    println!("{:?}",compiler.parse_text("1 2 3"));
    println!("{:?}",compiler.values);
}
