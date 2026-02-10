use crate::ast::*;
use crate::bast::bound_ast::*;
use std::collections::HashMap;

pub enum FnDeclError {
    DuplicateDecl,
    DuplicateParam,
}

pub enum EnvError {
    SymbolNotFound,
    BadReference,
}

#[derive(Debug, Copy, Clone)]
pub enum BAstEnvType {
    Variables,
    Functions,
}

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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct RelSymID {
    pub level: usize,
    pub id: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct RelVarID(pub RelSymID);
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct RelFnID(pub RelSymID);

impl RelVarID {
    pub fn new(level: usize, id: usize) -> Self {
        Self(RelSymID { level, id })
    }
}

pub struct BAstEnv {
    pub variables: Vec<VSymInfo>,
    pub functions: Vec<FSymInfo>,

    pub variable_history: HashMap<String, usize>,
    pub function_mappings: HashMap<String, usize>,
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

    pub fn get_env(&self, env: BAstEnvType) -> &HashMap<String, usize> {
        match env {
            BAstEnvType::Variables => &self.variable_history,
            BAstEnvType::Functions => &self.function_mappings,
        }
    }

    pub fn get_variable(&self, id: usize) -> Option<&VSymInfo> {
        self.variables.get(id)
    }

    pub fn get_function(&self, id: usize) -> Option<&FSymInfo> {
        self.functions.get(id)
    }

    // I think this is inefficient but idk
    // Also kind of unsafe because easy to provide the wrong id
    // TODO: wrap id in a struct where index is private
    pub fn swap_function_body(&mut self, id: usize, body: BAst) -> BAst {
        let s = &mut self.functions[id];
        std::mem::replace(&mut s.body, body)
    }

    pub fn lookup(&self, env: BAstEnvType, ident: &str) -> Option<usize> {
        self.get_env(env).get(ident).copied()
    }

    pub fn exists(&self, env: BAstEnvType, ident: &str) -> bool {
        self.lookup(env, ident).is_some()
    }

    pub fn new_variable(&mut self, ident: String, typ: LType) -> usize {
        let id = self.variables.len();
        let info = VSymInfo { typ, init: false };
        self.variables.push(info);
        self.variable_history.insert(ident, id);
        id
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
                if fun_env.exists(BAstEnvType::Variables, ident) {
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
            self.function_mappings.insert(ident.clone(), id);

            Ok(id)
        }
    }
}

pub struct EnvChain<'a> {
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

    fn lookup(&self, env_type: BAstEnvType, ident: &str) -> Result<RelSymID, EnvError> {
        self.lookup_at(env_type, ident, 0)
    }

    fn lookup_at(
        &self,
        env_type: BAstEnvType,
        ident: &str,
        level: usize,
    ) -> Result<RelSymID, EnvError> {
        let mut level = level;
        for env in self.chain.iter() {
            if let Some(id) = env.lookup(env_type, ident) {
                return Ok(RelSymID { level, id });
            }
            level += 1;
        }
        Err(EnvError::SymbolNotFound)
    }

    pub fn lookup_variable(&self, ident: &str) -> Result<RelVarID, EnvError> {
        Ok(RelVarID(self.lookup(BAstEnvType::Variables, ident)?))
    }

    pub fn get_variable(&self, id: RelVarID) -> Result<&VSymInfo, EnvError> {
        if let Some(env) = self.chain.get(id.0.level)
            && let Some(s) = env.get_variable(id.0.id)
        {
            Ok(s)
        } else {
            Err(EnvError::BadReference)
        }
    }

    pub fn get_variable_from_ident(&self, ident: &str) -> Result<(RelVarID, &VSymInfo), EnvError> {
        let loc = self.lookup_variable(ident)?;
        Ok((loc, self.get_variable(loc)?))
    }

    pub fn lookup_function(&self, ident: &str) -> Result<RelFnID, EnvError> {
        Ok(RelFnID(self.lookup(BAstEnvType::Functions, ident)?))
    }

    pub fn get_function(&self, id: RelFnID) -> Result<&FSymInfo, EnvError> {
        if let Some(env) = self.chain.get(id.0.level)
            && let Some(s) = env.get_function(id.0.id)
        {
            Ok(s)
        } else {
            Err(EnvError::BadReference)
        }
    }

    pub fn get_function_from_ident(&self, ident: &str) -> Result<(RelFnID, &FSymInfo), EnvError> {
        let loc = self.lookup_function(ident)?;
        Ok((loc, self.get_function(loc)?))
    }
}
