//! The BFC library.

#![no_std]

extern crate alloc;

pub mod ir;
pub mod vm;

pub use ir::IR;
pub use vm::{VM, VMOptions};

#[cfg(test)]
mod tests {
    use core::str::FromStr;

    use alloc::{string::String, vec::Vec};

    use crate::{IR, VM, VMOptions};

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

        let ir = IR::from_str(program).unwrap();
        let mut buffer = Vec::new();
        let options = VMOptions {
            memory_buffer_size: 30_000,
            out_fn: &mut |ch| {
                buffer.push(ch);
            },
            in_fn: &mut || unreachable!(),
        };
        let mut vm = VM::from_ir(ir, options);
        vm.run();
        let output = String::from_utf8(buffer).unwrap();
        assert_eq!(output, "Hello, World!");
    }
}
