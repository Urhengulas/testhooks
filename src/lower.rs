use syn::{parse_quote, Item, ItemFn, ItemMod, Stmt};

use crate::Model;

pub type Ir = ItemMod;

pub fn lower(model: Model) -> Ir {
    let Model {
        mut module,
        tests,
        untouched_tokens,
    } = model;

    let tests = insert_triggers(tests);

    // chain all items together
    let items = vec![global()]
        .into_iter()
        .chain(tests)
        .chain(untouched_tokens)
        .collect();

    // and add items back to the module
    let (brace, _) = module.content.unwrap();
    module.content = Some((brace, items));
    module
}

fn insert_triggers(tests: Vec<ItemFn>) -> Vec<Item> {
    let trigger: Stmt = parse_quote!(
        if GLOBAL.eq(&()) {};
    );

    tests
        .into_iter()
        .map(|mut test| {
            test.block.stmts.insert(0, trigger.clone());
            Item::Fn(test)
        })
        .collect()
}

fn global() -> Item {
    // TODO: once_cell::Lazy
    parse_quote!(
        static GLOBAL: () = ();
    )
}
