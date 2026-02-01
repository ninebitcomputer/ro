use crate::ast::*;
use crate::env::*;

//AST w/symbols resolved

pub enum ASTError {
    BadFunction(FnDeclError),
    BadExpr(ExprError),
    Env(EnvError),
}

pub enum FnDeclError {
    DuplicateDecl,
    DuplicateParam,
}

pub struct BAst {
    pub environment: BAstEnv,
    pub statements: Vec<Statement>,
}

pub enum BStmt {
    If(BIf),
    Assign(BAssign),
    While(BWhile),
    Call(BCall),
    Return(Box<BExpr>),
}

pub struct BIf {
    pub guard: Box<BExpr>,
    pub t: BAst,
    pub f: Option<BAst>,
}

pub struct BAssign {
    pub ident: RelVarID,
    pub value: Box<BExpr>,
}

pub struct BWhile {
    pub cond: Box<BExpr>,
    pub body: BAst,
}

pub struct BCall {
    pub ident: RelFnID,
    pub args: Vec<BExpr>,
}

pub enum BExpr {
    Unary(BUnary),
    IntLit(i64),
    FloatLit(f32),
    Binop(BBinop),
    Var(RelVarID),
}

pub struct BUnary {
    pub op: UOp,
    pub expr: Box<BExpr>,
}

pub struct BBinop {
    pub a: Box<BExpr>,
    pub op: Op,
    pub b: Box<BExpr>,
}

impl BExpr {
    pub fn get_type(&self) -> Result<LType, ASTError> {
        todo!()
    }

    pub fn default_value(typ: LType) -> BExpr {
        match typ {
            LType::Int => BExpr::IntLit(0),
            LType::Float => BExpr::FloatLit(0.0),
        }
    }
}

fn convert_statements(ast: &Vec<Statement>, env_chain: &EnvChain) -> Result<BAst, ASTError> {
    let mut current_env = BAstEnv::new();
    let mut stmts: Vec<BStmt> = Vec::new();

    // load function definitions, then process all statements
    for stmt in ast.iter() {
        if let Statement::Function(f) = stmt {
            current_env
                .new_function(f.ident.clone(), f.ret, &f.params)
                .map_err(|e| ASTError::BadFunction(e))?;
        }
    }

    for stmt in ast.iter() {
        match stmt {
            Statement::If(iff) => {
                let if_env = env_chain.with(&current_env);
                let guard = convert_expr_wrapped(&iff.guard, &if_env)?;
                let t = convert_statements(&iff.t, &if_env)?;
                let f = if let Some(ff) = &iff.f {
                    Some(convert_statements(ff, &if_env)?)
                } else {
                    None
                };

                stmts.push(BStmt::If(BIf {
                    guard: Box::new(guard),
                    t,
                    f,
                }))
            }
            Statement::Declare(d) => {
                let assign = if let Some(e) = &d.assign {
                    convert_expr_wrapped(e, &env_chain.with(&current_env))?
                } else {
                    BExpr::default_value(d.typ)
                };

                let id = current_env.new_variable(d.ident.clone(), d.typ);
                stmts.push(BStmt::Assign(BAssign {
                    ident: RelVarID::new(0, id),
                    value: Box::new(assign),
                }));
            }
            Statement::While(w) => {
                let snapshot = env_chain.with(&current_env);
                let cond = convert_expr_wrapped(&w.cond, &snapshot)?;
                let body = convert_statements(&w.body, &snapshot)?;

                stmts.push(BStmt::While(BWhile {
                    cond: Box::new(cond),
                    body,
                }))
            }
            Statement::Call(c) => {
                let f = env_chain
                    .with(&current_env)
                    .lookup_function(&c.ident)
                    .map_err(|e| ASTError::Env(e))?;
            }
            _ => todo!(),
        };
    }

    todo!()
}

pub enum ExprError {
    Env(EnvError),
}

pub fn convert_expr_wrapped(expr: &Expr, env_chain: &EnvChain) -> Result<BExpr, ASTError> {
    convert_expr(expr, env_chain).map_err(|e| ASTError::BadExpr(e))
}

// TODO: Type checking
pub fn convert_expr(expr: &Expr, env_chain: &EnvChain) -> Result<BExpr, ExprError> {
    let r = match expr {
        Expr::Unary(u) => {
            let subexpr = convert_expr(&u.x, env_chain)?;
            BExpr::Unary(BUnary {
                op: u.op,
                expr: Box::new(subexpr),
            })
        }
        Expr::IntLit(i) => BExpr::IntLit(i.clone()),
        Expr::Binop(bin) => {
            let a = convert_expr(&bin.a, env_chain)?;
            let b = convert_expr(&bin.b, env_chain)?;
            BExpr::Binop(BBinop {
                a: Box::new(a),
                op: bin.op,
                b: Box::new(b),
            })
        }
        Expr::Ident(ident) => {
            let sym = env_chain
                .lookup_variable(ident)
                .map_err(|e| ExprError::Env(e))?;
            BExpr::Var(sym)
        }
    };
    Ok(r)
}
