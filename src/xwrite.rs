// Adds macros that wrap write and writeln macros but expects the output to
// succeed. xwrite is short for "expected write".
// This is so we can use it like print and println but with a target output
// for better composability and testability.

macro_rules! xwrite {
    ($output:expr, $($args:tt)*) => {
        write!($output, $($args)*).expect("failed to write to output")
    }
}

macro_rules! xwriteln {
    ($output:expr, $($args:tt)*) => {
        writeln!($output, $($args)*).expect("failed to write to output")
    }
}
