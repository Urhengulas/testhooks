extern crate proc_macro;

mod parse;

use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;

use crate::parse::parse;

#[proc_macro_attribute]
#[proc_macro_error]
pub fn testhooks(args: TokenStream, item: TokenStream) -> TokenStream {
    let _ast = parse(args.into(), item.into());

    todo!()
}
