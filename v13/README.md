```
=== Test 1: CliFormat with Raw span ===
In process_span with F::SpanType = main_a3c2c129cff80a931a4a4d6b::Raw
SPECIALIZED: Raw to Cooked conversion for format: main_a3c2c129cff80a931a4a4d6b::CliFormat
Result: Span { start: 10, len: 20, _p: PhantomData<main_a3c2c129cff80a931a4a4d6b::Cooked> }

=== Test 2: JsonFormat with Cooked span ===
In process_span with F::SpanType = main_a3c2c129cff80a931a4a4d6b::Cooked
SPECIALIZED: Already Cooked for format: main_a3c2c129cff80a931a4a4d6b::JsonFormat
Result: Span { start: 30, len: 40, _p: PhantomData<main_a3c2c129cff80a931a4a4d6b::Cooked> }
```
