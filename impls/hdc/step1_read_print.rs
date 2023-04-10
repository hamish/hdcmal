use std::io;
use std::io::Write;

use rustyline::error::ReadlineError;

use rustyline::{Config, Editor, Result};
fn main() -> Result<()> {
    let config = Config::builder().auto_add_history(true).build();
    let history = if false {
        // memory
        rustyline::sqlite_history::SQLiteHistory::with_config(config)?
    } else {
        // file
        rustyline::sqlite_history::SQLiteHistory::open(config, ".hdc_history.sqlite3")?
    };
    let mut rl: Editor<(), _> = Editor::with_history(config, history)?;


    loop {
        let readline = rl.readline("user> ");
        match readline {
            Ok(line) => {
                println!("{}" , line);
                rl.add_history_entry(line.as_str()).unwrap();
                io::stdout().flush().unwrap();
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }
    #[cfg(feature = "with-file-history")]
    rl.save_history("history.txt");
    Ok(())
}


/*


// use std::io;
use std::io::{self, Write};


use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};
const HISTORY_FILENAME: &str = "~/.hdc_history.txt";

fn main() -> Result<()> {
    // `()` can be used when no completer is required
    let mut rl = DefaultEditor::new()?;
    // #[cfg(feature = "with-file-history")]
    if rl.load_history(HISTORY_FILENAME).is_err() {
        println!("No previous history.");
    }
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                print!("{}" , line);
                io::stdout().flush().unwrap();            
            },
            Err(ReadlineError::Interrupted) => {
                //println!("CTRL-C");
                break
            },
            Err(ReadlineError::Eof) => {
                // println!("CTRL-D");
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }
    // #[cfg(feature = "with-file-history")]
    rl.save_history(HISTORY_FILENAME);
    Ok(())
}


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
*/