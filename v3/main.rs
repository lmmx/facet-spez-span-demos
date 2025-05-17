//! ```cargo
//! [dependencies]
//! spez = "0.1.2"
//! ```

use spez::spez;
use std::marker::PhantomData;

#[derive(Debug)] pub enum Cooked {}
#[derive(Debug)] pub enum Raw    {}

pub type Pos = usize;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span<C = Cooked> {
    pub start: Pos,
    pub len:   usize,
    _p:        PhantomData<C>,
}
impl<C> Span<C> {
    pub fn new(start: Pos, len: usize) -> Self {
        Self { start, len, _p: PhantomData }
    }
}

/* ---- macro: every expansion is monomorphic ---- */
macro_rules! show_span {
    ($span:expr) => {
        spez! {
            for s = $span;
            match Span<Cooked> {
                println!("✓ cooked span: {:?}", s);
            }
            match Span<Raw> {
                println!("✓  raw   span: {:?}", s);
            }
        }
    };
}

fn main() {
    let normal = Span::new(12, 34);
    let cooked = Span::<Cooked>::new(12, 34);
    let raw    = Span::<Raw>::new(56, 78);

    show_span!(normal);  // expands with Span<C = Cooked> → first arm
    show_span!(cooked);  // expands with Span<Cooked> → first arm
    show_span!(raw);     // expands with Span<Raw>    → second arm
}
