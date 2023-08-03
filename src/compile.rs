//! Transforms an AST into (not really byte) code.

use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Display, Formatter};

use crate::ast;
use crate::ast::{ComparisonOp, IntegerExpr};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CompileError {
    location: usize,
    kind: CompileErrorKind,
}

impl Error for CompileError {}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum CompileErrorKind {
    ConstRedeclaration(String),
    FuncRedeclaration(String),
    UndefinedFunction(String),
    InlineFunctionCyclicDependency(String),
    MainNotFound,
    ReturnInNoReturnContext,
    UndefinedConstant(String),
    Unimplemented(&'static str),
}

impl Display for CompileError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use CompileErrorKind::*;
        write!(f, "CompileError at {}: {}", self.location, match &self.kind {
            ConstRedeclaration(name) => format!("Declaration of already existing const {}", name),
            FuncRedeclaration(name) => format!("Declaration of already existing function {}", name),
            UndefinedFunction(name) => format!("Reference to undefined function {}", name),
            InlineFunctionCyclicDependency(name) => format!("Detected cyclic dependency in inline functions ({})", name),
            MainNotFound => format!("Could not find 'main' function"),
            ReturnInNoReturnContext => format!("Cannot use 'return' here"),
            UndefinedConstant(name) => format!("Undefined constant {}", name),
            Unimplemented(what) => format!("\"{}\" is not implemented yet", what),
        })
    }
}

fn error<T>(kind: CompileErrorKind, location: usize) -> Result<T, CompileError> {
    Err(CompileError { location, kind })
}

#[derive(Debug)]
pub struct Constant {
    value: u32,
}

#[derive(Debug)]
pub enum AstFunc {
    Function(AstFunction),
    InlineFunction(AstInlineFunction),
}

#[derive(Debug)]
pub struct AstFunction {
    name: String,
    arg: ast::ArgumentType,
    returns: ast::ReturnType,
    body: Vec<ast::Statement>,
}

#[derive(Debug)]
pub struct AstInlineFunction {
    name: String,
    body: Vec<ast::Statement>,
}

#[derive(Debug, Default)]
pub struct Evaluator {
    functions: HashMap<String, Option<AstFunc>>,
    consts: HashMap<String, Constant>,
}

impl Evaluator {
    pub fn evaluate(&mut self, ast: ast::Program) -> Result<(), CompileError> {
        for def in ast.definitions {
            use ast::DefinitionKind::*;
            match def.node {
                Function {
                    name,
                    arg,
                    returns,
                    body,
                } => {
                    if self.functions.contains_key(&name) {
                        return error(CompileErrorKind::FuncRedeclaration(name), def.location);
                    }
                    self.functions.insert(
                        name.clone(),
                        Some(AstFunc::Function(AstFunction {
                            name,
                            body,
                            arg,
                            returns,
                        })),
                    );
                }
                InlineFunction { name, body } => {
                    if self.functions.contains_key(&name) {
                        return error(CompileErrorKind::FuncRedeclaration(name), def.location);
                    }
                    self.functions.insert(
                        name.clone(),
                        Some(AstFunc::InlineFunction(AstInlineFunction { name, body })),
                    );
                }
                Const { name, value } => {
                    if self.consts.contains_key(&name) {
                        return error(CompileErrorKind::ConstRedeclaration(name), def.location);
                    }
                    self.consts.insert(name.clone(), Constant { value });
                }
            }
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct CompiledFunction {
    inline: bool,
    code: Vec<String>,
    compiled: bool,
    compiling: bool,
    label: u32,
    calling_context: CallingContext,
}

#[derive(Debug, Default)]
pub struct Compiler {
    evaluator: Evaluator,
    functions: HashMap<String, CompiledFunction>,
    next_label: u32,
}

impl Compiler {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn compile(&mut self, ast: ast::Program) -> Result<String, CompileError> {
        self.evaluator.evaluate(ast)?;

        for func in self.evaluator.functions.values() {
            use AstFunc::*;
            match func.as_ref().expect("At this stage all functions should be Some")
            {
                InlineFunction(f) => {
                    self.functions.insert(
                        f.name.clone(),
                        CompiledFunction {
                            compiled: false,
                            compiling: false,
                            inline: true,
                            label: self.next_label,
                            // Calling context should never be used for inline function
                            calling_context: CallingContext::PassNothing,
                            code: vec![],
                        },
                    );
                    self.next_label += 1;
                }
                Function(f) => {
                    self.functions.insert(
                        f.name.clone(),
                        CompiledFunction {
                            compiled: false,
                            compiling: false,
                            inline: false,
                            label: self.next_label,
                            calling_context: self.reveal_calling_context(&f.name),
                            code: vec![],
                        },
                    );
                    self.next_label += 1;
                }
            };
        }

        for name in self.functions.keys().map(|x| x.clone()).collect::<Vec<_>>() {
            self.compile_function(&name)?;
        }

        let main_label = match self.functions.get("main") {
            Some(x) => x.label,
            None => {
                return error(CompileErrorKind::MainNotFound, 0);
            }
        };

        // Set our starting point at 'main'
        let mut code = vec![format!(">{}|", Self::encode_label(main_label))];

        // Add our functions
        for func in self.functions.values_mut() {
            if !func.inline {
                code.push(format!("|{}:", Self::encode_label(func.label)));
                code.append(&mut func.code);
                // TODO make sure our functions return
            }
        }

        Ok(Self::encode_sequence(code))
    }

    fn encode_sequence(code: Vec<String>) -> String {
        let mut result: String = "$".into();

        for page in code.chunks(12 * 15) {
            for (i, line) in page.chunks(15).enumerate() {
                for cmd in line {
                    result.push_str(cmd);
                }
                result.push_str(",");
                if i != 11 {
                    result.push_str("\n");
                }
            }
            result.push_str("~\n");
        }

        result
    }

    fn reveal_calling_context(&self, func_name: &String) -> CallingContext {
        match self.evaluator.functions.get(func_name).expect("must exist").as_ref().expect("must exist") {
            AstFunc::Function(f) => match f.arg {
                ast::ArgumentType::Nothing => CallingContext::PassNothing,
                ast::ArgumentType::Argument => CallingContext::PassArgument,
                ast::ArgumentType::State => CallingContext::PassState,
            },
            _ => panic!("reveal_calling_context should be called only for non-inline functions")
        }
    }

    fn compile_statements(
        &mut self,
        ast: Vec<ast::Statement>,
        ctx: &ReturnContext,
    ) -> Result<Vec<String>, CompileError> {
        let mut result = vec![];

        for stmt in ast {
            use ast::StatementKind;
            match stmt.node {
                StatementKind::StatementBlock { body } => {
                    result.append(&mut self.compile_statements(body, ctx)?);
                }
                StatementKind::FunctionCall { name } => {
                    result.append(&mut self.compile_function_call(name, stmt.location)?);
                }
                StatementKind::Loop { body } => {
                    let label = Self::encode_label(self.next_label);
                    self.next_label += 1;
                    result.push(format!("|{}:", label));
                    result.append(&mut self.compile_statements(body, ctx)?);
                    result.push(format!(">{}|", label));
                }
                StatementKind::While { condition, body, negative } => {
                    let start_label = Self::encode_label(self.next_label);
                    self.next_label += 1;
                    let finish_label = Self::encode_label(self.next_label);
                    self.next_label += 1;

                    // start label
                    result.push(format!("|{}:", start_label));
                    // eval expr
                    result.append(&mut self.compile_expr(*condition)?);
                    if negative {
                        // negative: jump to finish if ok
                        result.push(format!("!?{}<", finish_label));
                    } else {
                        // positive: jump to finish if failed
                        result.push(format!("?{}<", finish_label));
                    }
                    // body
                    result.append(&mut self.compile_statements(body, ctx)?);
                    // finish label
                    result.push(format!("|{}:", finish_label));
                }
                StatementKind::If { condition, body, orelse, negative } => {
                    let failed_label = Self::encode_label(self.next_label);
                    self.next_label += 1;
                    let finish_label = Self::encode_label(self.next_label);
                    self.next_label += 1;
                    // eval expr first
                    result.append(&mut self.compile_expr(*condition)?);
                    if negative {
                        // negative: jump to failed if ok
                        result.push(format!("!?{}<", failed_label));
                    } else {
                        // positive: jump to failed if failed
                        result.push(format!("?{}<", failed_label));
                    }
                    // ok branch
                    result.append(&mut self.compile_statements(body, ctx)?);
                    // jump to finish
                    result.push(format!(">{}|", finish_label));
                    // failed branch
                    result.push(format!("|{}:", failed_label));
                    result.append(&mut self.compile_statements(orelse, ctx)?);
                    // finish label
                    result.push(format!("|{}:", finish_label));
                }
                StatementKind::Return { expr } => {
                    if let Some(expr) = expr {
                        result.append(&mut self.compile_expr(*expr)?);
                    }
                    result.push(match ctx {
                        ReturnContext::NoReturn => {
                            return error(CompileErrorKind::ReturnInNoReturnContext, stmt.location);
                        }
                        ReturnContext::ReturnNothing => "<|",
                        ReturnContext::ReturnArgument => "<-|",
                        ReturnContext::ReturnState => "<=|",
                    }.into());
                }
                StatementKind::InlineCode { body } => {
                    result.push(body.code);
                }
            }
        }

        Ok(result)
    }

    fn compile_function_call(&mut self, name: String, loc: usize) -> Result<Vec<String>, CompileError> {
        self.compile_function_if_inline(&name)?;

        let func = match self.functions.get(&name) {
            Some(f) => f,
            None => {
                return error(CompileErrorKind::UndefinedFunction(name), loc);
            }
        };

        let mut result = vec![];

        if func.inline {
            result.append(&mut func.code.clone());
        } else {
            let label = Self::encode_label(func.label);
            result.push(match func.calling_context {
                CallingContext::PassNothing => format!(":>{}>", label),
                CallingContext::PassArgument => format!("->{}>", label),
                CallingContext::PassState => format!("=>{}>", label),
            });
        }

        Ok(result)
    }

    fn reveal_const(&self, name: &String, loc: usize) -> Result<u32, CompileError> {
        match self.evaluator.consts.get(name) {
            None => error(CompileErrorKind::UndefinedConstant(name.clone()), loc),
            Some(val) => Ok(val.value)
        }
    }

    fn compile_expr(&mut self, expr: ast::Expr) -> Result<Vec<String>, CompileError> {
        let mut result = vec![];

        use ast::ExprKind::*;
        result.append(&mut match expr.node {
            FunctionCallResult { name } => {
                // TODO check that function returns arg or state
                self.compile_function_call(name, expr.location)?
            }
            VariableComparison { var, value, op } => {
                let value = match value {
                    IntegerExpr::Literal { value } => value,
                    IntegerExpr::Const { name } => self.reveal_const(&name, expr.location)?,
                };
                let op = match op {
                    ComparisonOp::Eq => "=",
                    ComparisonOp::NotEq => {
                        return error(CompileErrorKind::Unimplemented("!= operator"), expr.location);
                    }
                    ComparisonOp::LessThan => "<",
                    ComparisonOp::GreaterThan => ">",
                    ComparisonOp::LessOrEqual => {
                        return error(CompileErrorKind::Unimplemented("<= operator"), expr.location);
                    }
                    ComparisonOp::GreaterOrEqual => {
                        return error(CompileErrorKind::Unimplemented(">= operator"), expr.location);
                    }
                };
                vec![format!("({}{}{})", var.name, op, value)]
            }
            InlineCode { body } => {
                vec![body.code]
            }
        });

        Ok(result)
    }

    fn encode_label(label: u32) -> String {
        label.to_string()
    }

    fn compile_function_if_inline(&mut self, name: &String) -> Result<(), CompileError> {
        let inline = {
            let func = self
                .functions
                .get(name)
                .expect("compile_function() called on nonexistent function");
            func.inline
        };

        if inline {
            self.compile_function(name)
        } else {
            Ok(())
        }
    }

    fn compile_function(&mut self, name: &String) -> Result<(), CompileError> {
        let (compiled, inline) = {
            let func = self
                .functions
                .get_mut(name)
                .expect("compile_function() called on nonexistent function");
            if !func.compiled {
                if func.compiling {
                    return error(
                        CompileErrorKind::InlineFunctionCyclicDependency(name.clone()),
                        0,
                    );
                    // FIXME: can't get location here
                }
                func.compiling = true;
            }
            (func.compiled, func.inline)
        };

        let code = if !compiled {
            if inline {
                let ast = self.get_inline_function_ast(name);
                Some(self.compile_statements(ast, &ReturnContext::NoReturn)?)
            } else {
                let (ast, returns) = self.get_function_ast(name);
                Some(self.compile_statements(
                    ast,
                    &match returns {
                        ast::ReturnType::Nothing => ReturnContext::ReturnNothing,
                        ast::ReturnType::Argument => ReturnContext::ReturnArgument,
                        ast::ReturnType::State => ReturnContext::ReturnState,
                    },
                )?)
            }
        } else {
            None
        };

        if let Some(code) = code {
            let func = self.functions.get_mut(name).unwrap(); // already checked above
            func.code = code;
            func.compiling = false;
            func.compiled = true;
        }

        Ok(())
    }

    fn get_inline_function_ast(&mut self, name: &String) -> Vec<ast::Statement> {
        match self
            .evaluator
            .functions
            .get_mut(name)
            .expect("Functions in Compiler must exist in Evaluator")
            .take()
            .expect("get_inline_function_ast should never be called twice")
        {
            AstFunc::InlineFunction(AstInlineFunction { body, .. }) => body,
            _ => {
                panic!("get_inline_function_ast() called on non-inline function");
            }
        }
    }

    fn get_function_ast(
        &mut self,
        name: &String,
    ) -> (Vec<ast::Statement>, ast::ReturnType) {
        match self
            .evaluator
            .functions
            .get_mut(name)
            .expect("Functions in Compiler must exist in Evaluator")
            .take()
            .expect("get_function_ast should never be called twice")
        {
            AstFunc::Function(AstFunction {
                                  body, returns, ..
                              }) => (body, returns),
            _ => {
                panic!("get_function_ast() called on inline function");
            }
        }
    }
}

#[derive(Debug)]
pub enum ReturnContext {
    NoReturn,
    ReturnNothing,
    ReturnArgument,
    ReturnState,
}

#[derive(Debug)]
pub enum CallingContext {
    PassNothing,
    PassArgument,
    PassState,
}