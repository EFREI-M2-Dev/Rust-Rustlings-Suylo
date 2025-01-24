fn main() {
    // TODO: Create an array called `a` with at least 100 elements in it.
    // let a = ???
    // SOLUTION: mettre un point virgule pour créer un tableau contenant 0 à 100 valeurs de 0
    let a = [0; 100];

    println!("{:?}", a);

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed");
    }
}
