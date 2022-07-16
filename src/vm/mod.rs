//! # Virtual Machine Module
//!
//! This module contains all things related to the virtual machine.
//!
//! ## What is this machine?
//!
//! This virtual machine is a simple turing tape machine.
//! There is one register, a tape of cells, and a tape pointer. Cells are
//! restricted to integers in the core variant of the machine code, but floats
//! are supported in the standard variant.
//!
//! ## What should I know first?
//!
//! You should ***NOT*** use pointers (values used with the Deref, Refer, Where,
//! Alloc, and Free operations) as if they are integers. Think about pointers
//! and integers as two completely separate types.
//!
//! #### Why?
//!
//! This is because virtual machine **implementations are bound to vary**.
//! For example: my C implementation uses *real pointers* (which are retrieved
//! through virtual machine instructions `Where` and `Alloc`, and allows the
//! implementation to be used with valgrind, gprof, a custom allocater, or
//! potentially garbage collection!), but an implementation in a language
//! like Python might use integer indices in a list instead.
//!
//! If the backend implementation uses pointers, *using `Inc` to move a pointer
//! to the next cell **will not work***. This is because pointers need to be
//! incremented by the size of the data type they point to. Because the virtual
//! machine's cell size is undefined (purposely, to make it as portable as possible),
//! ***you cannot know this size***. Therefore you cannot use `Inc` to move a pointer
//! to the next cell unless *you want your code to be unportable*.
//!
//! ***DO NOT USE `Inc` AND `Dec` TO MOVE POINTERS! NAVIGATE THE POINTER TO THE
//! DESIRED POSITION AND USE `Where` INSTEAD! OR YOUR CODE WILL NOT PORT TO ALL
//! IMPLEMENTATIONS!!***
//!
//! ## What data can it use?
//!
//! ***The virtual machine uses cells of any arbitrary bit width >= 16. The tape
//! must contain at least 4096 cells.*** The bit width is undefined, but **it must
//! remain constant for every cells.** Additionally, **the floating point (and pointer)
//! representation must be identical in size to the integer representation**.
//!
//! In this particular assembler, we assume that the bit width is 64. Supporting
//! smaller or larger bit widths is supported just by using integers of the
//! appropriate size, or using addition / multiplication to reach larger numbers.
//!
//! **An implementation of the virtual machine *should* be of any reasonable
//! bit width like: 16, 32, 64 bits (the standard), or unbounded.** For each implementation, the bits
//! of the integer and floats supported should be identical. The
//! default implementation should be with 64 bit ints and floats, but
//! 16 bit ints + no floats for a hardware implementation would suffice.
//! Infinitely large ints and floats are also supported, but the implementation
//! must be able to handle them.

mod core;
pub use self::core::*;

mod std;
pub use self::std::*;

mod interpreter;
pub use interpreter::*;

/// An interface to conveniently create virtual machine programs,
/// of either the core or standard variant.
pub trait VirtualMachineProgram {
    fn append_core_op(&mut self, op: CoreOp);
    fn append_standard_op(&mut self, op: StandardOp);

    fn comment(&mut self, comment: &str) {
        self.append_core_op(CoreOp::Comment(comment.to_string()));
    }

    fn restore(&mut self) {
        self.append_core_op(CoreOp::Restore);
    }

    fn save(&mut self) {
        self.append_core_op(CoreOp::Save);
    }

    fn ret(&mut self) {
        self.append_core_op(CoreOp::Return);
    }

    fn where_is_pointer(&mut self) {
        self.append_core_op(CoreOp::Where);
    }

    fn deref(&mut self) {
        self.append_core_op(CoreOp::Deref);
    }

    fn refer(&mut self) {
        self.append_core_op(CoreOp::Refer);
    }

    fn move_pointer(&mut self, cells: isize) {
        if cells != 0 {
            self.append_core_op(CoreOp::Move(cells));
        }
    }

    fn set_register(&mut self, val: isize) {
        self.append_core_op(CoreOp::Constant(val))
    }

    fn begin_while(&mut self) {
        self.append_core_op(CoreOp::While)
    }

    fn begin_if(&mut self) {
        self.append_core_op(CoreOp::If)
    }

    fn begin_else(&mut self) {
        self.append_core_op(CoreOp::Else)
    }

    fn begin_function(&mut self) {
        self.append_core_op(CoreOp::Function)
    }

    fn end(&mut self) {
        self.append_core_op(CoreOp::End)
    }

    fn call(&mut self) {
        self.append_core_op(CoreOp::Call)
    }

    fn inc(&mut self) {
        self.append_core_op(CoreOp::Inc)
    }

    fn dec(&mut self) {
        self.append_core_op(CoreOp::Dec)
    }

    fn get(&mut self) {
        self.append_core_op(CoreOp::Get)
    }

    fn put(&mut self) {
        self.append_core_op(CoreOp::Put)
    }

    fn is_non_negative(&mut self) {
        self.append_core_op(CoreOp::IsNonNegative)
    }
}
