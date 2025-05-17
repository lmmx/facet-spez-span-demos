use core::marker::PhantomData;
use core::cell::Cell;
use std::fmt::Debug;

#[derive(Debug)]
pub enum Cooked {}
#[derive(Debug)]
pub enum Raw {}

pub type Pos = usize;

// Format trait with an associated type
trait Format {
    type SpanType: 'static + std::fmt::Debug;
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

// The key trait for conversion
trait ToCooked {
    fn to_cooked(self) -> Span<Cooked>;
}

// Implement for Raw - does a conversion
impl ToCooked for Span<Raw> {
    fn to_cooked(self) -> Span<Cooked> {
        println!("TOCOOKED TRAIT: Converting from Raw to Cooked span: {:?}", self);
        Span::<Cooked>::new(self.start, self.len)
    }
}

// Wrapper struct for the dispatch mechanism
struct __Match<T>(Cell<Option<T>>);

// Base dispatch trait for the specialization
trait __Dispatch<F: Format> {
    fn run(self, format: &F, input: &str) -> Span<Cooked>;
}

// Specialized implementation for spans that implement ToCooked (medium priority)
impl<F: Format> __Dispatch<F> for &__Match<Span<F::SpanType>> 
where
    F::SpanType: 'static,
    Span<F::SpanType>: ToCooked,
{
    fn run(self, _format: &F, _input: &str) -> Span<Cooked> {
        let s = self.0.take().unwrap();
        println!("MEDIUM PRIORITY: Converting span with ToCooked trait: {:?}", s);
        println!("  Type: {}", std::any::type_name::<F::SpanType>());
        // Use the ToCooked trait implementation
        s.to_cooked()
    }
}

// Generic fallback implementation (lowest priority)
impl<F: Format> __Dispatch<F> for &&__Match<Span<F::SpanType>> 
where
    F::SpanType: 'static,
{
    fn run(self, _format: &F, _input: &str) -> Span<Cooked> {
        let s = self.0.take().unwrap();
        println!("LOW PRIORITY: Generic fallback conversion: {:?}", s);
        println!("  Type: {}", std::any::type_name::<F::SpanType>());
        // Default conversion
        Span::<Cooked>::new(s.start, s.len)
    }
}

// The macro with the correct reference depth
macro_rules! cook_span_dispatch {
    ($format:expr, $span:expr, $input:expr) => {{
        let __tmp = __Match(Cell::new(Some($span)));
        // Try with the lowest reference depth first (highest priority)
        (&__tmp).run($format, $input)
    }};
}

// A concrete format implementation
struct CliFormat;
impl Format for CliFormat {
    type SpanType = Raw; // Implements ToCooked
}

// Another format with a different SpanType
struct JsonFormat;
impl Format for JsonFormat {
    type SpanType = Cooked; // Doesn't implement ToCooked
}

// A generic function that will use the macro with a generic Format type
fn process_span<F: Format>(format: &F, span: Span<F::SpanType>, input: &str) -> Span<Cooked> 
where
    F::SpanType: 'static,
{
    println!("In process_span with F::SpanType = {}", std::any::type_name::<F::SpanType>());
    
    let cooked_span = cook_span_dispatch!(format, span, input);
    cooked_span
}

fn main() {
    // Test with CliFormat (SpanType = Raw, has ToCooked impl)
    let cli_format = CliFormat;
    let raw_cli_span = Span::<Raw>::new(10, 20);
    println!("=== Test 1: CliFormat with Raw span (has ToCooked impl) ===");
    let result1 = process_span(&cli_format, raw_cli_span, "sample cli");
    println!("Result: {:?}", result1);

    println!("");
    
    // Test with JsonFormat (SpanType = Cooked, special case)
    let json_format = JsonFormat;
    let cooked_json_span = Span::<Cooked>::new(30, 40);
    println!("=== Test 2: JsonFormat with Cooked span (special case) ===");
    let result2 = process_span(&json_format, cooked_json_span, "sample json");
    println!("Result: {:?}", result2);
}
