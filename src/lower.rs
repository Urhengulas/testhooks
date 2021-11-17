use syn::{parse_quote, ItemFn, ItemMod, ItemStatic, Stmt};

use crate::analyze::Model;

pub struct Ir {
    pub global: ItemStatic,
    pub module: ItemMod,
    pub tests: Vec<ItemFn>,
}

pub fn lower(model: Model) -> Ir {
    let Model { module, tests } = model;

    Ir {
        global: global(),
        module,
        tests: insert_triggers(tests),
    }
}

fn insert_triggers(tests: Vec<ItemFn>) -> Vec<ItemFn> {
    let trigger: Stmt = parse_quote!(
        if GLOBAL.eq(&()) {};
    );

    tests
        .into_iter()
        .map(|mut test| {
            test.block.stmts.insert(0, trigger.clone());
            test
        })
        .collect()
}

fn global() -> ItemStatic {
    // TODO: once_cell::Lazy
    parse_quote!(
        static GLOBAL: () = ();
    )
}
