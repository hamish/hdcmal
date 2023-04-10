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
                println!("{}", line);
                rl.add_history_entry(line.as_str()).unwrap();
                io::stdout().flush().unwrap();
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    Ok(())
}
