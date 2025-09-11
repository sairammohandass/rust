use std::fmt;

/// Supported operators for binary expressions.
#[derive(Debug, Clone, Copy)]
pub enum Operator {
    Add,
    Subtract,
    // Future: Multiply, Divide, etc.
}

impl Operator {
    /// Apply the operator to two operands.
    fn apply(&self, left: f64, right: f64) -> f64 {
        match self {
            Operator::Add => left + right,
            Operator::Subtract => left - right,
            
        }
    }
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Operator::Add => write!(f, "+"),
            Operator::Subtract => write!(f, "-"),
        }
    }
}

/// Recursive representation of an expression.
#[derive(Debug, Clone)]
enum Expression {
    Constant(f64),
    Binary {
        op: Operator,
        left: Box<Expression>,
        right: Box<Expression>,
    },
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Constant(val) => write!(f, "{:.6}", val),
            Expression::Binary { op, left, right } => {
                write!(f, "({} {} {})", left, op, right)
            }
        }
    }
}

/// Public-facing wrapper around an expression tree.
#[derive(Debug, Clone)]
pub struct ExpressionContext {
    expr: Expression,
    
}

impl ExpressionContext {
    /// Creates a new empty context. (Not tied to a specific expression.)
    pub fn new() -> Self {
        // Empty context — could be extended later (variables, env, etc.)
        ExpressionContext {
            expr: Expression::Constant(0.0),
        }
    }

    /// Creates a constant expression.
    pub fn new_constant_expression(value: f64) -> Self {
        ExpressionContext {
            expr: Expression::Constant(value),
        }
    }

    /// Creates a binary expression from two sub-expressions.
    pub fn new_binary_expression(op: Operator, left: Self, right: Self) -> Self {
        ExpressionContext {
            expr: Expression::Binary {
                op,
                left: Box::new(left.expr),
                right: Box::new(right.expr),
            },
        }
    }

    /// Evaluate an expression asynchronously.
    pub async fn eval(&self, expr: &ExpressionContext) -> Result<f64, ExpressionError> {
        expr.eval_inner()
    }

    /// Internal recursive evaluator.
    fn eval_inner(&self) -> Result<f64, ExpressionError> {
        match &self.expr {
            Expression::Constant(val) => Ok(*val),
            Expression::Binary { op, left, right } => {
                let l = ExpressionContext { expr: *left.clone() }.eval_inner()?;
                let r = ExpressionContext { expr: *right.clone() }.eval_inner()?;
                Ok(op.apply(l, r))
            }
        }
    }
}

impl fmt::Display for ExpressionContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.expr)
    }
}

/// Error type for expression evaluation.
#[derive(Debug)]
pub enum ExpressionError {
    InvalidExpression,
}
