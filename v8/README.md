```
=== Test 1: CliFormat with Raw span ===
In process_span, F::SpanType = main_38f3a6ec413f5aecb4499655::Raw
GENERIC: Converting generic span: Span { start: 10, len: 20, _p: PhantomData<main_38f3a6ec413f5aecb4499655::Raw> }
  Generic span type: main_38f3a6ec413f5aecb4499655::Raw
Result with CliFormat (raw): Span { start: 10, len: 20, _p: PhantomData<main_38f3a6ec413f5aecb4499655::Cooked> }


=== Test 2: JsonFormat with Cooked span ===
In process_span, F::SpanType = main_38f3a6ec413f5aecb4499655::Cooked
GENERIC: Converting generic span: Span { start: 50, len: 60, _p: PhantomData<main_38f3a6ec413f5aecb4499655::Cooked> }
  Generic span type: main_38f3a6ec413f5aecb4499655::Cooked
Result with JsonFormat (cooked): Span { start: 50, len: 60, _p: PhantomData<main_38f3a6ec413f5aecb4499655::Cooked> }
```
