use crate::ast::*;
use crate::bast::bound_ast::*;
use crate::bast::env::*;

pub enum ASTError {
    BadFunction(FnDeclError),
    BadExpr(ExprError),
    Env(EnvError),
    BadArity,
    WrongType,
}

impl From<EnvError> for ASTError {
    fn from(value: EnvError) -> Self {
        Self::Env(value)
    }
}

pub enum ExprError {
    Env(EnvError),
    MismatchedTypes,
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
                let mut assign = if let Some(e) = &d.assign {
                    convert_expr_wrapped(e, &env_chain.with(&current_env))?
                } else {
                    AnnotatedExpr {
                        typ: d.typ,
                        body: BExpr::default_value(d.typ),
                    }
                };

                if !assign.cast_to(d.typ) {
                    return Err(ASTError::WrongType);
                }

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
                let snapshot = env_chain.with(&current_env);

                let (ident, finfo) = snapshot.get_function_from_ident(&c.ident)?;

                if c.params.len() != finfo.args.len() {
                    return Err(ASTError::BadArity);
                }

                let mut args: Vec<AnnotatedExpr> = Vec::with_capacity(c.params.len());
                for i in 0..c.params.len() {
                    let expected_type = finfo.args[i];
                    let mut expr = convert_expr_wrapped(&c.params[i], &snapshot)?;
                    if !expr.cast_to(expected_type) {
                        return Err(ASTError::WrongType);
                    }
                    args.push(expr);
                }

                stmts.push(BStmt::Call(BCall { ident, args }));
            }

            Statement::Function(f) => {
                let snapshot = env_chain.with(&current_env);
            }
            _ => todo!(),
        };
    }

    todo!()
}

pub fn convert_expr_wrapped(expr: &Expr, env_chain: &EnvChain) -> Result<AnnotatedExpr, ASTError> {
    convert_expr(expr, env_chain).map_err(|e| ASTError::BadExpr(e))
}

// Maybe better to have BExpr implement an annotate method
pub fn convert_expr(expr: &Expr, env_chain: &EnvChain) -> Result<AnnotatedExpr, ExprError> {
    let r: AnnotatedExpr = match expr {
        Expr::Unary(u) => {
            let subexpr = convert_expr(&u.x, env_chain)?;
            let typ = subexpr.typ;
            let body = BExpr::Unary(BUnary {
                op: u.op,
                expr: Box::new(subexpr),
            });

            AnnotatedExpr { typ, body }
        }
        Expr::IntLit(i) => AnnotatedExpr {
            typ: LType::Int,
            body: BExpr::IntLit(i.clone()),
        },
        Expr::Binop(bin) => {
            let a = convert_expr(&bin.a, env_chain)?;
            let b = convert_expr(&bin.b, env_chain)?;
            if a.typ != b.typ {
                return Err(ExprError::MismatchedTypes);
            }

            let typ = a.typ;

            let body = BExpr::Binop(BBinop {
                a: Box::new(a),
                op: bin.op,
                b: Box::new(b),
            });
            AnnotatedExpr { typ, body }
        }
        Expr::Ident(ident) => {
            let sym = env_chain
                .lookup_variable(ident)
                .map_err(|e| ExprError::Env(e))?;
            let assoc = env_chain.get_variable(sym).map_err(|e| ExprError::Env(e))?;
            let body = BExpr::Var(sym);
            AnnotatedExpr {
                typ: assoc.typ,
                body,
            }
        }
    };
    Ok(r)
}
