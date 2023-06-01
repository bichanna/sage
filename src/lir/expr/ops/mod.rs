//! # Operations
//!
//! This module contains implementations of various
//! tertiary, binary, and unary operations on LIR expressions.
mod arithmetic;
mod assign;
mod comparison;
mod io;
mod logic;
mod tagged_union;
mod memory;

pub use arithmetic::*;
pub use assign::*;
pub use comparison::*;
pub use io::*;
pub use logic::*;
pub use tagged_union::*;
pub use memory::*;

use crate::{asm::AssemblyProgram, lir::*};

/// A trait used to implemented an assignment operation.
///
/// This trait is used to implement assignment operations like `+=` and `-=`.
pub trait AssignOp: std::fmt::Debug + std::fmt::Display {
    /// Typechecks the operation on the given expressions.
    fn type_check(&self, dst: &Expr, src: &Expr, env: &Env) -> Result<(), Error> {
        if self.can_apply(&dst.get_type(env)?, &src.get_type(env)?, env)? {
            dst.type_check(env).and(src.type_check(env))
        } else {
            Err(Error::InvalidAssignOp(
                self.clone_box(),
                dst.clone(),
                src.clone(),
            ))
        }
    }
    /// Gets the type of the operation on the given expressions.
    fn return_type(&self, dst: &Expr, src: &Expr, env: &Env) -> Result<Type, Error> {
        if self.can_apply_exprs(dst, src, env)? {
            dst.get_type(env)
        } else {
            Err(Error::InvalidAssignOp(
                self.clone_box(),
                dst.clone(),
                src.clone(),
            ))
        }
    }
    /// Clones the operation into a boxed trait object.
    fn clone_box(&self) -> Box<dyn AssignOp>;
    /// Formats the operation for display.
    fn display(&self, dst: &Expr, src: &Expr) -> String {
        format!("{} {} {}", dst, self, src)
    }

    /// Checks if the operation can be applied to the given types.
    fn can_apply(&self, dst: &Type, src: &Type, env: &Env) -> Result<bool, Error>;
    /// Checks if the operation can be applied to the given expressions.
    fn can_apply_exprs(&self, dst: &Expr, src: &Expr, env: &Env) -> Result<bool, Error> {
        self.can_apply(&dst.get_type(env)?, &src.get_type(env)?, env)
    }
    /// Evaluates the operation on the given constant expressions.
    fn eval(&self, dst: &ConstExpr, src: &ConstExpr, env: &mut Env) -> Result<ConstExpr, Error>;
    /// Compiles the operation on the given expressions.
    fn compile(
        &self,
        dst: &Expr,
        src: &Expr,
        env: &mut Env,
        output: &mut dyn AssemblyProgram,
    ) -> Result<(), Error> {
        dst.clone().compile_expr(env, output)?;
        src.clone().compile_expr(env, output)?;
        self.compile_types(&dst.get_type(env)?, &src.get_type(env)?, env, output)
    }
    /// Compiles the operation on the given types. (Generates the code for the operation.)
    fn compile_types(
        &self,
        dst: &Type,
        src: &Type,
        env: &mut Env,
        output: &mut dyn AssemblyProgram,
    ) -> Result<(), Error>;
}

/// A trait used to implement a unary operation.
///
/// This trait is used to implement unary operations like `not` and `~`.
pub trait UnaryOp: std::fmt::Debug + std::fmt::Display {
    /// Typechecks the operation on the given expression.
    fn type_check(&self, expr: &Expr, env: &Env) -> Result<(), Error> {
        if self.can_apply(&expr.get_type(env)?, env)? {
            expr.type_check(env)
        } else {
            Err(Error::InvalidUnaryOp(self.clone_box(), expr.clone()))
        }
    }
    /// Gets the type of the operation on the given expression.
    fn return_type(&self, expr: &Expr, env: &Env) -> Result<Type, Error> {
        if self.can_apply_exprs(expr, env)? {
            Ok(expr.get_type(env)?)
        } else {
            Err(Error::InvalidUnaryOp(self.clone_box(), expr.clone()))
        }
    }
    /// Clones the operation into a boxed trait object.
    fn clone_box(&self) -> Box<dyn UnaryOp>;
    /// Formats the operation for display.
    fn display(&self, expr: &Expr) -> String {
        format!("{} {}", self, expr)
    }
    /// Checks if the operation can be applied to the given type.
    fn can_apply(&self, expr: &Type, env: &Env) -> Result<bool, Error>;
    /// Checks if the operation can be applied to the given expression.
    fn can_apply_exprs(&self, expr: &Expr, env: &Env) -> Result<bool, Error> {
        self.can_apply(&expr.get_type(env)?, env)
    }
    /// Evaluates the operation on the given constant expression.
    fn eval(&self, expr: &ConstExpr, env: &mut Env) -> Result<ConstExpr, Error>;
    /// Compiles the operation on the given expression.
    fn compile(
        &self,
        expr: &Expr,
        env: &mut Env,
        output: &mut dyn AssemblyProgram,
    ) -> Result<(), Error> {
        expr.clone().compile_expr(env, output)?;
        self.compile_types(&expr.get_type(env)?, env, output)
    }
    /// Compiles the operation on the given type. (Generates the code for the operation.)
    fn compile_types(
        &self,
        expr: &Type,
        env: &mut Env,
        output: &mut dyn AssemblyProgram,
    ) -> Result<(), Error>;
}

/// A trait used to implement a binary operation.
///
/// This trait is used to implement binary operations like `+` and `==`.
pub trait BinaryOp: std::fmt::Debug + std::fmt::Display {
    /// Typechecks the operation on the given expressions.
    fn type_check(&self, lhs: &Expr, rhs: &Expr, env: &Env) -> Result<(), Error> {
        if self.can_apply(&lhs.get_type(env)?, &rhs.get_type(env)?, env)? {
            lhs.type_check(env).and(rhs.type_check(env))
        } else {
            Err(Error::InvalidBinaryOp(
                self.clone_box(),
                lhs.clone(),
                rhs.clone(),
            ))
        }
    }
    /// Gets the type of the operation on the given expressions.
    fn return_type(&self, lhs: &Expr, rhs: &Expr, env: &Env) -> Result<Type, Error> {
        if self.can_apply_exprs(lhs, rhs, env)? {
            lhs.get_type(env)
        } else {
            Err(Error::InvalidBinaryOp(
                self.clone_box(),
                lhs.clone(),
                rhs.clone(),
            ))
        }
    }
    /// Clones the operation into a boxed trait object.
    fn clone_box(&self) -> Box<dyn BinaryOp>;
    /// Formats the operation for display.
    fn display(&self, lhs: &Expr, rhs: &Expr) -> String {
        format!("{} {} {}", lhs, self, rhs)
    }

    /// Checks if the operation can be applied to the given types.
    fn can_apply(&self, lhs: &Type, rhs: &Type, env: &Env) -> Result<bool, Error>;
    /// Checks if the operation can be applied to the given expressions.
    fn can_apply_exprs(&self, lhs: &Expr, rhs: &Expr, env: &Env) -> Result<bool, Error> {
        self.can_apply(&lhs.get_type(env)?, &rhs.get_type(env)?, env)
    }
    /// Evaluates the operation on the given constant expressions.
    fn eval(&self, lhs: &ConstExpr, rhs: &ConstExpr, env: &mut Env) -> Result<ConstExpr, Error>;
    /// Compiles the operation on the given expressions.
    fn compile(
        &self,
        lhs: &Expr,
        rhs: &Expr,
        env: &mut Env,
        output: &mut dyn AssemblyProgram,
    ) -> Result<(), Error> {
        lhs.clone().compile_expr(env, output)?;
        rhs.clone().compile_expr(env, output)?;
        self.compile_types(&lhs.get_type(env)?, &rhs.get_type(env)?, env, output)
    }
    /// Compiles the operation on the given types. (Generates the code for the operation.)
    fn compile_types(
        &self,
        lhs: &Type,
        rhs: &Type,
        env: &mut Env,
        output: &mut dyn AssemblyProgram,
    ) -> Result<(), Error>;
}

/// A trait used to implement a ternary operation.
pub trait TernaryOp: std::fmt::Debug + std::fmt::Display {
    /// Typechecks the operation on the given expressions.
    fn type_check(&self, a: &Expr, b: &Expr, c: &Expr, env: &Env) -> Result<(), Error> {
        if self.can_apply(
            &a.clone().get_type(env)?,
            &b.clone().get_type(env)?,
            &c.get_type(env)?,
            env,
        )? {
            a.type_check(env)
                .and(b.type_check(env))
                .and(c.type_check(env))
        } else {
            Err(Error::InvalidTernaryOp(
                self.clone_box(),
                a.clone(),
                b.clone(),
                c.clone(),
            ))
        }
    }
    /// Gets the type of the operation on the given expressions.
    fn return_type(&self, a: &Expr, b: &Expr, c: &Expr, env: &Env) -> Result<Type, Error> {
        if self.can_apply_exprs(a, b, c, env)? {
            c.get_type(env)
        } else {
            Err(Error::InvalidTernaryOp(
                self.clone_box(),
                a.clone(),
                b.clone(),
                c.clone(),
            ))
        }
    }
    /// Clones the operation into a boxed trait object.
    fn clone_box(&self) -> Box<dyn TernaryOp>;
    /// Formats the operation for display.
    fn display(&self, a: &Expr, b: &Expr, c: &Expr) -> String {
        format!("{} {} {} {}", a, self, b, c)
    }

    /// Checks if the operation can be applied to the given types.
    fn can_apply(&self, a: &Type, b: &Type, c: &Type, env: &Env) -> Result<bool, Error>;
    /// Checks if the operation can be applied to the given expressions.
    fn can_apply_exprs(&self, a: &Expr, b: &Expr, c: &Expr, env: &Env) -> Result<bool, Error> {
        self.can_apply(&a.get_type(env)?, &b.get_type(env)?, &c.get_type(env)?, env)
    }
    /// Evaluates the operation on the given constant expressions.
    fn eval(
        &self,
        a: &ConstExpr,
        b: &ConstExpr,
        c: &ConstExpr,
        env: &mut Env,
    ) -> Result<ConstExpr, Error>;
    /// Compiles the operation on the given expressions.
    fn compile(
        &self,
        a: &Expr,
        b: &Expr,
        c: &Expr,
        env: &mut Env,
        output: &mut dyn AssemblyProgram,
    ) -> Result<(), Error> {
        // Evaluate the three expression on the stack.
        a.clone().compile_expr(env, output)?;
        b.clone().compile_expr(env, output)?;
        c.clone().compile_expr(env, output)?;
        // Compile the operation.
        self.compile_types(
            &a.get_type(env)?,
            &b.get_type(env)?,
            &c.get_type(env)?,
            env,
            output,
        )
    }
    /// Compiles the operation on the given types. (Generates the code for the operation.)
    fn compile_types(
        &self,
        a: &Type,
        b: &Type,
        c: &Type,
        env: &mut Env,
        output: &mut dyn AssemblyProgram,
    ) -> Result<(), Error>;
}

impl PartialEq for dyn AssignOp {
    fn eq(&self, other: &Self) -> bool {
        self.display(
            &Expr::ConstExpr(ConstExpr::Int(0)),
            &Expr::ConstExpr(ConstExpr::Int(0)),
        )
        .eq(&other.display(
            &Expr::ConstExpr(ConstExpr::Int(0)),
            &Expr::ConstExpr(ConstExpr::Int(0)),
        ))
    }
}

impl PartialEq for dyn UnaryOp {
    fn eq(&self, other: &Self) -> bool {
        self.display(&Expr::ConstExpr(ConstExpr::Int(0)))
            .eq(&other.display(&Expr::ConstExpr(ConstExpr::Int(0))))
    }
}

impl PartialEq for dyn BinaryOp {
    fn eq(&self, other: &Self) -> bool {
        self.display(
            &Expr::ConstExpr(ConstExpr::Int(0)),
            &Expr::ConstExpr(ConstExpr::Int(0)),
        )
        .eq(&other.display(
            &Expr::ConstExpr(ConstExpr::Int(0)),
            &Expr::ConstExpr(ConstExpr::Int(0)),
        ))
    }
}

impl PartialEq for dyn TernaryOp {
    fn eq(&self, other: &Self) -> bool {
        self.display(
            &Expr::ConstExpr(ConstExpr::Int(0)),
            &Expr::ConstExpr(ConstExpr::Int(0)),
            &Expr::ConstExpr(ConstExpr::Int(0)),
        )
        .eq(&other.display(
            &Expr::ConstExpr(ConstExpr::Int(0)),
            &Expr::ConstExpr(ConstExpr::Int(0)),
            &Expr::ConstExpr(ConstExpr::Int(0)),
        ))
    }
}

impl Clone for Box<dyn AssignOp> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

impl Clone for Box<dyn UnaryOp> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

impl Clone for Box<dyn BinaryOp> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

impl Clone for Box<dyn TernaryOp> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
