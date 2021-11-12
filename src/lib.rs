extern crate proc_macro;

mod parse;

use proc_macro::TokenStream;

use crate::parse::parse;

#[proc_macro_attribute]
pub fn testhooks(args: TokenStream, item: TokenStream) -> TokenStream {
    let ast = parse(args.into(), item.into());

    todo!()
}
