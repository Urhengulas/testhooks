use proc_macro2::TokenStream as TokenStream2;
use proc_macro_error::{abort, abort_call_site};
use syn::Item;

pub type Ast = syn::ItemMod;

pub fn parse(args: TokenStream2, item: TokenStream2) -> Ast {
    if !args.is_empty() {
        abort_call_site!("this attribute takes no arguments"; help = "use `#[testhooks]`")
    }

    match syn::parse2::<Item>(item) {
        Ok(Item::Mod(m)) => m,
        Ok(item) => {
            abort!(
                item,
                "item is not a module";
                help = "`#[testhooks]` can only be used on modules"
            )
        }
        // TODO: why is it unreachable?
        Err(_) => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use quote::quote;

    use super::*;

    #[test]
    fn valid_syntax() {
        parse(
            quote!(),
            quote!(
                #[cfg(test)]
                mod tests {
                    #[test]
                    fn it_works() {
                        let result = 2 + 2;
                        assert_eq!(result, 4);
                    }
                }
            ),
        );
    }
}
