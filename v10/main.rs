use core::marker::PhantomData;

#[derive(Debug)]
pub enum Cooked {}
#[derive(Debug)]
pub enum Raw {}

pub type Pos = usize;

// Format trait with an associated type
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

// The key trait for conversion
trait ToCooked {
    fn to_cooked(self) -> Span<Cooked>;
}

// Implement for Raw - does a conversion
impl ToCooked for Span<Raw> {
    fn to_cooked(self) -> Span<Cooked> {
        println!("SPECIALIZED: Converting from Raw to Cooked span: {:?}", self);
        Span::<Cooked>::new(self.start, self.len)
    }
}

// Implement for Cooked - identity function
impl ToCooked for Span<Cooked> {
    fn to_cooked(self) -> Span<Cooked> {
        println!("SPECIALIZED: Already a Cooked span, returning as is: {:?}", self);
        self // Just return self
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

// A generic function that uses the trait approach
fn process_span<F: Format>(_format: &F, span: Span<F::SpanType>, _input: &str) -> Span<Cooked> 
where
    Span<F::SpanType>: ToCooked, // This ensures span.to_cooked() is available
{
    println!("In process_span with F::SpanType = {}", std::any::type_name::<F::SpanType>());
    let cooked = span.to_cooked();
    cooked
}

fn main() {
    // Test with CliFormat (SpanType = Raw)
    let cli_format = CliFormat;
    let raw_cli_span = Span::<Raw>::new(10, 20);
    println!("=== Test 1: CliFormat with Raw span ===");
    let result1 = process_span(&cli_format, raw_cli_span, "sample cli 1");
    println!("Result with CliFormat (raw): {:?}", result1);

    println!("");

    // Test with JsonFormat (SpanType = Cooked)
    let json_format = JsonFormat;
    let cooked_json_span = Span::<Cooked>::new(50, 60);
    println!("=== Test 2: JsonFormat with Cooked span ===");
    let result2 = process_span(&json_format, cooked_json_span, "sample json 2");
    println!("Result with JsonFormat (cooked): {:?}", result2);
}
