use quote::{ToTokens, TokenStreamExt, __private::Span};
use syn::{parse::Parse, Ident, Token, LitInt};

use crate::alias::Dims;

use super::{Args, Arr};

impl Parse for Args {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let alias_type: Ident = input.parse()?;
        dbg!(&alias_type);
        let _: Token![,] = input.parse()?;
        let element_type: Ident = input.parse()?;
        dbg!(&element_type);
        let _: Token![,] = input.parse()?;
        let dims: Dims = input.parse()?;
        dbg!(&dims);
        let _: Option<Token![,]> = input.parse()?;
        let arr: Arr = Arr {
            element_type,
            dims,
        };
        Ok(Args {
            alias_type,
            arr,
        })
        // let err = syn::Error::new_spanned(
        //     input.cursor().token_stream(),
        //     format!("make Args from {input:#?}")
        // );
        // Err(err)
    }
}

impl ToTokens for Args {
    fn to_tokens(&self, tokens: &mut quote::__private::TokenStream) {
        // this will look like `type MyArr = [[[f32; 4]; 3]; 2];`
        let Self {
            alias_type, // `MyArr`
            arr,   // `Dims([2, 3, 4])`
        } = self;
        let new_tokens = quote::quote_spanned! { Span::mixed_site() =>
            type #alias_type = #arr;
        };
        dbg!(&new_tokens.to_string());
        tokens.extend(new_tokens)
    }
}

impl Parse for Dims {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        // e.g., convert `2 * 3 * 4` to a Vec of 3 elements
        // c.f., https://docs.rs/syn/latest/syn/all.html
        let dim: LitInt = input.parse()?;
        let dim: usize = dim.base10_parse()?;
        dbg!(&dim);
        let mut dims: Vec<usize> = vec![dim];
        while input.peek(Token![*]) {
            let _: Token![*] = input.parse()?;
            let dim: LitInt = input.parse()?;
            let dim: usize = dim.base10_parse()?;
            dbg!(&dim);
            dims.push(dim);
        }
        Ok(Dims(dims))
        // let err = syn::Error::new_spanned(
        //     input.cursor().token_stream(),
        //     format!("make Dims from {input:#?}")
        // );
        // Err(err)
    }
}

impl ToTokens for Arr {
    fn to_tokens(&self, tokens: &mut quote::__private::TokenStream) {
        // this will look like `[[[f32; 4]; 3]; 2]`
        let Self {
            element_type, // `f32`
            dims,   // `Dims([2, 3, 4])`
        } = self;
        let mut new_tokens = quote::quote_spanned! { Span::mixed_site() =>
            #element_type
        };
        dims.0.iter().rev().for_each(|dim| {
            new_tokens = quote::quote_spanned! { Span::mixed_site() =>
                [#new_tokens; #dim]
            };
        });
        // let x: usize = 4usize;
        // let y: u8 = 204u8;
        dbg!(&new_tokens.to_string());
        tokens.extend(new_tokens)
    }
}