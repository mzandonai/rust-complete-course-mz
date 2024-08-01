use std::io;
use std::io::Write;

fn main() {
    // Chiedi all'utente di inserire il primo numero
    print!("Inserisci il primo numero: ");
    io::stdout().flush().unwrap(); // Assicurati che il prompt venga stampato

    // Crea una stringa vuota per contenere l'input
    let mut x: String = String::new();
    io::stdin()
        .read_line(&mut x)
        .expect("Errore nella lettura dell'input x");

    // Chiedi all'utente di inserire il secondo numero
    print!("Inserisci il secondo numero: ");
    io::stdout().flush().unwrap(); // Assicurati che il prompt venga stampato

    let mut y: String = String::new();
    io::stdin()
        .read_line(&mut y)
        .expect("Errore nella lettura dell'input y");

    // Rimuovi gli spazi bianchi iniziali e finali dall'input e prova a convertirlo in un numero
    let x_parsed: i32 = match x.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Per favore, inserisci un numero valido per x");
            return;
        }
    };

    let y_parsed: i32 = match y.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Per favore, inserisci un numero valido per y");
            return;
        }
    };

    // Calcola la somma dei numeri
    let z: i32 = calculator(x_parsed, y_parsed);
    println!("La somma dei due numeri è: {}", z);
}

fn calculator(x: i32, y: i32) -> i32 {
    x + y
}
