fn call_me(num: u8) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}

fn main() {
    // TODO: Fix the function call.
    // SOLUTION: si type i8, alors -1 est un nombre négatif, donc erreur de compilation, il faut mettre entre 0 et 255 car 2 puissance 8
    call_me(4);
}
