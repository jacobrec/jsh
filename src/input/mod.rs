extern crate rustyline;

pub fn read (editor: &mut rustyline::Editor<()>) -> Result<String, u8> {
    let readline = editor.readline("jsh> ");
    match readline {
        Ok(line) => {
            editor.add_history_entry(line.as_str());
            Ok(line)
        },
        Err(rustyline::error::ReadlineError::Interrupted) => {
            Err(2)
        },
        Err(rustyline::error::ReadlineError::Eof) => {
            Err(1)
        },
        Err(_) => {
            Err(0)
        }
    }
}
