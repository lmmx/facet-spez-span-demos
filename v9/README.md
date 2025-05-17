```
=== Test 1: CliFormat with Raw span ===
In process_span with F::SpanType = main_b6e4487f7be54cb4320f2976::Raw
SPECIALIZED: Converting from Raw to Cooked span: Span { start: 10, len: 20, _p: PhantomData<main_b6e4487f7be54cb4320f2976::Raw> }
Result with CliFormat (raw): Span { start: 10, len: 20, _p: PhantomData<main_b6e4487f7be54cb4320f2976::Cooked> }

=== Test 2: JsonFormat with Cooked span ===
In process_span with F::SpanType = main_b6e4487f7be54cb4320f2976::Cooked
SPECIALIZED: Already a Cooked span, returning as is: Span { start: 50, len: 60, _p: PhantomData<main_b6e4487f7be54cb4320f2976::Cooked> }
Result with JsonFormat (cooked): Span { start: 50, len: 60, _p: PhantomData<main_b6e4487f7be54cb4320f2976::Cooked> }
```
