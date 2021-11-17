use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::Item;

use crate::lower::Ir;

pub type Rust = TokenStream;

pub fn codegen(ir: Ir) -> Rust {
    let Ir {
        global,
        mut module,
        tests,
    } = ir;

    // SAFETY: `unwrap` won't panic, since we already accessed it during the `analyze`-phase
    let (brace, untouched_tokens) = module.content.unwrap();

    let items = untouched_tokens
        .into_iter()
        .chain(tests.into_iter().map(Item::Fn))
        .chain(vec![global].into_iter().map(Item::Static))
        .collect::<Vec<_>>();

    module.content = Some((brace, items));

    module.into_token_stream()
}
