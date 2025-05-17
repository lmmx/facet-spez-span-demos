```
error[E0599]: no method named `run` found for reference `&__Match<Span<<F as Format>::SpanType>>` in the current scope
  --> /home/louis/lab/rust/traits/span/v6/main.rs:69:18
   |
69 |         (&__tmp).run($format, $input)
   |                  ^^^ method not found in `&__Match<Span<<F as Format>::SpanType>>`
...
83 |     let cooked_span = cook_span_dispatch!(format, span, input);
   |                       ---------------------------------------- in this macro invocation
   |
   = help: items from traits can only be used if the trait is implemented and in scope
note: `__Dispatch` defines an item `run`, perhaps you need to implement it
  --> /home/louis/lab/rust/traits/span/v6/main.rs:42:1
   |
42 | trait __Dispatch {
   | ^^^^^^^^^^^^^^^^
   = note: this error originates in the macro `cook_span_dispatch` (in Nightly builds, run with -Z macro-backtrace for more info)

For more information about this error, try `rustc --explain E0599`.
error: could not compile `main_cd89d5e3ae1612b3eeb1ec99` (bin "main_cd89d5e3ae1612b3eeb1ec99") due to 1 previous error
error: Could not execute cargo
```
