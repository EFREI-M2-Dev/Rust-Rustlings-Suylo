// TODO: Add the missing type of the argument `num` after the colon `:`.
// SOLUTION: ajouter le type de num, i32 pour < & > 0, ou u32 pour >= 0
fn call_me(num: i32) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}

fn main() {
    call_me(-1);
}
