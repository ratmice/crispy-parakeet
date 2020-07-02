# Description:
a port of the rowan calc example using lalrpop & logos as the parser/lexer.
The concrete syntax tree produced is fairly clunky still.

This repo is a demonstration of the current state, and will probably be deleted or moved elsewhere eventually.

## What works
However it does successfully produce a syntax tree, passing the builder through the parser
and getting a syntax tree out.  The way this works is the grammar gets passed a `&mut Builder`
which the grammar does not return, thus the mutable reference goes out of scope, the caller then
has exclusive ownership.

## branches
there are branches for sorbus, rowan, and the rowan thread_local_cache branch.
The resulting tree ends up something like a reverse polish notation.

## AST Clunkiness

* Doesn't include whitespace for no good reason other than i'm too lazy.
* The top-down builder API isn't a natural fit for the bottom up parser
* We end up with `Oper` nodes of 2 or 3 children
  In a stack-like semantics rather than consistent 3 nodes like the rowan calc example.

```
- Root
  - Oper
    - Oper
      - "2" Number
      - "3" Number
      - "*" Mul
    - "1" Number
    - "+" Add
  - Oper
    - "4" Number
    - "-" Sub
```
## TODO ...

Needs to test out a branch based on the rowan pr #63

## Other thoughts

Sorbus has a `finish_node_at`, function perhaps this could be more useful for constructing bottom-up
however I haven't thought through it.

It seems like an appropriate API might be something like:

`reparent_current_node_onto(self, kind: SyntaxKind)`, which would take the current node
and make it a child of the new node of kind.

