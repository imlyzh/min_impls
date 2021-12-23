use std::collections::{HashMap, HashSet, VecDeque};

use crate::ir::{FunDef, Inst, BasicBlock};


fn get_all_veriable(fun_def: &FunDef) -> Vec<String> {
    let mut vars = Vec::new();
    for bb in fun_def.bbs.iter() {
        for inst in bb.instrs.iter() {
            match inst {
                Inst::Add(dst, _, _) => {
                    vars.push(dst.clone());
                }
                _ => (),
            }
        }
    }
    vars
}

fn fun_ana(fun_def: &FunDef) -> HashMap<Option<String>, HashMap<String, bool>> {
    let all_var = get_all_veriable(fun_def);
    let all_var = all_var.into_iter().map(|x| (x, false)).collect::<HashMap<_, _>>();
    let mut old  = fun_def.bbs.iter().map(|x| (x.label.clone(), all_var.clone())).collect::<HashMap<_, _>>();
    loop {
        let new = one_pass(fun_def, old.clone());
        if old == new {
            break;
        } else {
            old = new;
        }
    }
    old
}

fn one_pass(fun_def: &FunDef, mut inp: HashMap<Option<String>, HashMap<String, bool>>) -> HashMap<Option<String>, HashMap<String, bool>> {
    let mapping = fun_def.bbs.iter().enumerate()
        .map(|(offset, x)| (x.label.clone(), offset)).collect::<HashMap<_, _>>();
    let mut next_set: VecDeque<Option<String>> = VecDeque::new();
    let mut used_bb: HashSet<Option<String>> = HashSet::new();
    next_set.push_back(fun_def.bbs[0].label.clone());
    while next_set.is_empty() {
        let task = next_set.pop_front().unwrap();
        if !used_bb.contains(&task) {
            let bb_size = mapping.get(&task).unwrap();
            let bb = &fun_def.bbs[*bb_size];
            inp.insert(task.clone(), basicblock_ana(bb, inp.get(&task).unwrap().clone()));
            if let Some(x) = bb.get_next() {
                for next in x.iter() {
                    next_set.push_back(Some(next.clone().clone()));
                }
                if x.is_empty() && bb_size + 1 < fun_def.bbs.len() {
                    next_set.push_back(fun_def.bbs[bb_size + 1].label.clone());
                }
            } else {
                return inp;
            }
            used_bb.insert(task);
        }
    }
    todo!()
}

fn basicblock_ana(bb: &BasicBlock, mut inp: HashMap<String, bool>) -> HashMap<String, bool> {
    for inst in bb.instrs.iter() {
        match inst {
            Inst::Add(_, v0, v1) => {
                inp.insert(v0.clone(), true);
                inp.insert(v1.clone(), true);
            },
        }
    }
    inp
}