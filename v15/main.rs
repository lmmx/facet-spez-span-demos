use core::marker::PhantomData;
use std::fmt::Debug;

#[derive(Debug)]
pub enum Cooked {}
#[derive(Debug)]
pub enum Raw {}

pub type Pos = usize;

// Format trait with associated input and span types
trait Format {
    type SpanType: Debug + 'static;
    type Input<'a>: ?Sized;
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

// The key trait for conversion, now parameterized by Format and input
trait ToCooked<'input, F: Format> {
    fn to_cooked(self, format: &F, input: &'input F::Input<'input>) -> Span<Cooked>;
}

impl<'input, F: Format> ToCooked<'input, F> for Span<Cooked> {
    #[inline]
    fn to_cooked(self, _format: &F, _input: &'input F::Input<'input>) -> Span<Cooked> { 
        self 
    }
}

impl<'input, F: Format<SpanType = Raw, Input<'input> = [String]>> ToCooked<'input, F> for Span<Raw> {
    #[inline]
    fn to_cooked(self, _format: &F, input: &'input [String]) -> Span<Cooked> {
        println!("SPECIALIZED: Raw to Cooked conversion for CLI format");
        
        // Calculate start position by summing lengths of preceding args plus spaces
        let mut start = 0;
        for i in 0..self.start {
            start += input[i].len() + 1; // +1 for space between args
        }
        
        // Length is the length of the current arg
        let len = input[self.start].len();
        
        Span::<Cooked>::new(start, len)
    }
}

// CLI Format implementation
struct CliFormat;
impl Format for CliFormat {
    type SpanType = Raw;
    type Input<'a> = [String];
}

// JSON Format implementation
struct JsonFormat;
impl Format for JsonFormat {
    type SpanType = Cooked;
    type Input<'a> = [u8];
}

// A generic function that uses the ToCooked trait with input
fn process_span<'input, F: Format>(
    format: &F, 
    span: Span<F::SpanType>, 
    input: &'input F::Input<'input>
) -> Span<Cooked> 
where
    Span<F::SpanType>: ToCooked<'input, F>,
{
    println!("In process_span with F::SpanType = {}", std::any::type_name::<F::SpanType>());
    span.to_cooked(format, input)
}

// Add this function to your code
fn visualize_span<T: AsRef<[u8]>>(input: T, span: &Span<Cooked>) {
    let input_str = String::from_utf8_lossy(input.as_ref());
    println!("Input: {}", input_str);

    // Create underline string with spaces before the span and '^' under the span
    let mut underline = String::with_capacity(input_str.len());
    for i in 0..input_str.len() {
        if i >= span.start && i < span.start + span.len {
            underline.push('^');
        } else {
            underline.push(' ');
        }
    }

    println!("Span:  {}", underline);
}

fn main() {
    // Test with CliFormat (SpanType = Raw)
    let cli_format = CliFormat;
    let cli_input = vec!["--foo".to_string(), "bar".to_string()];
    let raw_span = Span::<Raw>::new(1, 1); // Second arg (index 1), length 1 in raw terms
    
    println!("=== Test 1: CliFormat with Raw span ===");
    println!("Input: {:?}", cli_input);
    println!("Raw span: {:?}", raw_span);
    
    let result1 = process_span(&cli_format, raw_span, &cli_input);
    println!("Cooked span: {:?}", result1);
    let cli_string_owned = cli_input.join(" ");
    let cli_string = cli_string_owned.as_bytes();
    visualize_span(cli_string, &result1);

    println!("");
    
    // Test with JsonFormat (SpanType = Cooked)
    let json_format = JsonFormat;
    let json_input = r#"{"foo": "bar"}"#.as_bytes();
    let cooked_span = Span::<Cooked>::new(2, 3); // "foo" field position in JSON
    
    println!("=== Test 2: JsonFormat with Cooked span ===");
    println!("Input: {}", String::from_utf8_lossy(json_input));
    println!("Cooked span: {:?}", cooked_span);
    
    let result2 = process_span(&json_format, cooked_span, json_input);
    println!("Result (unchanged): {:?}", result2);
    visualize_span(json_input, &result2);
}
