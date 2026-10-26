# Tutoring notes

## Initial baseline — 2026-10-21

- Radan has completed Parts I and II of the book and implemented Part II in Ruby.
- Rust experience is self-described as familiar but rusty. Revisit concepts as they arise; no specific gaps have been identified yet.

## Concepts needing practice

- Newtype runtime representation: explained zero-cost wrapping versus the explicit layout/ABI guarantee of `repr(transparent)`; this representation distinction has not yet been assessed (2026-10-23).
- Built-in types versus the standard-library prelude: distinction introduced while discussing C's common headers; understanding not yet assessed (2026-10-22).
- Derived `Debug`, compile-time macro generation versus runtime reflection, and `stringify!` operating on source tokens: explained in response to Radan's opcode-name question; understanding not yet assessed (2026-10-23).

## Tutoring pacing

- Radan values expressiveness from his Ruby background and explicitly wants to explore expressive Rust designs, including macros. Treat this as part of the learning goals; weigh abstractions by their learning value as well as implementation simplicity. Retain the chosen automatic opcode formatting and defer snake case until a multiword opcode arises (2026-10-23).
- Anchor explanations in the section Radan has read. The enum-versus-byte-stream discussion introduced operands from 14.5.3 while Radan was at 14.3.1; defer the representation decision until the relevant material arises. Distinguish the book's explicit claims from Rust-specific comparisons (2026-10-22).
- Session ended after section 14.3.1; resume at 14.4. Do not treat the open representation choice as a correctness defect in the current implementation or a prerequisite to continuing (2026-10-22).

## Demonstrated understanding

Prior completion of the book's earlier parts is background, not an assessment from this session.

- Correctly chooses a newtype rather than a type alias to require explicit construction of Value and prevent accidental use of ordinary floats where a Value is expected (2026-10-23).
- Correctly defines Value using a single-field tuple struct wrapping `f64` (2026-10-23).
- Correctly applied public visibility to both Value and its tuple field, allowing direct construction from sibling modules once module wiring is in place (2026-10-26).
- After discussing the child-module lookup error, correctly identified main.rs as the declaration location that makes chunk and value sibling modules (2026-10-26).
- Correctly declared value at the crate root and used a crate-root import in chunk.rs; module resolution now succeeds (2026-10-26).
- Changed add_constant to accept Value and store it directly, enforcing explicit wrapping at the caller (2026-10-26).
- Added a constructor to initialize Chunk's private constant pool and used it from main, resolving the struct privacy compilation error; opcode initialization still needs correction (2026-10-26).
- Resolved the ignored-constructor-argument issue by deliberately constructing an empty Chunk and appending instructions through write. Kept storage private and moved disassembly into an immutable method. Execution confirms Return then Constant referring to pool index 0 (2026-10-26).
- Correctly explains that Chunk's borrowed name must remain valid for the chunk's lifetime. Clarified validity for the duration the borrow is needed and that annotations do not extend the owner's lifetime; current string literal has static lifetime (2026-10-26).
- Independently identified the need to resolve a constant-pool index to its value for useful disassembler output (2026-10-26).
- Correctly changed read_constant's return type and indexed expression to borrow Value. Compiler confirms the move error is resolved without cloning or explicit lifetime annotations; lifetime elision was explained but not independently assessed (2026-10-26).
- Correctly identifies f64 as already supporting numeric display while its Value newtype does not automatically inherit Display (2026-10-26).
- Implemented appending a constant and returning its zero-based index with Vec length minus one, using mutable self borrowing and usize appropriately; execution now confirms the first index is zero and its value is 1.2 (2026-10-26).
- Correctly implemented context-aware constant formatting and independently implemented Display for Value by delegating to f64. Execution prints the constant index and value without warnings; formatting check passes (2026-10-26).
- Correctly recognized that Display for Value removes the need for disassembly to access its numeric field directly, and updated the call to format the borrowed Value itself. Execution confirms the same correct output (2026-10-26).
- Recognizes zero as the conventional successful process exit status and recalls Rust's tail-expression return behavior (2026-10-22).
- Recalls that omitting a function return type means no meaningful return value; initially used C's `void` terminology. Introduced Rust's unit type (`()`) (2026-10-22).
- Correctly reasons that an integer tail expression causes a type mismatch in a function returning unit (2026-10-22).
- Correctly implemented `main` returning `ExitCode::SUCCESS`; compilation and execution confirmed exit status zero. Unit-returning `main` was also explained as an alternative (2026-10-22).
- Correctly explains that a semicolon turns the tail expression into a statement, leaving the body with unit and causing a mismatch with the declared `ExitCode` return type. No implicit conversion applies (2026-10-22).
- Correctly declared the `chunk` module from `main.rs` and applied UpperCamelCase naming to the `Return` variant. Compiler feedback confirms `chunk.rs` is included (2026-10-22).
- Identifies `Vec` as Rust's standard dynamic array, avoiding a custom implementation of the book's C allocation machinery (2026-10-22).
- Independently proposes data-carrying enum variants to represent instructions with operands; correctly recognizes that Rust enums can contain payloads (2026-10-22).
- Correctly explains uniform enum element size accommodating the largest variant. Connected this to the memory tradeoff between typed instruction vectors and variable-length packed bytecode; exact layout may include discriminant and padding or compiler optimizations (2026-10-22).
- Implemented the section 14.4 disassembler using borrowed iteration, `enumerate()`, zero-padded indices, and a match returning an opcode name. Execution and formatting checks pass (2026-10-23).
- Correctly explains that safe Rust's typed enum prevents constructing an unknown opcode, removing the need for an unknown-opcode branch in the current disassembler. Explained the additional protection from exhaustive matching when adding new variants (2026-10-23).
- Successfully applied derived `Debug` and debug formatting to the opcode enum. Introduced `format!` for producing an owned string and `to_uppercase()` for transforming it; transformation not yet reviewed (2026-10-23).
- Independently recognized that uppercasing alone cannot produce underscore-separated labels from multiword enum variants, and investigated a case-conversion crate (2026-10-23).
