use quote::ToTokens;

pub(crate) mod alias;
// pub(crate) mod reorder;

#[proc_macro]
pub fn alias(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    use alias::Args;
    let args: Args = syn::parse_macro_input!(input as Args);
    args.to_token_stream().into()
}

// #[proc_macro_attribute]
// pub fn reorder(
//     attr: proc_macro::TokenStream,
//     input: proc_macro::TokenStream,
// ) -> proc_macro::TokenStream {
//     use reorder::Attrs;
//     use reorder::OriginalItem;
//     use reorder::ReorderedItem;
//     let attrs: Attrs = syn::parse_macro_input!(attr as Attrs);
//     let item: OriginalItem = syn::parse_macro_input!(input as OriginalItem);
//     let reordered_item: ReorderedItem = reorder::reorder(attrs, item);
//     reordered_item.to_token_stream().into()
// }