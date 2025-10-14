// The Brainfuck interpreter.

use std::boxed::Box;

use crate::parser::{self, Ast, ParseError, Token};

/// Options for the interpreter.
pub struct InterpreterOptions<'a> {
    /// The size of the memory buffer in bytes.
    pub memory_buffer_size: u32,
    /// The output function to use for the `.` instruction.
    pub out_fn: &'a mut dyn FnMut(u8),
    /// The input function to use for the `,` instruction.
    pub in_fn: &'a mut dyn FnMut() -> u8,
}

/// The Brainfuck interpreter.
pub struct Interpreter<'a> {
    /// The abstract syntax tree to execute.
    ast: Ast,
    /// The memory buffer for the program.
    memory_buffer: Box<[u8]>,
    /// A pointer to the current cell in the memory buffer.
    memory_buffer_ptr: u32,
    /// The index of the current token being executed.
    current_token_idx: u32,
    /// The output function.
    out_fn: &'a mut dyn FnMut(u8),
    /// The input function.
    in_fn: &'a mut dyn FnMut() -> u8,
}

impl<'a> Interpreter<'a> {
    /// Create a new interpreter from a source string.
    pub fn new(source: &str, options: InterpreterOptions<'a>) -> Result<Self, ParseError> {
        let ast = parser::parse(source)?;
        Ok(Self::from_ast(ast, options))
    }

    /// Create a new interpreter from an AST.
    pub fn from_ast(ast: Ast, options: InterpreterOptions<'a>) -> Self {
        Self {
            ast,
            memory_buffer: vec![0; options.memory_buffer_size as usize].into_boxed_slice(),
            memory_buffer_ptr: 0,
            current_token_idx: 0,
            out_fn: options.out_fn,
            in_fn: options.in_fn,
        }
    }

    /// Execute a single step of the interpreter.
    ///
    /// Returns `false` if the program has finished executing.
    pub fn step(&mut self) -> bool {
        // Check if we've reached the end of the program.
        if self.current_token_idx as usize >= self.ast.tokens.len() {
            return false;
        }

        let current_token = self.ast.tokens[self.current_token_idx as usize];
        let heap_ptr = self.memory_buffer_ptr as usize;

        // Execute the current token.
        match current_token {
            Token::IncPtr => self.memory_buffer_ptr += 1,
            Token::DecPtr => self.memory_buffer_ptr -= 1,
            Token::IncByte => {
                self.memory_buffer[heap_ptr] = self.memory_buffer[heap_ptr].wrapping_add(1)
            }
            Token::DecByte => {
                self.memory_buffer[heap_ptr] = self.memory_buffer[heap_ptr].wrapping_sub(1)
            }
            Token::OutByte => (self.out_fn)(self.memory_buffer[heap_ptr]),
            Token::InByte => self.memory_buffer[heap_ptr] = (self.in_fn)(),
            Token::LoopStart => {
                // If the current cell is 0, jump to the matching `]`.
                if self.memory_buffer[heap_ptr] == 0 {
                    self.current_token_idx = self.ast.jump_table[self.current_token_idx as usize];
                }
            }
            Token::LoopEnd => {
                // If the current cell is not 0, jump to the matching `[`.
                if self.memory_buffer[heap_ptr] != 0 {
                    self.current_token_idx = self.ast.jump_table[self.current_token_idx as usize];
                }
            }
        }

        self.current_token_idx += 1;

        // Return true if there are more tokens to execute.
        (self.current_token_idx as usize) < self.ast.tokens.len()
    }

    /// Run the interpreter until the program has finished executing.
    pub fn run(&mut self) {
        while self.step() {}
    }
}
