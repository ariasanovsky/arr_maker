use syn::Ident;

#[cfg(test)]
mod test;
mod tokens;

#[derive(Debug)]
pub struct Args {
    alias_type: Ident,
    arr: Arr,
}

#[derive(Debug)]
pub struct Dims(Vec<usize>);

#[derive(Debug)]
pub struct Arr {
    dims: Dims,
    element_type: Ident,
}
