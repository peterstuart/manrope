use proc_macro::TokenStream;
use proc_macro2::{Literal, Span};
use quote::quote;
use syn::{
    self,
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    Error, ExprLit, Ident, Lit, Token,
};

struct GenerateFontsInput {
    name: String,
    min_size: u32,
    max_size: u32,
}

impl Parse for GenerateFontsInput {
    fn parse(input: ParseStream) -> Result<Self, Error> {
        let args = Punctuated::<ExprLit, Token![,]>::parse_terminated(input)?;

        let name = match &args[0].lit {
            Lit::Str(litstr) => litstr.value(),
            _ => panic!(),
        };

        let min_size = match &args[1].lit {
            Lit::Int(litint) => litint.base10_parse()?,
            _ => panic!(),
        };

        let max_size = match &args[2].lit {
            Lit::Int(litint) => litint.base10_parse()?,
            _ => panic!(),
        };

        Ok(GenerateFontsInput {
            name,
            min_size,
            max_size,
        })
    }
}

#[proc_macro]
pub fn generate_fonts(token_stream: TokenStream) -> TokenStream {
    let input = parse_macro_input!(token_stream as GenerateFontsInput);

    let mut fonts = quote!();

    for size in input.min_size..=input.max_size {
        let path = Literal::string(&format!("bdfs/{}-{}.bdf", input.name, size));
        let name = Ident::new(
            &format!(
                "{}_{}_POINT",
                input.name.to_uppercase().replace('-', "_"),
                size
            ),
            Span::call_site(),
        );
        let font = quote! {
            pub const #name: eg_bdf::BdfFont = eg_bdf::include_bdf!(#path);
        };

        fonts.extend(font);
    }

    fonts.into()
}
