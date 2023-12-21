# Analisi grammaticale

## use std::io

```rust
use std::io; // E' un modulo
```

Con `use` si importa un modulo. In questo caso il **modulo** `io` del crate `std`.

## prelude

Implicitamente il compilatore rende disponibile al programmatore i tipi/funzione maggiormente utilzizati.

E' prassi normale, se necessario, aggiungere **prelude** ad un modulo per rendere disponibile al programmatore i tipi/funzione maggiormente utilizzati.

## fn main()

E' la funzione principale del programma.

```rust
fn main() {
    // ...
}
```

> Possono essere passati argomenti alla funzione `main()`.
> `std::env::args()` restituisce un iteratore sugli argomenti passati al programma.

## let & mut

`let` è uno statementche definisce una **variabile**.

`mut` indica che una variabile è **mutabile**.

In Rust non esiste una istruzione esplicita per istanziare un oggetto, a differenza di altri linguaggi che usano new. Si utilizzano quindi delle convenzioni.

`String::new()` è una funzione che restituisce una nuova istanza di `String` con una dimensione variabile.

La sintassi `::` indica che la funzione **new** alloca una nuova istanza di `String`.

Il compilatore **deduce** il tipo della variabile `guess` in base al valore restituito dalla funzione `String::new()`. E' comunque possibile specificare il tipo della variabile.

```rust

let secret_number = rand::thread_rng().gen_range(1..101);

let mut guess: String = String::new();
```

> In Rust forse è meglio non specificare il tipo della variabile, lasciando che il compilatore lo deduca.

## io::stdin().read_line(&mut guess).expect("Failed to read line");

`io::stdin()` ritorna un'istanza di `std::io::Stdin`.

`read_line(&mut guess)` invoca il metodo di `Stdin` che legge l'input dell'utente e lo memorizza in `guess`.

`&` indica che si sta passando un **reference** di `guess` a `read_line()`.

`expect("Failed to read line")` gestico, male, l'eccezione in caso di problemi nell'interazione stdin.

> In Rust non esistono eccezioni... Da approfondire.

## &mut guess

`&` indica che si sta passando un **reference** di `guess` a `read_line()`.

`mut` indica che `guess` è mutabile.
