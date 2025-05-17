#!/usr/bin/env rust-script
use core::fmt;
use std::marker::PhantomData;

/// Zero-sized marker for raw spans (format-specific coordinate system)
pub enum Raw {}

/// Zero-sized marker for cooked spans (byte coordinate system for error reporting)
pub enum Cooked {}

/// Position in the input (byte index, if cooked)
pub type Pos = usize;

/// A span in the input, with a start position and length
/// The type parameter C defines the coordinate system (Raw or Cooked).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span<C = Cooked> {
    /// Starting position of the span (in bytes, if cooked)
    pub start: Pos,
    /// Length of the span (in bytes, if cooked)
    pub len: usize,
    /// Phantom data to use the type parameter
    _phantom: PhantomData<C>,
}

/// Trait for types that can be annotated with a Span.
pub trait Spannable<C = Cooked>: Sized {
    /// Annotate this value with a span, wrapping it in Spanned<Self, C>
    fn with_span(self, span: Span<C>) -> Spanned<Self, C>;
}

impl<T, C> Spannable<C> for T {
    fn with_span(self, span: Span<C>) -> Spanned<Self, C> {
        Spanned { node: self, span }
    }
}

impl<C> Span<C> {
    /// Creates a new span with the given start position and length
    pub fn new(start: Pos, len: usize) -> Self {
        Span { start, len, _phantom: PhantomData }
    }

    /// Start position of the span
    pub fn start(&self) -> Pos {
        self.start
    }

    /// Length of the span
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns true if this span has zero length
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// End position (start + length)
    pub fn end(&self) -> Pos {
        self.start + self.len
    }

    /// Convert this span to a different coordinate system
    /// Only for internal use - actual conversion should happen via Format::convert_span
    pub fn as_type<D>(self) -> Span<D> {
        Span {
            start: self.start,
            len: self.len,
            _phantom: PhantomData,
        }
    }
}

/// A value of type T annotated with its Span<C>
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Spanned<T, C = Cooked> {
    /// The actual data/value being wrapped
    pub node: T,
    /// The span information indicating the position and length in the source
    pub span: Span<C>,
}

impl<T, C> Spanned<T, C> {
    /// Map the node value with a function
    pub fn map<U>(self, f: impl FnOnce(T) -> U) -> Spanned<U, C> {
        Spanned::<U, C> {
            node: f(self.node),
            span: self.span,
        }
    }

    /// Map the span with a function
    pub fn map_span<D>(self, f: impl FnOnce(Span<C>) -> Span<D>) -> Spanned<T, D> {
        Spanned::<T, D> {
            node: self.node,
            span: f(self.span),
        }
    }
}

impl<T: fmt::Display, C> fmt::Display for Spanned<T, C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} at {}-{}",
            self.node,
            self.span.start(),
            self.span.end()
        )
    }
}

pub trait Format {
    type SpanType;
    
    fn convert_span(&self, span: Span<Self::SpanType>) -> Span<Cooked>;
}

// Define the format types specifically
pub struct RawFormat<T>(pub T);
pub struct CookedFormat<T>(pub T);

// Implementation for raw formats
impl<T> Format for RawFormat<T> {
    type SpanType = Raw;
    
    fn convert_span(&self, span: Span<Self::SpanType>) -> Span<Cooked> {
        span.as_type::<Cooked>()
    }
}

// Implementation for cooked formats
impl<T> Format for CookedFormat<T> {
    type SpanType = Cooked;
    
    fn convert_span(&self, span: Span<Self::SpanType>) -> Span<Cooked> {
        span // Just return the span directly
    }
}

fn main() {
    println!("Rust-script with Format trait implementation");

    // Example usage with explicit type parameters
    let raw_span = Span::<Raw>::new(10, 5);

    // Using default type parameter (Cooked)
    let cooked_span1 = Span::new(20, 8);  // Type inference uses the default
    let cooked_span2: Span = Span::new(30, 3);  // Explicitly typed but using default
    let cooked_span3: Span<Cooked> = Span::new(40, 6);  // Fully explicit

    // Create formatters
    let raw_formatter = RawFormat(());
    let cooked_formatter = CookedFormat(());

    // Convert spans
    let converted_raw = raw_formatter.convert_span(raw_span);
    let converted_cooked1 = cooked_formatter.convert_span(cooked_span1);
    let converted_cooked2 = cooked_formatter.convert_span(cooked_span2);
    let converted_cooked3 = cooked_formatter.convert_span(cooked_span3);

    println!("Converted raw span: start={}, len={}", converted_raw.start, converted_raw.len);
    println!("Converted cooked span 1 (default inference): start={}, len={}",
             converted_cooked1.start, converted_cooked1.len);
    println!("Converted cooked span 2 (explicit Span type): start={}, len={}",
             converted_cooked2.start, converted_cooked2.len);
    println!("Converted cooked span 3 (explicit Span<Cooked>): start={}, len={}",
             converted_cooked3.start, converted_cooked3.len);

    // All of these produce the same span type
    println!("All spans have the same type: {}",
             std::any::type_name::<Span<Cooked>>() == std::any::type_name::<Span>());
}
