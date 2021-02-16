use std::env;
use std::fs::File;
use std::io::{self, Read};

fn main(){
    let args: Vec<String> = env::args().collect();
    let num = args.len();
    if num == 1{
        readstdin();
    }    
    else {    
        if args[1] == "-h" || args[1] == "--help" {
            println!("Kat - Koncatenate FILE(s) to standard output.\n");
            println!("Usage: kat [FILE]s... 'OR' kat from standard input");
        }
        else{
            for i in 1..num{
                let arg1 = args[i].clone();
                let _rs = readfile(arg1);
            }
        }
    }
}

pub fn readfile(filename: String) -> std::io::Result<()> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    println!("{}", contents);
    Ok(())
}
pub fn readstdin() -> io::Result<()>{
    let mut buffer = String::new();
    let mut stdin = io::stdin();
    stdin.read_to_string(&mut buffer)?;
    println!("{}", buffer);
    Ok(())
}
