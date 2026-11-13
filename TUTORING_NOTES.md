# Tutoring notes

## Initial baseline — 2026-10-21

- Radan has completed Parts I and II of the book and implemented Part II in Ruby.
- Rust experience is self-described as familiar but rusty. Revisit concepts as they arise; no specific gaps have been identified yet.

## Concepts needing practice

- Lifetime relationships (2026-11-13): correctly identified that compiled chunks do not borrow source and that source need not live as long as the VM. Needed guidance on elided return lifetimes connecting compiler output to source, and on using a static name lifetime independently of chunk ownership. Revisit with future borrowed token/source representations.
- Trait method visibility (2026-11-13): imported Write and successfully flushed stdout after explanation; revisit why a trait must be in scope when a new trait-provided method arises. Implementation demonstrated, independent explanation not yet assessed.
- Reusable VM state (2026-11-13): correctly reasoned that ip must reset and implemented reset in run. Stack cleanup after failure remains an open question; do not treat successful repeated-run coverage as proving cleanup.

- Newtype runtime representation: explained zero-cost wrapping versus the explicit layout/ABI guarantee of `repr(transparent)`; this representation distinction has not yet been assessed (2026-10-23).
- Built-in types versus the standard-library prelude: distinction introduced while discussing C's common headers; understanding not yet assessed (2026-10-22).
- Derived `Debug`, compile-time macro generation versus runtime reflection, and `stringify!` operating on source tokens: explained in response to Radan's opcode-name question; understanding not yet assessed (2026-10-23).

## Tutoring pacing

- Radan values expressiveness from his Ruby background and explicitly wants to explore expressive Rust designs, including macros. Treat this as part of the learning goals; weigh abstractions by their learning value as well as implementation simplicity. Retain the chosen automatic opcode formatting and defer snake case until a multiword opcode arises (2026-10-23).
- Anchor explanations in the section Radan has read. The enum-versus-byte-stream discussion introduced operands from 14.5.3 while Radan was at 14.3.1; defer the representation decision until the relevant material arises. Distinguish the book's explicit claims from Rust-specific comparisons (2026-10-22).
- Session ended after section 14.3.1; resume at 14.4. Do not treat the open representation choice as a correctness defect in the current implementation or a prerequisite to continuing (2026-10-22).

## Demonstrated understanding

- Chapter 16.1 (2026-11-13): collected CLI arguments, correctly counted the executable name, implemented prompt flushing, EOF termination, and Result-based file error handling with failure exit status and stderr output.
- Chapter 16.1 (2026-11-13): reasoned that the compiler should report compilation errors separately from VM runtime errors. Implemented a unit-struct error after discussion of why an empty enum cannot be constructed.
- Chapter 16.1 (2026-11-13): recognized that a standalone compile function needs no self parameter, and that consuming self prevents VM reuse; changed interpret to borrow mutably. Moved compiled chunks into VM ownership with syntax guidance.
- Chapter 16.1 (2026-11-13): adapted execution tests to run on prepared chunks without needing a parser and added a two-chunk repeated-run test; 11 tests passed at last verification. Correctly proposed None as a terminating scanner stub.

Prior completion of the book's earlier parts is background, not an assessment from this session.

- Independently applied the same-file unit-test pattern to Chunk, importing parent-module items, building a chunk, and asserting exact disassembly output. Both Value and Chunk tests pass under cargo test (2026-10-27).
- Implemented a mutable impl std::io::Write output parameter with explicit expect handling, stdout at the application call site, and Vec<u8> in the test. Correctly uses String::from_utf8 and concat! with explicit newlines to compare the output. Unit-testing refresher complete; no current difficulty identified (2026-10-27).
- Correctly identifies a mutable reference as having its own type, distinct from the underlying value's type, explaining the difference between the stdout caller variable and disassemble's output parameter. Automatic method borrowing/reborrowing has been explained but not independently assessed (2026-10-27).
- Correctly explains why disassembly borrows the chunk immutably and its output target mutably: only the output target is modified. Deliberately accepts panicking on output failures for now; writer implementation remains to be reviewed (2026-10-27).
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

## Section 14.6 review — 2026-10-27

- Demonstrated in implementation: associates each typed instruction with a source line through write; displays the first line explicitly and uses a marker only for consecutive equal lines. The initial-index check also handles a first line of zero correctly.
- Needs follow-up: a Constant output arm changed from println to print, leaving its row unterminated. Ask about the output of Constant followed by Return to revisit row termination; no conclusion about conceptual difficulty until Radan responds.

- Follow-up assessment (2026-10-27): resolved the row-termination concern. Radan correctly predicted the merged output, restored println for Constant, and reordered the example to exercise Constant followed by Return. Execution confirms correct separate rows. This demonstrates understanding of print versus println; no further practice needed on this issue currently.

- Chapter 15.1.1: Radan independently proposed both a zero-based instruction index and an iterator; explore iterator borrowing and VM access to the chunk before assessing understanding (2026-11-02).
- Demonstrated understanding (2026-11-02): multiple immutable borrows can coexist. Revisit distinction between ownership and restrictions imposed by active borrows.
- Demonstrated understanding (2026-11-02): recognized that VM control flow can move the instruction position beyond sequential iteration and connected that to choosing an index.
- Demonstrated understanding (2026-11-02): reasoned that the chunk created in main can outlive interpretation, allowing the VM to borrow it.
- Demonstrated understanding (2026-11-02): reasoned transitively about validity of the chunk name and VM's borrow, and applied lifetime annotations successfully.
