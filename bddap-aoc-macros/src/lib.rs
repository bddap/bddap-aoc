use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    Token,
};

struct Args {
    year: syn::LitInt,
    day: syn::LitInt,
    part: syn::LitInt,
}

impl Parse for Args {
    fn parse(input: ParseStream) -> syn::parse::Result<Self> {
        let year = input.parse()?;
        input.parse::<Token![,]>()?;
        let day = input.parse()?;
        input.parse::<Token![,]>()?;
        let part: syn::LitInt = input.parse()?;

        let part_val = part.to_string();
        if part_val != "1" && part_val != "2" {
            return Err(syn::Error::new(part.span(), "part must be 1 or 2"));
        }

        // optional comma
        input.parse::<Token![,]>().ok();
        Ok(Args { year, day, part })
    }
}

fn as_challege_struct(args: Args, func: syn::ItemFn) -> syn::ExprStruct {
    let year = args.year;
    let day = args.day;
    let part = args.part;
    let funcname = &func.sig.ident;

    syn::parse_quote! {
        ::bddap_aoc::Challenge {
            name: stringify!(#funcname),
            year: #year,
            day: #day,
            part: #part,
            run: {
                #func
                fn run(input: &str) -> String {
                    ::std::string::ToString::to_string(& #funcname ( input ))
                }
                run
            },
        }
    }
}

/// Marks a function as a solution to an Advent of Code challenge.
///
/// The function must have the signature `fn() -> String`.
///
/// # Example
///
/// ```rust
/// #[bddap_aoc::register(2020, 1, 1)]
/// fn part1() -> String {
///     "Hello, world!".to_string()
/// }
/// ```
#[proc_macro_attribute]
pub fn register(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = syn::parse_macro_input!(attr as Args);
    let fn_def = syn::parse_macro_input!(item as syn::ItemFn);
    let challenge = as_challenge(args, fn_def);
    quote! {
        #[::bddap_aoc::linkme::distributed_slice(::bddap_aoc::CHALLENGES)]
        #challenge
    }
    .into()
}

/// Declare a challenge function without registering it.
///
/// ```rust
/// #[bddap_aoc::unregistered_challenge(2020, 1, 1)]
/// fn part1() -> String {
///     "Hello, world!".to_string()
/// }
/// ```
#[proc_macro_attribute]
pub fn unregistered_challenge(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = syn::parse_macro_input!(attr as Args);
    let fn_def = syn::parse_macro_input!(item as syn::ItemFn);
    let declaration = as_challenge(args, fn_def);
    quote!(#declaration).into()
}

fn as_challenge(args: Args, fn_def: syn::ItemFn) -> syn::ItemStatic {
    let fn_name = fn_def.sig.ident.clone();
    let visibility = fn_def.vis.clone();
    let struct_literal = as_challege_struct(args, fn_def);
    syn::parse_quote! {
        #[allow(non_upper_case_globals)]
        #visibility static #fn_name: ::bddap_aoc::Challenge = #struct_literal;
    }
}
