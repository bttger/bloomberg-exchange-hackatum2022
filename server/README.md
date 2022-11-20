# Server

## Build

Requieres Rust 1.65 and some SSL library to be available in the system (see rust
native tls).

```cli
$ cargo build
```

## Additiona components

A PostgreSQL database should be available at
`postgresql://hackatum2022@localhost/hackatum2022` using the [schema](../schema.sql)

## Run

```cli
$ cargo run
```

The server will be up on `ws://127.0.0.1:3000/`

## Examples

Using `websocat` you can try some [example queries](../examples/)
