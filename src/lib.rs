extern crate proc_macro;

mod analyze;
mod parse;

use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;

use crate::{
    analyze::analyze,
    parse::{parse, Ast},
};

#[proc_macro_attribute]
#[proc_macro_error]
pub fn testhooks(args: TokenStream, item: TokenStream) -> TokenStream {
    let ast = parse(args.into(), item.into());
    let model = analyze(ast);
    dbg!(model);

    todo!()
}
