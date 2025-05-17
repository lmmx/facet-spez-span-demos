```
error[E0271]: type mismatch resolving `<F as Format>::SpanType == Raw`
   --> /home/louis/lab/rust/traits/span/v10/main.rs:120:43
    |
97  |         (&__tmp).run($format, $input)
    |                  --- required by a bound introduced by this call
...
120 |     let cooked_span = cook_span_dispatch!(format, span, input);
    |                                           ^^^^^^ expected `Raw`, found associated type
    |
    = note:         expected enum `Raw`
            found associated type `<F as Format>::SpanType`
note: required for `&__Match<Span<<F as Format>::SpanType>>` to implement `__Dispatch<F>`
   --> /home/louis/lab/rust/traits/span/v10/main.rs:64:17
    |
64  | impl<F: Format> __Dispatch<F> for &__Match<Span<F::SpanType>> 
    |                 ^^^^^^^^^^^^^     ^^^^^^^^^^^^^^^^^^^^^^^^^^^
...
67  |     Span<F::SpanType>: ToCooked,
    |                        -------- unsatisfied trait bound introduced here
    = note: associated types for the current `impl` cannot be restricted in `where` clauses
help: consider constraining the associated type `<F as Format>::SpanType` to `Raw`
    |
114 | fn process_span<F: Format<SpanType = Raw>>(format: &F, span: Span<F::SpanType>, input: &str) -> Span<Cooked> 
    |                          ++++++++++++++++

For more information about this error, try `rustc --explain E0271`.
error: could not compile `main_29e67a5a609a8327ce28bb44` (bin "main_29e67a5a609a8327ce28bb44") due to 1 previous error
error: Could not execute cargo
```
