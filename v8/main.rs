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

// The dispatch trait now takes a generic parameter F
trait __Dispatch<F: Format> {
    fn run(self, format: &F, input: &str) -> Span<Cooked>;
}

// Implement for the generic case
impl<F: Format> __Dispatch<F> for &__Match<Span<F::SpanType>> {
    fn run(self, _format: &F, _input: &str) -> Span<Cooked> {
        let s = self.0.take().unwrap();
        println!("Converting generic span from format's associated type: {:?}", s);
        // Convert generic span to Cooked
        Span::<Cooked>::new(s.start, s.len)
    }
}

// The macro now correctly handles any type that matches the pattern
macro_rules! cook_span_dispatch {
    ($format:expr, $span:expr, $input:expr) => {{
        let __tmp = __Match(Cell::new(Some($span)));
        (&__tmp).run($format, $input)
    }};
}

// A concrete format implementation
struct JsonFormat;
impl Format for JsonFormat {
    type SpanType = Raw;
}

// Another format with a different SpanType
struct XmlFormat;
impl Format for XmlFormat {
    type SpanType = Cooked;
}

// A generic function that will use the macro with a generic Format type
fn process_span<F: Format>(format: &F, span: Span<F::SpanType>, input: &str) -> Span<Cooked> {
    // This now works without error
    let cooked_span = cook_span_dispatch!(format, span, input);
    cooked_span
}

fn main() {
    // Test with JsonFormat (SpanType = Raw)
    let json_format = JsonFormat;
    let raw_span = Span::<Raw>::new(10, 20);
    let result1 = process_span(&json_format, raw_span, "sample json");
    println!("Result with JsonFormat: {:?}", result1);
    
    // Test with XmlFormat (SpanType = Cooked)
    let xml_format = XmlFormat;
    let cooked_span = Span::<Cooked>::new(30, 40);
    let result2 = process_span(&xml_format, cooked_span, "sample xml");
    println!("Result with XmlFormat: {:?}", result2);

    // Test direct conversions
    let arbitrary_span = Span::<Raw>::new(50, 60);
    let cooked_arbitrary = cook_span_dispatch!(&json_format, arbitrary_span, "direct conversion");
    println!("Direct conversion result: {:?}", cooked_arbitrary);
}
