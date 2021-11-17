use syn::{Item, ItemFn, ItemMod};

use crate::parse::Ast;

pub struct Model {
    pub module: ItemMod,
    pub tests: Vec<ItemFn>,
}

pub fn analyze(ast: Ast) -> Model {
    let mut module = ast;

    let (brace, items) = module.content.unwrap_or_else(|| {
        // TODO: proc-macro-error
        todo!()
    });

    // Die Guten ins Töpfchen, die Schlechten ins Kröpfchen
    let mut tests = vec![];
    let mut untouched_tokens = vec![];
    for item in items {
        match item {
            Item::Fn(f) if f.is_test() => tests.push(f),
            _ => untouched_tokens.push(item),
        }
    }

    // add brace and untouched_tokens back into the module
    module.content = Some((brace, untouched_tokens));

    Model { module, tests }
}

trait IsTest {
    /// Is this [`ItemFn`] a test function?
    fn is_test(&self) -> bool;
}

impl IsTest for ItemFn {
    fn is_test(&self) -> bool {
        self.attrs.iter().any(|attr| attr.path.is_ident("test"))
    }
}

#[cfg(test)]
mod tests {
    use syn::parse_quote;

    use super::*;

    #[test]
    fn can_extract_precondition() {
        let model = analyze(parse_quote!(
            #[cfg(test)]
            #[testhooks]
            mod tests {
                #[test]
                fn it_works() {}
            }
        ));
        let expected: &[ItemFn] = &[parse_quote!(
            #[test]
            fn it_works() {}
        )];
        assert_eq!(expected, model.tests);
    }

    #[test]
    fn attributes_are_preserved() {
        let model = analyze(parse_quote!(
            #[cfg(test)]
            #[testhooks]
            mod tests {
                #[test]
                #[should_panic]
                fn it_works() {
                    panic!()
                }
            }
        ));
        let expected: &[ItemFn] = &[parse_quote!(
            #[test]
            #[should_panic]
            fn it_works() {
                panic!()
            }
        )];
        assert_eq!(expected, model.tests);
    }
}
