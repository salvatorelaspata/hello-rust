# Flusso del programma

## If

Gli if permettono di eseguire codice condizionato da una specifica condizione.
Le condizioni rappresentano espressioni booleane.

- Variabili booleane
- Comparazioni: ==, !=, >, <, >=, <=
- Operatori logici: && (and), || (or), ! (not)

```rust
if condition {
    // ...
} else if condition {
    // ...
} else {
    // ...
}
```

> In Rust non esiste il concetto di **truthy** e **falsy**.
> Le parentesi tonde sono opzionali e non vengono utilizzate se l'espressione non lo richiede.

Il blocco if è possibile utilizzarlo anche come espressione.

```rust
let number = if condition { 5 } else { 6 };
```

## Iterazioni

Il loop permette di eseguire il codice più di una volta

In rust esistono 3 tipi di loop:

- loop
- while
- for

### loop

Per eseguire il codice all'interno del loop è necessario utilizzare la keyword `break`.

Il blocco loop può essere utilizzato anche anche come espressione per l'inizializzazione di una variabile.

```rust
let mut counter = 0;
loop {
    if counter >= 10 { // break condition
        break;
    }

    counter += 1;
}

let result = loop {
    if counter >= 10 { // break condition
        break counter * 2;
    }

    counter += 1;
};
```

### while

Fin tanto che la condizione è vera il codice all'interno del while viene eseguito.

```rust
let mut counter = 3;

while counter > 0 {
    printerln("{}...", counter);
    counter -= 1;
}

println("GO!")
```

### for

Come in altri linguaggi il for permette di iterare su una collezione.

```rust
for counter in 1..4.rev() {
    println!("{}...", counter);
}

println("GO!")
```

Il costrutto (x..y) in Rust permette di creare un iteratore
.rev() è un metodo che permette di invertire l'ordine del range.

## Funzioni

Sono un'insieme di istruzioni che permettono di essere riutilizzate. Le funzioni possono avere parametri e restituire valori.

Le funzioni in rust possono essere dichiarati sia prima che dopo il main.

Vengono definite con la parola chiave `fn`.

```rust
fn main() {
    println!("Hello, world!");
}
```

> Rust utilizza lo snake case per la definizione dei nomi delle funzioni.
> Il nome delle funzioni devono usare gli underscore per separare le parole.

### Parametri

I parametri sono gli input di una funzione.

```rust
fn add(x: i32, y: i32) {
    let a = 5, b = 2;
    a + b;
}
```

> I parametri sono immutabili per default.
> Per rendere un parametro mutabile è necessario utilizzare la keyword `mut`.

### Return

Il valore di ritorno di una funzione è definito dalla keyword `return`.

Il tipo di ritorno di una funzione è definito dopo la freccia `->`.

```rust
fn add(x: i32, y: i32) -> i32 {
    return x + y;
}
```

> Il return è opzionale.
> Se non viene specificato il valore di ritorno è l'ultimo valore dell'ultima espressione. (omettendo il punto e virgola)

```rust
fn add(x: i32, y: i32) -> i32 {
    x + y
}
```

### Funzione ricorsiva

Il classico esempio è la funzione di fibonacci.

```rust
fn fibonacci(n: i32) -> i32 {
    if n <= 1 {
        return 1;
    }

    fibonacci(n - 1) + fibonacci(n - 2)
}
```

## Array

Gli array sono una collezione di elementi dello stesso tipo.

```rust
let a: [usize; 5] = [1, 2, 3, 4, 5];
let b: [usize; 5] = [0; 5];
let c = b; // non è un riferimento ma una copia fisica

println!(a[4]); // 4
println!(b[4]); // 0
```

> Gli array passato per valore quando vengono copiati.

La dimensione dell'array è nota in fase di compilazione.

Per accedere ad un elemento dell'array è necessario utilizzare la sintassi `[]`. O tramite gli iteratori.

```rust
let a: [usize; 5] = [1, 2, 3, 4, 5];

println!(a[4]); // 4

for i in 0..a.len() {
    println!("{}", a[i]);
}

for i in a.iter() {
    println!("{}", i);
}
```

## Slices

Se è necessario creare un **riferimento** ad un array è possibile utilizzare i slices.

```rust
let my_arr = [1, 2, 3, 4, 5];
let a: &[usize] = &[1, 2, 3];
let b: &[usize] = &my_arr[1..3];
let c = b; // è un riferimento

println!(a[0]); // 1
println!(b[0]); // 2
```

> I slices sono un riferimento ad un array.
> 1..3 è un range che va da 1 (incluso) a 3 (escluso).
> I slices possono essere considerati puntatori di C.

Esempio:

```rust
let the_data: [usize; 7] = [1, 2, 3, 4, 5, 6, 7];
fn sun (s: &[usize]) -> usize {
    let mut sum = 0;
    for i in s.iter() {
        sum += i;
    }
    sum
}

for i in 0..the_data.len() -3 {
    println!("{}", sum($the_data[i..i+3]));
}

## Tuple
```
