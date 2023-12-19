# Variabili e tipi dati

# Variabili

```bash
let [mut] nome[:tipo] [= valore];
```

> - `[mut]` indica che la parola chiave `mut` è opzionale.
> - `[:tipo]` indica che la dichiarazione del tipo è opzionale.
> - `[= valore]` indica che l'inizializzazione è opzionale.

Le variabili in Rust vengono dichiarate con la parola chiave `let`:

```rust
let x = 5;
```

La tipizzazione può essere implicita (vedi esempio sopra) o esplicita:

```rust
let x: i32 = 5;
```

E' possibile dichiarare una variabile ed inizializzarla successivamente:

```rust
let x;
x = 5;
```

Il problema di questo approccio è possibile accedere alla variabile solo dopo averla inizializzata.

```rust
let x;
let y = x; // Errore
x = 5;
```

In C e C++ è possibile dichiarare una variabile senza inizializzarla, ma in Rust non è possibile. Questo perché Rust non permette di accedere a dati non inizializzati (quindi restituisce un errore di compilazione).

## Tipi di dato

### Interi

Gli interi sono numeri senza virgola. Possono essere firmati o non firmati. I tipi di dato interi sono:

| Lunghezza | Firmato | Non firmato |
| --------- | ------- | ----------- |
| 8-bit     | `i8`    | `u8`        |
| 16-bit    | `i16`   | `u16`       |
| 32-bit    | `i32`   | `u32`       |
| 64-bit    | `i64`   | `u64`       |
| 128-bit   | `i128`  | `u128`      |
| arch      | `isize` | `usize`     |

Il tipo `isize` e `usize` dipendono dall'architettura del sistema operativo. Se il sistema operativo è a 64-bit, allora `isize` e `usize` saranno a 64-bit. Se il sistema operativo è a 32-bit, allora `isize` e `usize` saranno a 32-bit.

### Altri tipi numerici

| Contenuto                       | Tipo   | Lunghezza |
| ------------------------------- | ------ | --------- |
| Single precision floating point | `f32`  | 32-bit    |
| Double precision floating point | `f64`  | 64-bit    |
| Booleano                        | `bool` | 1-bit     |
| Caratteri Unicode               | `char` | 4-bit     |

## (Im)mutabilità

Le variabili sono immutabili per default. Questo significa che non è possibile modificare il valore di una variabile dopo averlo assegnato. Per dichiarare una variabile mutabile, è necessario aggiungere la parola chiave `mut` prima del nome della variabile:

```rust
let mut x = 5;
x = 10;
```

## Costanti

Le costanti sono dichiarate con la parola chiave `const`:

```rust
const MAX_POINTS: u32 = 100_000;
```

Può sembrare una ridondanza avere sia le costanti che le variabili immutabili. Una variabile immutabile non può essere modificata dopo l'assegnazione, ma una costante non può essere modificata mai. Le costanti sono dichiarate solo nel contesto globale e non possono essere dichiarate all'interno di una funzione. Es di costante è pigreco mentre es di variabile immutabile è startTime che deve variare ogni esecuzione del software.

## Funzione variadica

Una funzione variadica è una funzione che può accettare un numero variabile di argomenti. In Rust, le funzioni variadiche sono implementate usando i parametri di tipo variadico. I parametri di tipo variadico sono parametri di tipo che possono accettare un numero variabile di argomenti.

Pro: sono molto flessibili

Contro: non sappiamo bene cosa accettano e cosa restituiscono

## Macro

Le macro sono simili alle funzioni, ma sono chiamate con un `!` alla fine e possono fare cose che le funzioni non possono fare. Le macro sono utilizzate per definire nuove sintassi. Le macro sono chiamate con un `!` alla fine del nome della macro.

```rust
let a = 2, b = 4;
println!("{a} il doppio è {b}");
println!("{} il doppio è {}", a, b);

// print_integer(a);
// print_string(" il doppio è ");
// print_integer(b);
// print_character('\n');
```

Poichè il compilatore conosce i tipi di ogni variabile in fase di compilazione (in Rust non esiste il dynamic typing), è possibile definire una macro che accetta un numero variabile di argomenti e che stampa il valore di ogni argomento e il suo tipo:

```rust
macros print_type_of($var: expr) {
    println!("{} è di tipo {}", stringify!($var), std::any::type_name_of_val(&$var));
}

let a = 2, b = 4;
print_type_of!(a);
print_type_of!(b);
```

## Funzioni

Le funzioni sono definite con la parola chiave `fn`:

```rust
fn main() {
    println!("Hello, world!");
}
```

Le funzioni possono avere parametri e possono anche non ammetere alcun parametro:

```rust
fn main() {
    println!("Hello, world!");
    another_function();
}

fn no_parameters(void) {
    println!("Another function.");
}
```

Le funzioni possono restituire un valore:

```rust
fn main() {
    println!("The value of five is: {}", five());
}

fn five() -> i32 {
    5
}
```

Il tipo di dato restituito è dichiarato dopo la freccia `->`. La funzione `five` restituisce il valore `5` che è un valore di tipo `i32`. Se la funzione non restituisce un valore, allora il tipo di dato restituito è `void`:

## Commenti

I commenti in Rust sono simili ai commenti in C++:

```rust
// Questo è un commento su una linea

/*
Questo è un commento
su più linee
*/
```

Commenti di documentazione:

```rust
/// Questo è un commento di documentazione su una linea

/** Questo è un commento di documentazione
 *  su più linee
 */
```
