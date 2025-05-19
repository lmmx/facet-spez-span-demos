```
=== Test 1: CliFormat with Raw span ===
Input: ["--foo", "bar"]
Raw span: Span { start: 1, len: 1, _p: PhantomData<main_ef2c51fe7734c45485fcc61e::Raw> }
In process_span with F::SpanType = main_ef2c51fe7734c45485fcc61e::Raw
SPECIALIZED: Raw to Cooked conversion for CLI format
Cooked span: Span { start: 6, len: 3, _p: PhantomData<main_ef2c51fe7734c45485fcc61e::Cooked> }
Input: --foo bar
Span:        ^^^

=== Test 2: JsonFormat with Cooked span ===
Input: {"foo": "bar"}
Cooked span: Span { start: 2, len: 3, _p: PhantomData<main_ef2c51fe7734c45485fcc61e::Cooked> }
In process_span with F::SpanType = main_ef2c51fe7734c45485fcc61e::Cooked
Result (unchanged): Span { start: 2, len: 3, _p: PhantomData<main_ef2c51fe7734c45485fcc61e::Cooked> }
Input: {"foo": "bar"}
Span:    ^^^         
```
