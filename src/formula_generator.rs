use crate::node::*;
use rand::prelude::*;

pub fn random_rpn_expr(maxdepth: u32, maxvars: usize) -> String {
    let vals = (b'A'..=b'A' + (thread_rng().gen_range(0..maxvars) as u8))
        .map(|x| x as char)
        .collect::<Vec<_>>();
    if maxdepth == 0 {
        return "".to_string();
    }
    random_node(&vals, maxdepth).to_string()
}

fn random_node(vals: &[char], maxdepth: u32) -> Node {
    use BinOp::*;
    use Node::*;

    if maxdepth == 1 {
        return Var(vals[random::<usize>() % vals.len()]);
    }
    let n = if maxdepth >= 5 {
        random::<usize>() % 6 + 1
    } else {
        random::<usize>() % 7
    };
    match n {
        0 => Var(vals[random::<usize>() % vals.len()]),
        1 => Not(Box::new(random_node(vals, maxdepth - 1))),
        n => Binary {
            op: match n {
                2 => And,
                3 => Or,
                4 => Xor,
                5 => Impl,
                _ => Leq,
            },
            left: Box::new(random_node(vals, maxdepth - 1)),
            right: Box::new(random_node(vals, maxdepth - 1)),
        },
    }
}
