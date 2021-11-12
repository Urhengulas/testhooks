use proc_macro2::TokenStream as TokenStream2;

type Ast = syn::ItemMod;

pub fn parse(args: TokenStream2, item: TokenStream2) -> Ast {
    todo!()
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
