# Tonic

A dynamically-typed (for now...) and procedural programming language.

## About

Tonic is a lightweight language primarily designed for scripting environments. It uses a compile-to-JS approach at runtime, converting all Tonic source-code into readable JavaScript code.

This JavaScript code is then executed on an embedded JavaScript machine.

### Why not write JavaScript?

JavaScript is also a great language for scripting. By writing a language on top of JavaScript, we can take advantage of it's more powerful features such as promises, async/await without having to implement it from scratch. The QuickJS engine is also incredibly lightweight and fast enough for 99% of use-cases.

## Example

```rust
fn fib(n) {
    if n < 2 {
        return n
    }

    return fib(n - 1) + fib(n - 2)
}

println(fib(27)) // -> 196418
```

## Limitations

During testing and development, these are some of the limitations I have found so far:

> A checked box marks the limitation as resolved or fixed.

* [x] A recursive `fib(n)` function that exceeds `fib(27)` causes a runtime stack-overflow. Realistically, nobody is going to be using this much recursion but it's normally a good benchmark for raw recursive performance.