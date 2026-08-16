pub mod token_interpreter;
pub mod parser;
pub mod interpreter;
pub mod optimizations;
pub mod error;
pub mod compiler;
pub mod io;

macro_rules! assert_eq_res {
    ($left:expr, $right:expr $(,)?) => {{
        let left = &$left;
        let right = &$right;

        if left != right {
            return Err(format!(
                "assertion failed\nleft:  {:?}\nright: {:?}",
                left, right,
            ).into());
        }
        else {
            Ok(())
        }
    }};
}