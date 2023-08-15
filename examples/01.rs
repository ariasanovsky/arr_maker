arr_maker_macros::alias!(
    MyArr,
    f32,
    2 * 3 * 4,
);
// will write
// type MyArr = [[[f32; 4]; 3]; 2];

fn main() {
    let arr: MyArr = Default::default();
    arr.into_iter().for_each(|row| println!("{row:?}"));
    // todo!("finish the macro")
}
