#![recursion_limit = "69"]

extern crate proc_macro;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{punctuated::Punctuated, spanned::Spanned, token::Comma};
use syn::{Block, Ident, ItemFn, Type};

fn parameters(function: &ItemFn) -> syn::Result<Vec<(Ident, Type)>> {
    use syn::FnArg::*;
    use syn::Pat;

    let new_error = |span, message| Some(Err(syn::Error::new(span, message)));

    function
        .decl
        .inputs
        .clone()
        .into_iter()
        .filter_map(|p| match p {
            Captured(c) => match c.pat {
                Pat::Ident(id) => Some(Ok((id.ident, c.ty))),
                _ => new_error(c.span(), "Parameters must be simple identifiers."),
            },
            SelfRef(_) | SelfValue(_) => new_error(p.span(), "Methods are not supported."),
            Inferred(_) => new_error(p.span(), "Parameters' types must be explicit."),
            Ignored(_) => None,
        })
        .collect()
}

fn return_type(function: &ItemFn) -> syn::Result<Type> {
    match function.decl.clone().output {
        syn::ReturnType::Type(_, boxed) => Ok(*boxed),
        _ => Err(syn::Error::new(
            function.block.span(),
            "There's no point in caching the output of \
             a function that doesn't return anything.",
        )),
    }
}

fn cache_initialization(
    cache_type: TokenStream,
    function: &ItemFn,
) -> syn::Result<(Ident, TokenStream)> {
    let key_type = parameters(&function)?
        .into_iter()
        .map(|(_id, ty)| ty)
        .collect::<Punctuated<_, Comma>>();

    let value_type = return_type(function)?;

    let function_name = function.ident.to_string();
    let cache_name = format!("_{}_CACHE", function_name.to_uppercase());
    let cache_ident = Ident::new(&cache_name, function.ident.span());

    let cache_init = quote! {
        thread_local!(
            static #cache_ident: std::cell::RefCell<
                #cache_type<(#(#key_type),*), #value_type>
            > = Default::default()
        );
    };

    Ok((cache_ident, cache_init))
}

macro_rules! unwrap_or_compile_error {
    ($result:expr) => {
        match $result {
            Ok(x) => x,
            Err(e) => return e.to_compile_error().into(),
        }
    };
}

#[proc_macro_attribute]
pub fn memoize(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let mut function = syn::parse_macro_input!(item as ItemFn);
    let mut wrapper_function = function.clone();

    let cache_type = if attr.is_empty() {
        quote!(std::collections::HashMap)
    } else {
        attr.into()
    };

    let cache_init = cache_initialization(cache_type, &function);
    let (cache_ident, cache_init) = unwrap_or_compile_error!(cache_init);

    function.ident = Ident::new(
        &format!("_{}_aux", function.ident.to_string()),
        function.ident.span(),
    );
    let function_ident = &function.ident;

    let parameter_idents = unwrap_or_compile_error!(parameters(&wrapper_function))
        .into_iter()
        .map(|(id, _ty)| id)
        .collect::<Punctuated<_, Comma>>();

    let key_parts = parameter_idents.clone();

    let new_block = quote! {
        {
            let key = (#(#key_parts.clone()),*);

            let cached = #cache_ident.with(|cell| {
                cell
                    .borrow_mut()
                    .get(&key)
                    .cloned()
            });

            if let Some(value) = cached {
                value.clone()
            } else {
                let value = #function_ident(#(#parameter_idents),*);

                #cache_ident.with(|cell| {
                    let mut cache = cell.borrow_mut();
                    cache.insert(key.clone(), value);
                    cache.get(&key).cloned().unwrap()
                })
            }
        }
    };

    let new_block = new_block.into();
    wrapper_function.block = Box::new(syn::parse_macro_input!(new_block as Block));

    let code = quote! {
        #cache_init
        #wrapper_function
        #function
    };

    code.into()
}
