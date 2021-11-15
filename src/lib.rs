extern crate proc_macro;

mod analyze;
mod lower;
mod parse;

use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;

use crate::{
    analyze::{analyze, Model},
    lower::lower,
    parse::{parse, Ast},
};

#[proc_macro_attribute]
#[proc_macro_error]
pub fn testhooks(args: TokenStream, item: TokenStream) -> TokenStream {
    let ast = parse(args.into(), item.into());
    let model = analyze(ast);
    let ir = lower(model);
    dbg!(ir);

    todo!()
}
