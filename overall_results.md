 Crate a | Crate b | Behavior |
| - | - | - |
| Non-generic function | Uses once in a module | Crate-a function doesn't appear in any codegen units of Crate-b |
| Non-generic function | Uses in two different modules |Crate-a function doesn't appear in any codegen units of Crate-b |
| Non-generic `#[inline]` function | Uses once in a module |Crate-a function appears with in a single codegen unit of Crate-b, and exists even after post-inlining stage|
| Non-generic `#[inline]` function | Uses in two different modules |Crate-a function appears with in a single codegen unit of Crate-b, and exists even after post-inlining stage|
| Generic function | Uses once in a module | Regardless of inlining all monoporphized (specialized) functions from Crate-a appear within a single codegen unit for Crate-b. The codegen unit exists even after the post inlining stage.|
| Generic function | Uses in two different modules | - same - |
| Generic `#[inline]` function | Uses once in a module |  - same - |
| Generic `#[inline]` function | Uses in two different modules | - same - |
