use std::vec;



pub struct FunDef {
    pub name: String,
    pub args: Vec<String>,
    pub bbs: Vec<BasicBlock>,
}

impl FunDef {
    pub fn get_bb(&self, name: &str) -> Option<&BasicBlock> {
        self.bbs
            .iter()
            .find(|x| if let Some(x) = &x.label {
                x == name
            } else {
                false
            })
    }
}

#[derive(Debug, Clone)]
pub struct BasicBlock {
    pub label: Option<String>,
    pub instrs: Vec<Inst>,
    pub terminator: Option<Terminator>,
}

impl BasicBlock {
    pub fn get_next(&self) -> Option<Vec<&String>> {
        match &self.terminator {
            Some(Terminator::Goto(x)) => Some(vec![x]),
            Some(Terminator::If(_c, t, e)) => Some(vec![t, e]),
            Some(Terminator::Return) => None,
            _ => Some(vec![]),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Inst {
    // Alloca(String),
    Add(String, String, String),
}

#[derive(Debug, Clone)]
pub enum Terminator {
    Goto(String),
    If(String, String, String),
    Return,
}

