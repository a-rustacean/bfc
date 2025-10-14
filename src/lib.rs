//! The BFC library.

pub mod interpreter;
pub mod parser;

#[cfg(test)]
mod tests {
    use crate::{
        interpreter::{Interpreter, InterpreterOptions},
        parser,
    };

    #[test]
    fn test_hello_world() {
        let program = "
>++++++++[<+++++++++>-]<.
>++++[<+++++++>-]<+.
+++++++..
+++.
>>++++++[<+++++++>-]<++.
------------.
>++++++[<+++++++++>-]<+.
<.
+++.
------.
--------.
>>>++++[<++++++++>-]<+.";

        let ast = parser::parse(program).unwrap();
        let mut buffer = Vec::new();
        let options = InterpreterOptions {
            memory_buffer_size: 30_000,
            out_fn: &mut |ch| {
                buffer.push(ch);
            },
            in_fn: &mut || unreachable!(),
        };
        let mut interpreter = Interpreter::from_ast(ast, options);
        interpreter.run();
        let output = String::from_utf8(buffer).unwrap();
        assert_eq!(output, "Hello, World!");
    }
}
