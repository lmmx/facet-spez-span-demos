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

## v9 :tada:

- Note: the specialisation approach was dropped here. This is purely using traits.

Taking into account that the whole point here is to specialise on **traits** and their bounds rather
than **types**, so the way to specialise on an associated type is to differentiate those associated
types based on their implementations. In this example though, there is no monomorphisation, the
trait is simply implemented for all format span types.

A better design would be to only implement ToCooked for those that needed it, and not require it for
those whose SpanType is already Cooked. This design simply always calls the trait, so the match
seems a bit redundant (besides being the machinery to call the trait implementation).

Nevertheless, it works!

## v10

Now it's time to bring back the specialisation macro approach, and only implement the `ToCooked` trait for
the formats whose SpanType is not already Cooked.

- Does not work: cannot specialise on the associated type

## v11 :tada:

Now we implement a ToCooked trait **parameterised by a Format** for the cases of SpanType = Raw and
SpanType = Cooked. This is sort of like a registry pattern for formats.

It **allows us the ideal design of not implementing ToCooked per format** but instead implementing
it on `SpanType = Cooked` (just returning `self` but satisfying the type checker) :tada:

**Summary**

11th! 🎉 Associated-type matching is served

Given a Format trait, on each format you associate a type {Raw|Cooked}

```rust
impl<F: Format<SpanType = Cooked>> ToCooked<F> for Span<Cooked> {
  fn to_cooked() -> Span<Cooked> { self }
}
```

- Parameterising by F is the trick to make it work
- Avoids requiring all formats to define a `to_cooked` method

## v12

The obvious next question: can you specify a different `to_cooked` or will you need to share the
same method across all sharing a given SpanType in this scenario (can different formats specify
their own `ToCooked` methods)?

If so, can they do it via overriding a default method, or would they need to have a helper method
shared across all formats of that SpanType (e.g. `format.convert_span()`)? It would be ideal if you
didn't *have* to specify it, but could override a default one.

This really illustrates the design choices, or the bind (and why v11 is a better one):

- You can do it like this **but** the cooked formats will need to specify a redundant `cook_span`
  method
- Better to do it like v11 and instead of having different formats using their own conversion
  functions to change the span index coordinate system, have different **span types** have an
  associated conversion function defined in the trait impl for `ToCooked` for that `F<SpanType = ...>`
  - It's the better choice essentially if you can enumerate a finite number of ways you'll want to
    transform it (and that's the whole point of associating the SpanType to the Format in this case)
  - It would not be the better choice if you **didn't** have a no-op case that you expected to be
    common (in our case, many spans will be cooked because they will be operating on bytes, e.g.
    JSON's tokeniser will maintain a byte index as it steps through the JSON byte stream. For CLI
    args and other `&[&str]` formats though, they will form categories where different situations
    might share the same processing.
  - "Raw" vs. "Cooked" is not a very informative name: "Words" vs. "Chars" might be a better one).

## v13

Looking at v11 again, I lastly was wondering could it be simplified: do we even need the matching?
It looks like we had replaced the matching with a single arm, so maybe we didn't need specialisation
in the same way (it'd been solved without it). **Answer: no**

Same functionality as the v11 with 1/3 fewer lines (150 -> 100 LOC).

## v14

The same as v13 but can importantly this version be called even if already cooked.

We remove the impl that matches on SpanType = Cooked

```rust
// // Implement for any format where SpanType = Cooked
// impl<F: Format<SpanType = Cooked>> ToCooked<F> for Span<Cooked> {
//     fn to_cooked(self, _format: &F) -> Span<Cooked> {
//         println!("SPECIALIZED: Already Cooked for format: {}", std::any::type_name::<F>());
//         self
//     }
```

We change to

```rust
// Implement for any format where SpanType = Raw
impl<F: Format> ToCooked<F> for Span<Cooked> {
    #[inline]
    fn to_cooked(self, _format: &F) -> Span<Cooked> { self }
}

// Unchanged
impl<F: Format<SpanType = Raw>> ToCooked<F> for Span<Raw> {
    #[inline]
    fn to_cooked(self, _format: &F) -> Span<Cooked> {
        println!("SPECIALIZED: Raw to Cooked conversion for format: {}", std::any::type_name::<F>());
        Span::<Cooked>::new(self.start, self.len)
    }
}
```

## v15

This time we introduce the real motivation and supply input to the function too, allowing us to
print out spans against the original input (turning raw spans into "cooked" ones for diagnostics).

## v16

As for v15, this one has input, but uses `&str` rather than `String` for the `Input` type on the CLI
format, meaning it gets some extra lifetimes involved.

<pre class="terminal">
<span class='shell'>&gt; </span><span class='cmd'>difft</span> <span class='flag'>--color</span><span class='arg'>=always</span> <span class='flag'>--display</span><span class='arg'>=inline</span> <span class='arg'>v15/main.rs</span> <span class='arg'>v16/main.rs</span>
<b><span style='color:var(--bright-yellow,#ff5)'>main.rs</span></b><span style='opacity:0.67'> --- 1/4 --- Rust</span>
<span style='color:var(--bright-red,#f55)'><b>54 </b></span>   <b>impl</b>&lt;<b>&#39;input</b>, <b>F</b>: <b>Format</b>&lt;<b>SpanType</b> = <b>Raw</b>, <b>Input</b>&lt;<b>&#39;input</b>&gt; = [<span style='color:var(--bright-red,#f55)'><b>String</b></span>]&gt;&gt; <b>ToCooked</b>&lt;<b>&#39;input</b>, <b>F</b>&gt; <b>for</b> <b>Span</b>&lt;<b>Raw</b>&gt; {
<span style='color:var(--bright-red,#f55)'><b>56 </b></span>       <b>fn</b> to_cooked(self, _format: <b>&amp;F</b>, input: <b>&amp;&#39;input</b> [<span style='color:var(--bright-red,#f55)'><b>String</b></span>]) -&gt; <b>Span</b>&lt;<b>Cooked</b>&gt; {
   <span style='color:var(--bright-green,#5f5)'><b>54 </b></span><b>impl</b>&lt;<b>&#39;input</b>, <b>F</b>: <b>Format</b>&lt;<b>SpanType</b> = <b>Raw</b>, <b>Input</b>&lt;<b>&#39;input</b>&gt; = [<span style='color:var(--bright-green,#5f5)'><b>&amp;&#39;input</b></span> <span style='color:var(--bright-green,#5f5)'><b>str</b></span>]&gt;&gt; <b>ToCooked</b>&lt;<b>&#39;input</b>, <b>F</b>&gt; <b>for</b> <b>Span</b>&lt;<b>Raw</b>&gt; {
   <span style='color:var(--bright-green,#5f5)'><b>56 </b></span>    <b>fn</b> to_cooked(self, _format: <b>&amp;F</b>, input: <b>&amp;&#39;input</b> [<span style='color:var(--bright-green,#5f5)'><b>&amp;&#39;input</b></span> <span style='color:var(--bright-green,#5f5)'><b>str</b></span>]) -&gt; <b>Span</b>&lt;<b>Cooked</b>&gt; {
   <span style='opacity:0.67'>57 </span>        println!(<span style='color:var(--bright-magenta,#f5f)'>&quot;SPECIALIZED: Raw to Cooked conversion for CLI format&quot;</span>);

<b>main.rs</b><span style='opacity:0.67'> --- 2/4 --- Rust</span>
<span style='opacity:0.67'>72 </span>   <span style='color:var(--bright-blue,#55f)'><i>// CLI Format implementation</i></span>
<span style='opacity:0.67'>73 </span>   <b>struct</b> <b>CliFormat</b>;
<span style='opacity:0.67'>74 </span>   <b>impl</b> <b>Format</b> <b>for</b> <b>CliFormat</b> {
<span style='opacity:0.67'>75 </span>       <b>type</b> <b>SpanType</b> = <b>Raw</b>;
<span style='color:var(--bright-red,#f55)'><b>76 </b></span>       <b>type</b> <b>Input</b>&lt;<b>&#39;a</b>&gt; = [<span style='color:var(--bright-red,#f55)'><b>String</b></span>];
   <span style='color:var(--bright-green,#5f5)'><b>76 </b></span>    <b>type</b> <b>Input</b>&lt;<b>&#39;a</b>&gt; = [<span style='color:var(--bright-green,#5f5)'><b>&amp;&#39;a</b></span> <span style='color:var(--bright-green,#5f5)'><b>str</b></span>];
   <span style='opacity:0.67'>77 </span>}
...
<span class='shell'>&gt; </span><span class='caret'> </span>
</pre>

The final result looks like this:

```rust
use core::marker::PhantomData;
use core::fmt::Debug;

#[derive(Debug)]
pub enum Cooked {}
#[derive(Debug)]
pub enum Raw {}

pub type Pos = usize;

// Format trait with associated input and span types
trait Format {
    type SpanType: Debug + 'static;
    type Input<'input>: ?Sized;
}

#[derive(Debug, PartialEq, Eq)]
pub struct Span<C = Cooked> {
    pub start: Pos,
    pub len: usize,
    _p: PhantomData<C>,
}

impl<C> Span<C> {
    pub fn new(start: Pos, len: usize) -> Self {
        Self {
            start,
            len,
            _p: PhantomData,
        }
    }
}

impl<C> Clone for Span<C> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<C> Copy for Span<C> {}

// The key trait for conversion, now parameterized by Format and input
trait ToCooked<'input, F: Format> {
    fn to_cooked(self, format: &F, input: &'input F::Input<'input>) -> Span<Cooked>;
}

impl<'input, F: Format> ToCooked<'input, F> for Span<Cooked> {
    #[inline]
    fn to_cooked(self, _format: &F, _input: &'input F::Input<'input>) -> Span<Cooked> {
        self
    }
}

impl<'input, F: Format<SpanType = Raw, Input<'input> = [&'input str]>> ToCooked<'input, F> for Span<Raw> {
    #[inline]
    fn to_cooked(self, _format: &F, input: &'input [&'input str]) -> Span<Cooked> {
        println!("SPECIALIZED: Raw to Cooked conversion for CLI format");

        // Calculate start position by summing lengths of preceding args plus spaces
        let mut start = 0;
        for arg in input.iter().take(self.start) {
            start += arg.len() + 1; // +1 for space between args
        }

        // Length is the length of the current arg
        let len = input[self.start].len();

        Span::<Cooked>::new(start, len)
    }
}

// CLI Format implementation
struct CliFormat;
impl Format for CliFormat {
    type SpanType = Raw;
    type Input<'input> = [&'input str];
}

// JSON Format implementation
struct JsonFormat;
impl Format for JsonFormat {
    type SpanType = Cooked;
    type Input<'input> = [u8];
}

// A generic function that uses the ToCooked trait with input
fn process_span<'input, F: Format>(
    format: &F,
    span: Span<F::SpanType>,
    input: &'input F::Input<'input>
) -> Span<Cooked>
where
    Span<F::SpanType>: ToCooked<'input, F>,
{
    span.to_cooked(format, input)
}
```

## Key Concepts

- **PhantomData**: Used to carry type information without runtime cost
- **Trait-based dispatch**: Using traits to select different implementations based on types
- **Type-based dispatch**: Using macros and type inference to select code paths
- **David Tolnay's trick**: Using reference depth to disambiguate trait implementations
- **Monomorphization**: Each expansion of the macros creates code specific to that type
