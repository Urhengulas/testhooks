extern crate proc_macro;

mod analyze;
mod codegen;
mod lower;
mod parse;

use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;

use crate::{analyze::analyze, codegen::codegen, lower::lower, parse::parse};

#[proc_macro_attribute]
#[proc_macro_error]
pub fn testhooks(args: TokenStream, item: TokenStream) -> TokenStream {
    let ast = parse(args.into(), item.into());
    let model = analyze(ast);
    let ir = lower(model);
    let rust = codegen(ir);
    rust.into()
}
