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
    type SpanType: Debug;
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

// Wrapper struct for the dispatch mechanism
struct __Match<T>(Cell<Option<T>>);

// Base dispatch trait for the specialization
trait __Dispatch<F: Format> {
    fn run(self, format: &F, input: &str) -> Span<Cooked>;
}

// This is the specialized implementation for Cooked
// Note the additional reference (&) which makes this more specific
impl<F: Format> __Dispatch<F> for &&__Match<Span<Cooked>> {
    fn run(self, _format: &F, _input: &str) -> Span<Cooked> {
        let s = self.0.take().unwrap();
        println!("SPECIALIZED: Using already cooked span: {:?}", s);
        s
    }
}

// This is the specialized implementation for Raw
// Again, note the additional reference
impl<F: Format> __Dispatch<F> for &&__Match<Span<Raw>> {
    fn run(self, _format: &F, _input: &str) -> Span<Cooked> {
        let s = self.0.take().unwrap();
        println!("SPECIALIZED: Converting raw span: {:?}", s);
        Span::<Cooked>::new(s.start, s.len)
    }
}

// This is the fallback generic implementation with one less &
impl<F: Format> __Dispatch<F> for &__Match<Span<F::SpanType>> {
    fn run(self, _format: &F, _input: &str) -> Span<Cooked> {
        let s = self.0.take().unwrap();
        println!("GENERIC: Converting generic span: {:?}", s);
        Span::<Cooked>::new(s.start, s.len)
    }
}

// The macro now uses double borrowing for specialization
macro_rules! cook_span_dispatch {
    ($format:expr, $span:expr, $input:expr) => {{
        let __tmp = __Match(Cell::new(Some($span)));
        // Two levels of borrowing for specialization to kick in
        (&&__tmp).run($format, $input)
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
fn process_span<F: Format>(format: &F, span: Span<F::SpanType>, input: &str) -> Span<Cooked> {
    let cooked_span = cook_span_dispatch!(format, span, input);
    cooked_span
}

fn main() {
    // Test with CliFormat (SpanType = Raw)
    let cli_format = CliFormat;
    let raw_span = Span::<Raw>::new(10, 20);
    let result1 = process_span(&cli_format, raw_span, "sample cli");
    println!("Result with CliFormat: {:?}", result1);
    
    // Test with JsonFormat (SpanType = Cooked)
    let json_format = JsonFormat;
    let cooked_span = Span::<Cooked>::new(30, 40);
    let result2 = process_span(&json_format, cooked_span, "sample json");
    println!("Result with JsonFormat: {:?}", result2);

    // Test direct specialization
    let cooked_direct = Span::<Cooked>::new(50, 60);
    let cooked_result = cook_span_dispatch!(&cli_format, cooked_direct, "direct cooked");
    println!("Direct cooked result: {:?}", cooked_result);
    
    let raw_direct = Span::<Raw>::new(70, 80);
    let raw_result = cook_span_dispatch!(&cli_format, raw_direct, "direct raw");
    println!("Direct raw result: {:?}", raw_result);
}
