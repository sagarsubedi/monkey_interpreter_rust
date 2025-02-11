# Monkey Programming Language Interpreter in Rust

This is an implementation of the Monkey programming language interpreter in Rust, based on the book ["Writing An Interpreter In Go"](https://interpreterbook.com/) by Thorsten Ball.

## About Monkey Language

Monkey is a programming language that supports:

- Variable bindings
- Integers and booleans
- Arithmetic expressions
- Built-in functions
- First-class and higher-order functions
- Closures
- A string data structure
- An array data structure
- A hash data structure

## Project Structure

The interpreter is being built in Rust, following the same concepts as outlined in the book but adapted to take advantage of Rust's features and idioms.

## Building and Running

```bash
cargo build
cargo run
```

## Implementation Progress

- [ ] Lexer
- [ ] Parser
- [ ] Abstract Syntax Tree (AST)
- [ ] Internal Object System
- [ ] Evaluator
- [ ] Built-in Functions
- [ ] Error Handling
