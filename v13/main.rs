use core::marker::PhantomData;
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

// A generic function that directly uses the ToCooked trait
fn process_span<F: Format>(format: &F, span: Span<F::SpanType>) -> Span<Cooked> 
where
    Span<F::SpanType>: ToCooked<F>,
{
    println!("In process_span with F::SpanType = {}", std::any::type_name::<F::SpanType>());
    span.to_cooked(format)
}

fn main() {
    // Test with CliFormat (SpanType = Raw)
    let cli_format = CliFormat;
    let raw_span = Span::<Raw>::new(10, 20);
    println!("=== Test 1: CliFormat with Raw span ===");
    let result1 = process_span(&cli_format, raw_span);
    println!("Result: {:?}", result1);

    println!("");
    
    // Test with JsonFormat (SpanType = Cooked)
    let json_format = JsonFormat;
    let cooked_span = Span::<Cooked>::new(30, 40);
    println!("=== Test 2: JsonFormat with Cooked span ===");
    let result2 = process_span(&json_format, cooked_span);
    println!("Result: {:?}", result2);
}
