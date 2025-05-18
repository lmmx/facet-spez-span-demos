```
=== Test 1: CliFormat with Raw span ===
In process_span with F::SpanType = main_b1b1a2071f1ff7c5d5c35920::Raw
SPECIALIZED: Raw to Cooked conversion for format: main_b1b1a2071f1ff7c5d5c35920::CliFormat
Result: Span { start: 10, len: 20, _p: PhantomData<main_b1b1a2071f1ff7c5d5c35920::Cooked> }

=== Test 2: JsonFormat with Cooked span ===
In process_span with F::SpanType = main_b1b1a2071f1ff7c5d5c35920::Cooked
Result: Span { start: 30, len: 40, _p: PhantomData<main_b1b1a2071f1ff7c5d5c35920::Cooked> }
```
