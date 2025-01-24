// TODO: Fix the function body without changing the signature.
fn square(num: i32) -> i32 {
    // SOLUTION: si ajout du return, nécessaire de mettre le ; à la fin de la ligne, sinon pas besoin de return et retourner seulement la valeur
    num * num
}

fn main() {
    let answer = square(3);
    println!("The square of 3 is {answer}");
}
