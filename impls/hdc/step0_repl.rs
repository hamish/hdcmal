// use std::io;
use std::io::{self, Write};


fn main() {  
    let stdin = io::stdin();
    let mut buf = String::new();
    loop {
        print!("user> ");
        io::stdout().flush().unwrap();
        let bytes = stdin.read_line(&mut buf).expect("read will not fail");
        if bytes == 0 {
            break;
        }
        print!("{}" , buf);
        io::stdout().flush().unwrap();
        buf.clear();
    }

}
