// The Brainfuck VM.

use core::str::FromStr;

use alloc::{boxed::Box, vec};

use crate::ir::{IR, Op, ParseError};

/// Options for the VM.
pub struct VMOptions<'a> {
    /// The size of the memory buffer in bytes.
    pub memory_buffer_size: u32,
    /// The output function to use for the `.` instruction.
    pub out_fn: &'a mut dyn FnMut(u8),
    /// The input function to use for the `,` instruction.
    pub in_fn: &'a mut dyn FnMut() -> u8,
}

/// The Brainfuck VM.
pub struct VM<'a> {
    /// The IR to execute.
    ir: IR,
    /// The memory buffer for the program.
    memory_buffer: Box<[u8]>,
    /// A pointer to the current cell in the memory buffer.
    memory_buffer_ptr: u32,
    /// The index of the next Op to execute.
    current_token_idx: u32,
    /// The output function.
    out_fn: &'a mut dyn FnMut(u8),
    /// The input function.
    in_fn: &'a mut dyn FnMut() -> u8,
}

impl<'a> VM<'a> {
    /// Create a new VM from a source string.
    pub fn new(source: &str, options: VMOptions<'a>) -> Result<Self, ParseError> {
        let ir = IR::from_str(source)?;
        Ok(Self::from_ir(ir, options))
    }

    /// Create a new VM from an IR.
    pub fn from_ir(ir: IR, options: VMOptions<'a>) -> Self {
        Self {
            ir,
            memory_buffer: vec![0; options.memory_buffer_size as usize].into_boxed_slice(),
            memory_buffer_ptr: 0,
            current_token_idx: 0,
            out_fn: options.out_fn,
            in_fn: options.in_fn,
        }
    }

    /// Execute a single step of the VM.
    ///
    /// Returns `false` if the program has finished executing.
    pub fn step(&mut self) -> bool {
        // Check if we've reached the end of the program.
        if self.current_token_idx as usize >= self.ir.tokens.len() {
            return false;
        }

        let current_token = self.ir.tokens[self.current_token_idx as usize];
        let heap_ptr = self.memory_buffer_ptr as usize;

        // Execute the current token.
        match current_token {
            Op::IncPtr => self.memory_buffer_ptr += 1,
            Op::DecPtr => self.memory_buffer_ptr -= 1,
            Op::IncByte => {
                self.memory_buffer[heap_ptr] = self.memory_buffer[heap_ptr].wrapping_add(1)
            }
            Op::DecByte => {
                self.memory_buffer[heap_ptr] = self.memory_buffer[heap_ptr].wrapping_sub(1)
            }
            Op::OutByte => (self.out_fn)(self.memory_buffer[heap_ptr]),
            Op::InByte => self.memory_buffer[heap_ptr] = (self.in_fn)(),
            Op::LoopStart => {
                // If the current cell is 0, jump to the matching `]`.
                if self.memory_buffer[heap_ptr] == 0 {
                    self.current_token_idx = self.ir.jump_table[self.current_token_idx as usize];
                }
            }
            Op::LoopEnd => {
                // If the current cell is not 0, jump to the matching `[`.
                if self.memory_buffer[heap_ptr] != 0 {
                    self.current_token_idx = self.ir.jump_table[self.current_token_idx as usize];
                }
            }
        }

        self.current_token_idx += 1;

        // Return true if there are more tokens to execute.
        (self.current_token_idx as usize) < self.ir.tokens.len()
    }

    /// Run the VM until the program has finished executing.
    pub fn run(&mut self) {
        while self.step() {}
    }
}
