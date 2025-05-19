```
=== Test 1: CliFormat with Raw span ===
Input: ["--foo", "bar"]
Raw span: Span { start: 1, len: 1, _p: PhantomData<main_e499237df34b307109313e6a::Raw> }
In process_span with F::SpanType = main_e499237df34b307109313e6a::Raw
SPECIALIZED: Raw to Cooked conversion for CLI format
Cooked span: Span { start: 6, len: 3, _p: PhantomData<main_e499237df34b307109313e6a::Cooked> }
Input: --foo bar
Span:        ^^^

=== Test 2: JsonFormat with Cooked span ===
Input: {"foo": "bar"}
Cooked span: Span { start: 2, len: 3, _p: PhantomData<main_e499237df34b307109313e6a::Cooked> }
In process_span with F::SpanType = main_e499237df34b307109313e6a::Cooked
Result (unchanged): Span { start: 2, len: 3, _p: PhantomData<main_e499237df34b307109313e6a::Cooked> }
Input: {"foo": "bar"}
Span:    ^^^         
```
