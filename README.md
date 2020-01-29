# Code practice for the Rust book

This is a repository of the code practice bits in *The Rust Programming Language*: https://doc.rust-lang.org/book/. Each directory corresponds to a chapter or subchapter of the book, matching its URL when possible (some examples spanning across several subchapters are merged into one directory, i.e. the extensive restaurant example of chapter 7).

Everything should compile without warning and binaries should run:
  - Warnings (dead_code, unused_variables) are disabled when it makes sense.
  - Code samples that are not functional are commented out.

For code samples from the "core" text, functions and structs declarations are often inlined straight in the main() function body to follow the book reading flow. This might feel a little awkward, but the idea is to prevent mixing up concepts to much. Coding exercises ("ex") are written using a more "traditional" fashion.