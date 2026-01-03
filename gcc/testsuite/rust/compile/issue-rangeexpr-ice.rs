/* { dg-do compile } */
/* Regression test for ICE in RangeExpr::get_outer_attrs() during macro expansion.
   The compiler was crashing with:
   internal compiler error: in get_outer_attrs, at rust/ast/rust-expr.h:3012 */

use std::ops::Range;

trait Itble<'r, T, I: Iterator<Item=T>> { fn iter(&'r self) -> I; }

impl<'r> Itble<'r, usize, Range<usize>> for (usize, usize) {
    fn iter(&'r self) -> Range<usize> {
        let &(min, max) = self;
        min..max
    }
}

fn check<'r, I: Iterator<Item=usize>, T: Itble<'r, usize, I>>(cont: &T) -> bool
{true}

fn main() {
    check(&(3, 5));
}
