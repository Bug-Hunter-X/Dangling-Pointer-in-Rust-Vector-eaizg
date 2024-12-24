This repository demonstrates a common error in Rust involving dangling pointers and vectors. The `bug.rs` file shows code that attempts to modify a vector through a raw pointer after the vector's ownership has changed, leading to undefined behavior.  The `bugSolution.rs` file provides a corrected version.