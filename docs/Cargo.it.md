# Cargo

## Cosa è

Build System e Package Manager per Rust.

Build System: strumento che automatizza il processo di compilazione di un programma.

Package Manager: strumento che automatizza il processo di installazione, aggiornamento, configurazione e rimozione di pacchetti software.

## Cosa fa

- **Creare** un progetto
- **Compilare** il codice
- **Scaricare** le dipendenze
- **Compilare** le **dipendenze**
- Assicurare la **consistenza** delle **versioni** delle dipendenze
- Integrare il progetto su **git** by default

## Installazione

```bash
$ curl https://sh.rustup.rs -sSf | sh
```

## Creare un progetto

Per creare un progetto Rust si usa il comando `cargo new`:

```bash
$ cargo new hello_world
```

Questo comando crea una directory `hello_world` con i seguenti file:

```bash
$ tree hello_world
hello_world
├── Cargo.toml
└── src
    └── main.rs
```

Il file `Cargo.toml` contiene le informazioni sul progetto e le sue dipendenze:

Il comando `cargo build` autogenererà il file `Cargo.lock` che contiene le informazioni sulle versioni delle dipendenze.

```toml
[package]
name = "hello_world"
version = "0.1.0"

[package]
name = "hello_world"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
rand = "0.8.5"
```

## Comandi principali

### cargo build

Compila il progetto e le sue dipendenze così da generare un file eseguire. Il file sarà posizionato in `target/debug/hello_world`.

```bash
$ cargo build
```

Si può eseguire il codice tramite il doppio click sul file eseguibile.

### cargo build --release

Compila il progetto e le sue dipendenze così da generare un file eseguire per la distribuzione. Il file sarà posizionato in `target/release/hello_world`.

```bash
$ cargo build --release
```

Con questo comando il codice viene ottimizzato e quindi la compilazione è più lenta (Per questo motivo è consigliato usare `cargo build` e `cargo run` durante lo sviluppo).

### cargo run (build + esegui)

Compila il progetto e le sue dipendenze e poi esegue il file eseguibile.

```bash
$ cargo run
```

### cargo check

Controlla che il codice sia compilabile senza generare il file eseguibile.

```bash
$ cargo check
```

## Clippy

Clippy è un un'insieme di più di 600 linter per Rust. Un linter è uno strumento che analizza il codice e segnala eventuali errori, bug, problemi di stile, ecc.

In generale serve a scrivere codice più pulito e corretto.

```bash
$ cargo clippy
```

E' possibile applicare dei suggerimenti automaticamente con il comando:

```bash
$ cargo clippy --fix
```
