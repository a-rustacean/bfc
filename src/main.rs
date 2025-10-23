// The main entry point for the BFC interpreter.
use std::{
    env, fs,
    io::{self, Read, Write},
    str::FromStr,
};

use bfc::{IR, VM, VMOptions};

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

    // Define the output function for the VM.
    fn putchar(ch: u8) {
        print!("{}", ch as char);
    }

    // Define the input function for the VM.
    fn getchar() -> u8 {
        io::stdout().flush().unwrap();
        let mut buffer = [0; 1]; // Read one byte from stdin.
        io::stdin().read_exact(&mut buffer).unwrap();
        buffer[0]
    }

    // Parse the source code into an IR.
    let ir = IR::from_str(&source).unwrap();

    // Set up the VM options.
    let options = VMOptions {
        memory_buffer_size: 30_000, // Standard Brainfuck memory size.
        out_fn: &mut putchar,
        in_fn: &mut getchar,
    };

    // Create a new VM from the IR and options.
    let mut vm = VM::from_ir(ir, options);

    // Run the VM.
    vm.run();

    Ok(())
}
