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
        let part = input.parse()?;
        Ok(Args { year, day, part })
    }
}

#[proc_macro_attribute]
pub fn challenge(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = syn::parse_macro_input!(attr as Args);
    let item = syn::parse_macro_input!(item as syn::ItemFn);

    let year = args.year;
    let day = args.day;
    let part = args.part;

    let part_val = part.to_string().parse::<usize>().unwrap();
    assert!(part_val == 1 || part_val == 2);

    let funcname = &item.sig.ident;

    let module = quote! {
        mod #funcname {
            #[::linkme::distributed_slice(::bddap_aoc::CHALLENGES)]
            static __: ::bddap_aoc::Challenge = {
                fn #funcname(a: &str) -> String {
                    super::#funcname(a).to_string()
                }

                ::bddap_aoc::Challenge {
                    year: #year,
                    day: #day,
                    part: #part,
                    run: #funcname,
                }
            };
        }

        #item
    };

    module.into()
}
