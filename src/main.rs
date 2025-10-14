// The main entry point for the BFC interpreter.
use std::{
    env, fs,
    io::{self, Read, Write},
};

use bfc::{
    interpreter::{Interpreter, InterpreterOptions},
    parser,
};

fn main() -> io::Result<()> {
    // Parse command-line arguments to get the file path.
    let mut args = env::args();
    args.next(); // Skip the program name.
    let file_path = match args.next() {
        Some(path) => path,
        None => {
            eprintln!("Error: No input file provided.");
            // Exit gracefully if no file is provided.
            return Ok(());
        }
    };

    // Read the Brainfuck source code from the file.
    let source = fs::read_to_string(file_path)?;

    // Define the output function for the interpreter.
    fn putchar(ch: u8) {
        print!("{}", ch as char);
    }

    // Define the input function for the interpreter.
    fn getchar() -> u8 {
        io::stdout().flush().unwrap();
        let mut buffer = [0; 1]; // Read one byte from stdin.
        io::stdin().read_exact(&mut buffer).unwrap();
        buffer[0]
    }

    // Parse the source code into an AST.
    let ast = parser::parse(&source).unwrap();

    // Set up the interpreter options.
    let options = InterpreterOptions {
        memory_buffer_size: 30_000, // Standard Brainfuck memory size.
        out_fn: &mut putchar,
        in_fn: &mut getchar,
    };

    // Create a new interpreter from the AST and options.
    let mut interpreter = Interpreter::from_ast(ast, options);

    // Run the interpreter.
    interpreter.run();

    Ok(())
}
