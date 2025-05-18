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
    type SpanType: Debug + 'static;
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

// The key trait for conversion, now parameterized by Format
trait ToCooked<F: Format> {
    fn to_cooked(self, format: &F) -> Span<Cooked>;
}

// Implement for any format where SpanType = Raw
impl<F: Format<SpanType = Raw>> ToCooked<F> for Span<Raw> {
    fn to_cooked(self, _format: &F) -> Span<Cooked> {
        println!("SPECIALIZED: Raw to Cooked conversion for format: {}", std::any::type_name::<F>());
        Span::<Cooked>::new(self.start, self.len)
    }
}

// Implement for any format where SpanType = Cooked
impl<F: Format<SpanType = Cooked>> ToCooked<F> for Span<Cooked> {
    fn to_cooked(self, _format: &F) -> Span<Cooked> {
        println!("SPECIALIZED: Already Cooked for format: {}", std::any::type_name::<F>());
        self
    }
}

// Wrapper struct for the dispatch mechanism
struct __Match<T>(Cell<Option<T>>);

// Base dispatch trait for the specialization
trait __Dispatch<F: Format> {
    fn run(self, format: &F, input: &str) -> Span<Cooked>;
}

// Implementation for spans that implement ToCooked<F> (high priority)
impl<F: Format> __Dispatch<F> for &__Match<Span<F::SpanType>> 
where
    Span<F::SpanType>: ToCooked<F>,
{
    fn run(self, format: &F, _input: &str) -> Span<Cooked> {
        let s = self.0.take().unwrap();
        println!("Using span with ToCooked<F> trait: {:?}", s);
        println!("  Format: {}", std::any::type_name::<F>());
        s.to_cooked(format)
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
    type SpanType = Raw;
}

// Another format with a different SpanType
struct JsonFormat;
impl Format for JsonFormat {
    type SpanType = Cooked;
}

// A generic function that will use the macro with a generic Format type
// Add the trait bound to make it explicit that F::SpanType must implement ToCooked<F>
fn process_span<F: Format>(format: &F, span: Span<F::SpanType>, input: &str) -> Span<Cooked> 
where
    Span<F::SpanType>: ToCooked<F>, // Add this trait bound
{
    println!("In process_span with F::SpanType = {}", std::any::type_name::<F::SpanType>());
    let cooked_span = cook_span_dispatch!(format, span, input);
    cooked_span
}

fn main() {
    // Test with CliFormat (SpanType = Raw)
    let cli_format = CliFormat;
    let raw_span = Span::<Raw>::new(10, 20);
    println!("=== Test 1: CliFormat with Raw span ===");
    let result1 = process_span(&cli_format, raw_span, "sample cli");
    println!("Result: {:?}", result1);

    println!("");
    
    // Test with JsonFormat (SpanType = Cooked)
    let json_format = JsonFormat;
    let cooked_span = Span::<Cooked>::new(30, 40);
    println!("=== Test 2: JsonFormat with Cooked span ===");
    let result2 = process_span(&json_format, cooked_span, "sample json");
    println!("Result: {:?}", result2);
}
