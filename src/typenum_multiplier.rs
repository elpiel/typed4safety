use core::ops::Mul;

use typenum::Unsigned;

pub const fn compile_time_mult<X, Y>() -> usize
where
    X: Mul<Y>,
    <X as Mul<Y>>::Output: Unsigned,
{
    <X as Mul<Y>>::Output::USIZE
}
