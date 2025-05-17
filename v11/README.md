```
=== Test 1: CliFormat with Raw span ===
In process_span with F::SpanType = main_7be617d7977e58ca59556b23::Raw
HIGH PRIORITY: Using span with ToCooked<F> trait: Span { start: 10, len: 20, _p: PhantomData<main_7be617d7977e58ca59556b23::Raw> }
  Format: main_7be617d7977e58ca59556b23::CliFormat
SPECIALIZED: Raw to Cooked conversion for format: main_7be617d7977e58ca59556b23::CliFormat
Result: Span { start: 10, len: 20, _p: PhantomData<main_7be617d7977e58ca59556b23::Cooked> }

=== Test 2: JsonFormat with Cooked span ===
In process_span with F::SpanType = main_7be617d7977e58ca59556b23::Cooked
HIGH PRIORITY: Using span with ToCooked<F> trait: Span { start: 30, len: 40, _p: PhantomData<main_7be617d7977e58ca59556b23::Cooked> }
  Format: main_7be617d7977e58ca59556b23::JsonFormat
SPECIALIZED: Already Cooked for format: main_7be617d7977e58ca59556b23::JsonFormat
Result: Span { start: 30, len: 40, _p: PhantomData<main_7be617d7977e58ca59556b23::Cooked> }
```
