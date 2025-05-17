use core::marker::PhantomData;
use core::cell::Cell;

#[derive(Debug)]
pub enum Cooked {}
#[derive(Debug)]
pub enum Raw {}

pub type Pos = usize;

// Add the Format trait with an associated type
trait Format {
    type SpanType;
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

// The dispatch trait that expects specific implementations
trait __Dispatch {
    fn run<F: Format>(self, format: &F, input: &str) -> Span<Cooked>;
}

// Only implement for specific concrete types, not for generic types
impl __Dispatch for &__Match<Span<Cooked>> {
    fn run<F: Format>(self, _format: &F, _input: &str) -> Span<Cooked> {
        let s = self.0.take().unwrap();
        println!("✓ cooked span: {:?}", s);
        s
    }
}

impl __Dispatch for &__Match<Span<Raw>> {
    fn run<F: Format>(self, _format: &F, _input: &str) -> Span<Cooked> {
        let s = self.0.take().unwrap();
        println!("✓ raw span: {:?}", s);
        Span::<Cooked>::new(s.start, s.len)
    }
}

struct __Match<T>(Cell<Option<T>>);

// The macro that will cause the error
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

// A generic function that will use the macro with a generic Format type
fn process_span<F: Format>(format: &F, span: Span<F::SpanType>, input: &str) -> Span<Cooked> {
    // This will cause the error because there's no implementation of
    // run() for &__Match<Span<F::SpanType>>
    let cooked_span = cook_span_dispatch!(format, span, input);
    cooked_span
}

fn main() {
    let format = JsonFormat;
    let span = Span::<Raw>::new(10, 20);
    
    // This will trigger the error
    let result = process_span(&format, span, "sample input");
    println!("Result: {:?}", result);
}