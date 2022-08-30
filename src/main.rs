mod formula_generator;
mod node;

use crate::formula_generator::random_rpn_expr;
use clap::*;

#[derive(Parser)]
#[clap(name = "formula_generator", about)]
struct Opts {
    #[clap(value_parser, help = "The maximum depth of the expression tree.")]
    max_depth: u32,
    #[clap(
        value_parser,
        help = "The maximum number of variables in the expression."
    )]
    max_var: usize,
}

fn main() {
    // println!("{}", random_rpn_expr(3, 3));
    let opts = Opts::parse();
    println!("{}", random_rpn_expr(opts.max_depth, opts.max_var));
}
