//! ```cargo
//! [dependencies]
//! facet-core = "0.27.1"
//! ```

use facet_core::spez::Spez;
use core::marker::PhantomData;

#[derive(Debug)] pub enum Cooked {}
#[derive(Debug)] pub enum Raw    {}

pub type Pos = usize;

#[derive(Debug, PartialEq, Eq)]
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

// Copy + Clone not auto-derived for PhantomData
// https://stackoverflow.com/a/31371094/2668831

impl<C> Clone for Span<C> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<C> Copy for Span<C> {}

#[allow(dead_code)]
trait ShowSpan { fn run(self) -> Span<Cooked>; }

impl ShowSpan for &Spez<Span<Cooked>> {
    fn run(self) -> Span<Cooked> {
        println!("✓ cooked span: {:?}", self.0);
        self.0
    }
}
impl ShowSpan for &Spez<Span<Raw>> {
    fn run(self) -> Span<Cooked> {
        println!("✓  raw   span: {:?}", self.0);
        // Convert from Raw to Cooked by creating a new span with the same properties
        Span::<Cooked>::new(self.0.start, self.0.len)
    }
}

// macro_rules! show_span {
//     ($span:expr) => { (&Spez($span)).run() };
// }

macro_rules! show_span {
    ($span:expr) => {{
        // ───────────────────────────────────────────────────────────────
        //  1.  Local wrapper identical to the one Tolnay's trick uses
        // ───────────────────────────────────────────────────────────────
        struct __Match<T>(core::cell::Cell<Option<T>>);

        // ───────────────────────────────────────────────────────────────
        //  2.  A single-method trait – one impl per "arm"
        //      The *reference depth* (1 & vs. 2 &) makes the impls
        //      mutually exclusive, so there is never any ambiguity.
        // ───────────────────────────────────────────────────────────────
        trait __Dispatch {
            fn run(self) -> Span<Cooked>;
        }

        // ✓ cooked  ────────────────────────────────────────────────────
        impl __Dispatch for &__Match<Span<Cooked>> {
            fn run(self) -> Span<Cooked> {
                let s = self.0.take().unwrap();
                println!("✓ cooked span: {:?}", s);
                s
            }
        }

        // ✓ raw     (note the *double* '&&' here)  ─────────────────────
        impl __Dispatch for &&__Match<Span<Raw>> {
            fn run(self) -> Span<Cooked> {
                let s = self.0.take().unwrap();
                println!("✓  raw   span: {:?}", s);
                // Convert from Raw to Cooked by creating a new span with the same properties
                Span::<Cooked>::new(s.start, s.len)
            }
        }

        // ───────────────────────────────────────────────────────────────
        //  3.  Wrap the expression, borrow it once, and call `run`.
        //      • If the value is `Span<Cooked>` the first impl matches.
        //      • If it is `Span<Raw>`         the second impl matches
        //        (it needs one more `&`, so the first impl is *not*
        //        considered and there is no ambiguity).
        // ───────────────────────────────────────────────────────────────
        let __tmp = __Match(core::cell::Cell::new(Some($span)));
        (&__tmp).run()
    }};
}

fn main() {
    let normal = Span::new(12, 34);
    let cooked = Span::<Cooked>::new(12, 34);
    let raw    = Span::<Raw>::new(56, 78);

    // show_span!(normal);  // expands with Span<C = Cooked> → first arm
    // show_span!(cooked);  // expands with Span<Cooked> → second arm
    // show_span!(raw);     // expands with Span<Raw>    → third arm
    
    let result1 = show_span!(normal);  // expands with Span<C = Cooked> → first arm
    let result2 = show_span!(cooked);  // expands with Span<Cooked> → second arm
    let result3 = show_span!(raw);     // expands with Span<Raw>    → third arm
    
    println!("Results as cooked spans:");
    println!("result1: {:?}", result1);
    println!("result2: {:?}", result2);
    println!("result3: {:?}", result3);

}
