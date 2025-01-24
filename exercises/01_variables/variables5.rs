fn main() {
    let number = "T-H-R-E-E"; // Don't change this line
    println!("Spell a number: {}", number);

    // TODO: Fix the compiler error by changing the line below without renaming the variable.
    // SOLUTION: changer la valeur de la variable pour un nombre, c'est possible car la variable n'est pas déclarée comme constante
    // Le type, même non défini n'influence pas la valeur de la variable
    let number = 3;
    println!("Number plus two is: {}", number + 2);
}
