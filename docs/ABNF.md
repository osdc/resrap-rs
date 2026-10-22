# ABNF — Awesome BNF

**ABNF (Awesome BNF)** is the grammar language used by Resrap. It extends EBNF
with weighted choices and an infinite-generation operator, and is designed for
high-throughput stochastic code generation.

## Rules

A grammar is a sequence of named rules:

```abnf
function : rules ;
```

- **function** is the non-terminal name.
- The part between `:` and `;` is a sequence of non-terminals, terminals and
  character classes.
- Rule names are referenced by writing their name.
- A rule may be empty.
- Parsing fails on undefined rule references, malformed tokens or invalid
  probability values.

### Standard EBNF operators

| Operator | Meaning |
| -------- | ------- |
| `+` | One or more repetitions of the preceding element |
| `*` | Zero or more repetitions |
| `?` | Optional element (zero or one occurrence) |
| `()` | Grouping; the enclosed sequence is treated as one element |

```abnf
expression : operand (operator operand)* ;
```

The parsed grammar is a directed graph. Nodes reference one another, which is
what allows recursion and infinite generation.

## Terminals and character classes

Single-quoted strings are emitted literally:

```abnf
program : 'hello world';
```

A character class emits one character chosen from the class:

```abnf
identifier : [a-z][a-z0-9_]*;
number     : [0-9]+;
```

Supported escapes inside terminals and character classes include `\n`, `\t`,
`\r`, `\\`, `\'`, `\"` and range separators such as `[a-z]`, `[0-9]` and
`[a-zA-Z0-9_]`.

## Infinite generation (`^`)

The `^` operator loops the preceding construct until the token budget is
reached:

```abnf
program : function^;
```

Generation does not halt when the loop is reached; it returns to the beginning
of the construct and continues. With `generate_complete`, the walk leaves the
loop once the budget is exhausted and then finishes the surrounding rule.

## Probabilities (`<...>`)

By default, alternatives are equally likely. Attach a probability in angle
brackets to weight a branch or loop:

```abnf
char : a<0.2> | b<0.8>;
a    : 'A';
b    : 'B';
```

- `a` is chosen about 20% of the time and `b` about 80%.
- Weights are normalized automatically; they do not need to sum to 1.
- A missing probability means equal weighting.

Repetition can also be weighted:

```abnf
a : b+<0.4>;
```

The same syntax applies to `*`, `?` and grouped constructs. Probabilities are
validated: negative, non-numeric, NaN and infinite values are rejected.

## Word lists (`@words`)

Word lists let a rule emit a readable token instead of expanding its grammar.
This is useful for identifiers, keywords and realistic sample data.

```abnf
@words identifier = count, total, index, value, result
@keywords datatype = int, long, double

program: datatype ' ' identifier ';\n';
```

- A directive occupies one line and ends at the line break.
- `@keywords` is an alias for `@words`.
- Any rule name may be used, including one with no rule definition.
- A reference to the rule emits one word chosen uniformly.
- Word lists also apply when the rule is used as the starting rule.
- Words can be attached at runtime with `Resrap::add_words` or
  `Resrap::add_keywords`.

## Comments and whitespace

`#` and `//` start a line comment:

```abnf
// This is a comment.
program : 'ok' ; # So is this.
```

Whitespace between tokens is ignored unless it is inside a quoted terminal or a
character class.

## Summary

ABNF extends standard BNF/EBNF with:

- repetition operators `+`, `*`, `?`
- grouping with `()`
- infinite generation with `^`
- weighted choices with `<prob>`
- word lists with `@words` / `@keywords`
- comments, escapes and character-class ranges

The dialect matches the Go reference implementation. Seeded output is tested
byte-for-byte for compatibility; see
[../tests/go_parity.rs](../tests/go_parity.rs) and the fixture data in
[../tests/data/go_parity_data.rs](../tests/data/go_parity_data.rs).
