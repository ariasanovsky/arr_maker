use crate::alias::Args;

#[test]
fn parse_3d_float_array_and_return_correct_tokens() {
    // quote: parse TokenStream from 01.rs
    let input = quote::quote! {
        MyArr,
        f32,
        2 * 3 * 4,
    };
    dbg!(&input);
    // syn: parse TokenStream to Args
    let args: Args = syn::parse_quote! { #input };
    dbg!(&args);
    // emit output as TokenStream
    let output = quote::quote! {
        #args
    };
    dbg!(&output);
    // compare to expected output
    let expected = quote::quote! {
        type MyArr = [[[f32; 4usize]; 3usize]; 2usize];
    };
    assert_eq!(output.to_string(), expected.to_string());
}
