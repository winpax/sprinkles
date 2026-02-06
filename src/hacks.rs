#![allow(
    clippy::single_component_path_imports,
    edition_2024_expr_fragment_specifier
)]

#[macro_use]
mod hackros {
    macro_rules! let_chain {
    (let $dis:ident($pat:ident) = $expr:expr_2021; $(let $dis2:ident($pat2:ident) = $expr2:expr_2021 ;)+ $then:expr_2021 $(; else $else:expr_2021)?) => {{
        #[allow(if_let_rescope)]
        if let $dis($pat) = $expr {
            let_chain!($(let $dis2($pat2) = $expr2 ;)+ $then $(; else $else)?)
        }
        $(else { $else })?
    }};

    (let $dis:ident($pat:ident) = $expr:expr_2021; $then:expr_2021 $(; else $else:expr_2021)?) => {{
        #[allow(if_let_rescope)]
        if let $dis($pat) = $expr {
            $then
        }
        $(else { $else })?
    }};
}

    #[cfg(test)]
    mod tests {
        #[test]
        fn test_let_chain() {
            let result = let_chain!(let Some(x) = Some(1); let Some(y) = Some(2); let Some(z) = Some(3); {
            (x, y, z)
        }; else panic!("nope"));

            assert_eq!(result, (1, 2, 3));
        }
    }
}

#[allow(unused_imports)]
pub(crate) use let_chain;
