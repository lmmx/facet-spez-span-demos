# Span Conversion Demos

This repository demonstrates different approaches to implementing and converting between
coordinate systems (represented by two generic types `Span::<Raw>` and `Span::<Cooked>`)
in Rust, with increasing levels of sophistication.

Initial example uses the `spez!` macro (from the spez crate), and then switches to `facet-core::spez::Spez`.

## Overview

The demos show how to handle span conversions using different techniques:

- **v1**: Basic trait-based implementation with phantom types
- **v3**: Using the `spez!` macro for type-based dispatch
- **v4**: Using `facet-core::spez::Spez` for trait-based dispatch
- **v5**: Extended version of v4 that returns converted spans

Their outputs are in the subfolders as READMEs

## v1: Basic Trait Implementation

The first version demonstrates a fundamental approach with:

- Zero-sized marker types (`Raw` and `Cooked` enums)
- A generic `Span<C>` struct with PhantomData for type parameters
- Format traits defining the conversion between coordinate systems
- Implementation for both Raw and Cooked formats

This approach uses classical Rust traits but requires explicit handling of format types.

## v2

(Skip)

## v3: spez! Macro Dispatch

Version 3 introduces the `spez!` macro for type-based dispatch:

- Uses the same marker types and `Span<C>` structure
- The `show_span!` macro uses `spez!` to dispatch based on span type
- Each macro expansion is monomorphic, selecting the appropriate match arm

This approach simplifies usage by automatically selecting the right implementation
based on the compile-time type.

## v4: facet-core::spez::Spez

Version 4 implements the same functionality using `facet-core::spez::Spez`:

- Uses a trait-based approach with `ShowSpan` trait
- Implements the trait for `&Spez<Span<Cooked>>` and `&Spez<Span<Raw>>`
- The `show_span!` macro uses Tolnay's trick with reference depth disambiguation
- Demonstrates how different reference depths make the impls mutually exclusive

This implementation shows how to handle dispatch without the macro by leveraging
Rust's trait system and reference depth for disambiguation.

## v5: Return Values

The final version extends v4 to return the converted spans:

- The `ShowSpan` trait now returns `Span<Cooked>`
- For Raw spans, it performs the conversion before returning
- The implementations add Clone and Copy implementations for Span
- Demonstrates how the returned values are all Cooked spans

This version shows how to use the dispatch mechanism not just for side effects but
to transform values based on their type.

## v6

This is a repro of the error you get when you pass an associated type.

## Running the Examples

Each example can be run with `rust-script`:

```bash
cd v1
rust-script main.rs

cd ../v3
rust-script main.rs

# etc.
```

## v7

Shows that you can capture the 'generic' case of an associated type (this may not be quite right
terminology: the value in an associated type is concrete, it's just stored on the associated type?)

## v8

Shows that you cannot actually monomorphise on an associated type... Back to the drawing board.

## v9

Taking into account that the whole point here is to specialise on **traits** and their bounds rather
than **types**, so the way to specialise on an associated type is to differentiate those associated
types based on their implementations. In this example though, there is no monomorphisation, the
trait is simply implemented for all format span types.

A better design would be to only implement ToCooked for those that needed it, and not require it for
those whose SpanType is already Cooked. This design simply always calls the trait, so the match
seems a bit redundant (besides being the machinery to call the trait implementation).

Nevertheless, it works!

## Key Concepts

- **PhantomData**: Used to carry type information without runtime cost
- **Trait-based dispatch**: Using traits to select different implementations based on types
- **Type-based dispatch**: Using macros and type inference to select code paths
- **David Tolnay's trick**: Using reference depth to disambiguate trait implementations
- **Monomorphization**: Each expansion of the macros creates code specific to that type
