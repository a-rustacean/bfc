// Brainfuck IR/Parser.

use core::{fmt, str::FromStr};

use alloc::{boxed::Box, vec::Vec};

/// Represents a single Brainfuck instruction/operation.
// size = 3 bits, physical size = 1 byte
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Op {
    IncPtr,
    DecPtr,
    IncByte,
    DecByte,
    OutByte,
    InByte,
    LoopStart,
    LoopEnd,
}

impl Op {
    /// Create a new token from a character.
    pub fn from_char(ch: char) -> Option<Self> {
        let token = match ch {
            '>' => Self::IncPtr,
            '<' => Self::DecPtr,
            '+' => Self::IncByte,
            '-' => Self::DecByte,
            '.' => Self::OutByte,
            ',' => Self::InByte,
            '[' => Self::LoopStart,
            ']' => Self::LoopEnd,
            _ => return None,
        };

        Some(token)
    }

    /// Convert a token back into a character.
    pub fn into_char(self) -> char {
        match self {
            Self::IncPtr => '>',
            Self::DecPtr => '<',
            Self::IncByte => '+',
            Self::DecByte => '-',
            Self::OutByte => '.',
            Self::InByte => ',',
            Self::LoopStart => '[',
            Self::LoopEnd => ']',
        }
    }
}

/// The intermediate representation for a Brainfuck program.
// just a list of [Op]s and a jump table.
//
// `jump_table[i]` will give the Op idx to conditionally
// jump to for the `ops[i]` Op, only two types of
// Ops need jumping, the `LoopStart` and `LoopEnd`.
//
// jump_table[loop_start_idx] = loop_end_idx
// jump_table[loop_end_idx] = loop_start_idx
pub struct IR {
    pub tokens: Box<[Op]>,
    pub jump_table: Box<[u32]>,
}

/// Kinds of parse errors that can occur.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParseErrorKind {
    /// A `[` was found without a matching `]`.
    UnclosedLoop,
    /// A `]` was found without a matching `[` before it.
    UnexpectedLoopEnd,
}

/// A parsing error with position and kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParseError {
    /// The index of the token where the error occurred.
    pub token_pos: u32,
    /// The type of error that occurred.
    pub kind: ParseErrorKind,
}

impl ParseError {
    /// Create a new parse error.
    #[inline]
    pub const fn new(token_pos: u32, kind: ParseErrorKind) -> Self {
        Self { token_pos, kind }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "parse error at {}: {:?}", self.token_pos, self.kind)
    }
}

impl FromStr for IR {
    type Err = ParseError;

    /// Parse a Brainfuck source string into an IR.
    fn from_str(input: &str) -> Result<Self, ParseError> {
        let mut tokens = Vec::new();
        let mut jump_table = Vec::new();
        let mut loop_starts = Vec::new();

        for (token_pos, char) in input.char_indices() {
            // Skip characters that are not Brainfuck instructions.
            let Some(token) = Op::from_char(char) else {
                continue;
            };

            match token {
                Op::LoopStart => {
                    // Record the position of the `[`.
                    loop_starts.push((tokens.len(), token_pos as u32));
                }
                Op::LoopEnd => {
                    // Pop the matching `[` from the stack.
                    let Some((loop_start, _)) = loop_starts.pop() else {
                        // If the stack is empty, there is no matching `[`.
                        return Err(ParseError::new(
                            token_pos as u32,
                            ParseErrorKind::UnexpectedLoopEnd,
                        ));
                    };

                    // Ensure the jump table is large enough.
                    let max = loop_start.max(tokens.len());
                    ensure_len(&mut jump_table, max);

                    // Set the jump table entries for the loop.
                    jump_table[loop_start] = tokens.len() as u32;
                    jump_table[tokens.len()] = loop_start as u32;
                }
                _ => {}
            }

            tokens.push(token);
        }

        // If there are any unclosed loops, return an error.
        if let Some((_, token_pos)) = loop_starts.pop() {
            return Err(ParseError::new(token_pos, ParseErrorKind::UnclosedLoop));
        }

        Ok(IR {
            tokens: tokens.into_boxed_slice(),
            jump_table: jump_table.into_boxed_slice(),
        })
    }
}

/// Ensure that a vector has a certain length.
fn ensure_len<T: Default + Clone>(v: &mut Vec<T>, index: usize) {
    if v.len() <= index {
        v.resize(index + 1, T::default());
    }
}

const _: () = {
    use core::mem::{align_of, size_of};
    assert!(size_of::<Op>() == 1);
    assert!(align_of::<Op>() == 1);

    assert!(size_of::<Option<Op>>() == 1);
    assert!(align_of::<Option<Op>>() == 1);
};
