#!/usr/bin/env rust-script
//! ```cargo
//! [dependencies]
//! spez = "0.1.2"
//! ```

use spez::spez;
use std::fmt::Debug;

// Define our Raw and Cooked types
#[derive(Debug, Clone)]
struct Raw(String);

#[derive(Debug, Clone)]
struct Cooked(String);

// Span generic container
#[derive(Debug, Clone)]
struct Span<T> {
    content: T,
    start: usize,
    end: usize,
}

// Implementation for Raw spans to convert to Cooked
impl Span<Raw> {
    fn as_type(self) -> Span<Cooked> {
        // Simulated conversion from Raw to Cooked
        Span {
            content: Cooked(self.content.0 + " (cooked)"),
            start: self.start,
            end: self.end,
        }
    }
}

// Helper function that uses spez for dispatch
fn convert_span<T>(span: Span<T>) -> Span<Cooked> {
    spez! {
        for span;
        match Span<Raw> -> Span<Cooked> {
            println!("Converting from Raw");
            span.as_type()
        }
        match Span<Cooked> -> Span<Cooked> {
            println!("Already Cooked, returning as is");
            span
        }
    }
}

// Format trait definition
pub trait Format {
    type SpanType;
    
    fn convert_span(&self, span: Span<Self::SpanType>) -> Span<Cooked>;
}

// RawFormat implements Format with SpanType = Raw
struct RawFormat;
impl Format for RawFormat {
    type SpanType = Raw;
    
    fn convert_span(&self, span: Span<Self::SpanType>) -> Span<Cooked> {
        convert_span(span)
    }
}

// CookedFormat implements Format with SpanType = Cooked
struct CookedFormat;
impl Format for CookedFormat {
    type SpanType = Cooked;
    
    fn convert_span(&self, span: Span<Self::SpanType>) -> Span<Cooked> {
        convert_span(span)
    }
}

fn main() {
    // Create a raw span
    let raw_span = Span {
        content: Raw("raw content".to_string()),
        start: 0,
        end: 11,
    };
    
    // Create a cooked span
    let cooked_span = Span {
        content: Cooked("cooked content".to_string()),
        start: 0,
        end: 14,
    };
    
    // Use the RawFormat implementation
    let raw_format = RawFormat;
    let cooked_result1 = raw_format.convert_span(raw_span);
    println!("Result from raw: {:?}", cooked_result1);
    
    // Use the CookedFormat implementation
    let cooked_format = CookedFormat;
    let cooked_result2 = cooked_format.convert_span(cooked_span);
    println!("Result from cooked: {:?}", cooked_result2);
}
