use crate::ast::*;
use std::collections::HashMap;

//AST w/symbols resolved

pub enum ASTError {
    BadFunction(FnDeclError),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct RelSymID {
    pub level: usize,
    pub id: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct RelVarID(pub RelSymID);
pub struct RelFnID(pub RelSymID);

pub struct VSymInfo {
    pub typ: LType,
    // TODO: Deprecate by forcing variable init on declare
    pub init: bool, // do not use unitialized variables
}

pub struct FSymInfo {
    pub out: LType,
    pub args: Vec<LType>, //params are always the first n symbols
    pub body: BAst,
}

pub struct BAstEnv {
    pub variables: Vec<VSymInfo>,
    pub functions: Vec<FSymInfo>,

    pub variable_history: HashMap<String, usize>,
    pub function_mappings: HashMap<String, usize>,
}

pub enum FnDeclError {
    DuplicateDecl,
    DuplicateParam,
}

impl BAstEnv {
    pub fn new() -> Self {
        Self {
            variables: Vec::new(),
            functions: Vec::new(),

            variable_history: HashMap::new(),
            function_mappings: HashMap::new(),
        }
    }

    pub fn new_variable(&mut self, ident: String, typ: LType) -> usize {
        let id = self.variables.len();
        let info = VSymInfo { typ, init: false };
        self.variables.push(info);
        self.variable_history.insert(ident, id);
        id
    }

    pub fn lookup_variable(&self, ident: &str) -> Option<usize> {
        self.variable_history.get(ident).copied()
    }

    pub fn variable_exists(&self, ident: &str) -> bool {
        self.lookup_variable(ident).is_some()
    }

    //TODO Unify params/args naming (see ast.rs)
    pub fn new_function(
        &mut self,
        ident: String,
        out: LType,
        params: &Vec<(LType, String)>,
    ) -> Result<usize, FnDeclError> {
        if self.function_mappings.get(&ident).is_some() {
            Err(FnDeclError::DuplicateDecl)
        } else {
            let id = self.functions.len();

            let mut fun_args: Vec<LType> = Vec::new();
            let mut fun_env = BAstEnv::new();

            for (typ, ident) in params.iter() {
                fun_args.push(typ.clone());
                if fun_env.variable_exists(ident) {
                    return Err(FnDeclError::DuplicateParam);
                }
                fun_env.new_variable(ident.clone(), typ.clone());
            }

            let fun_ast = BAst {
                environment: fun_env,
                statements: Vec::new(),
            };

            let fun_info = FSymInfo {
                out,
                args: fun_args,
                body: fun_ast,
            };

            self.functions.push(fun_info);

            Ok(id)
        }
    }
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
    Intermediate(i64),
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
}

pub enum EnvError {
    SymbolNotFound,
}

struct EnvChain<'a> {
    pub chain: Vec<&'a BAstEnv>,
}

impl<'a> EnvChain<'a> {
    pub fn new() -> Self {
        Self { chain: Vec::new() }
    }

    // chain is structured so idx 0 is always most recent env
    pub fn with<'b>(&'b self, env: &'b BAstEnv) -> EnvChain<'b>
    where
        'a: 'b,
    {
        let mut chain = Vec::with_capacity(self.chain.len() + 1);
        chain.push(env);
        chain.extend(self.chain.iter().copied());
        EnvChain { chain }
    }

    pub fn lookup_variable(&self, ident: &str) -> Result<RelVarID, EnvError> {
        self.lookup_variable_at(ident, 0)
    }

    pub fn lookup_variable_at(&self, ident: &str, level: usize) -> Result<RelVarID, EnvError> {
        let mut level = level;
        for env in self.chain.iter() {
            if let Some(vid) = Self::lookup_single_env(env, ident, level) {
                return Ok(vid);
            }
            level += 1;
        }
        Err(EnvError::SymbolNotFound)
    }

    pub fn lookup_variable_with_cur(
        &self,
        current_env: &BAstEnv,
        ident: &str,
    ) -> Result<RelVarID, EnvError> {
        if let Some(vid) = Self::lookup_single_env(current_env, ident, 0) {
            Ok(vid)
        } else {
            self.lookup_variable_at(ident, 1)
        }
    }

    fn lookup_single_env(env: &BAstEnv, ident: &str, level: usize) -> Option<RelVarID> {
        if let Some(id) = env.lookup_variable(ident) {
            Some(RelVarID(RelSymID { level, id }))
        } else {
            None
        }
    }
}

fn convert_statements(ast: &Vec<Statement>, env_chain: EnvChain) -> Result<BAst, ASTError> {
    let mut env = BAstEnv::new();
    let mut stmts: Vec<BStmt> = Vec::new();

    // load function definitions, then process all statements
    for stmt in ast.iter() {
        if let Statement::Function(f) = stmt {
            env.new_function(f.ident.clone(), f.ret, &f.params)
                .map_err(|e| ASTError::BadFunction(e))?;
        }
    }

    let n = env_chain.with(&env);

    for stmt in ast.iter() {
        match stmt {
            Statement::If(iff) => {
                todo!()
            }
            _ => todo!(),
        }
    }

    todo!()
}

pub enum ExprError {
    Env(EnvError),
}

pub fn convert_expr(expr: &Expr, env_chain: &EnvChain) -> Result<BExpr, ExprError> {
    let r = match expr {
        Expr::Unary(u) => {
            let subexpr = convert_expr(&u.x, env_chain)?;
            BExpr::Unary(BUnary {
                op: u.op,
                expr: Box::new(subexpr),
            })
        }
        _ => todo!(),
    };
    Ok(r)
}
