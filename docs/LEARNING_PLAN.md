# Learning Plan

## Baseline and approach

Current verified baseline: a fresh Rust 2024 Cargo binary project created with
`cargo new cosmic-calculator`. The standard Hello World application compiles and
runs successfully. No calculator functionality or dependencies have been added.

**Current stage: preparation. Stage 1 has not started.**

Build and test the calculator engine before introducing libcosmic. The learner
writes the core learning code; AI tools support planning, review, and mechanical
tasks. These stages are provisional and may change as we learn.

## Provisional stages

### 1. Addition

- **Working capability:** Add two integers through a small function.
- **Concepts:** Functions, integer types, return values, and tests.
- **Completion criteria:** Tests cover positive, negative, and zero operands and
  pass with `cargo test`. The learner can explain the function's parameter and
  return types and the integer type's range.

### 2. Arithmetic operations

- **Working capability:** Select addition, subtraction, multiplication, or
  division using an operation enum; use nonzero divisors at this stage.
- **Concepts:** Enums, `match`, and pattern matching.
- **Completion criteria:** Every operation has a passing test. The learner can
  explain how the enum selects behavior and how integer division behaves.

### 3. Calculator state

- **Working capability:** Store a calculator's value and pending operation, then
  update or clear them through methods.
- **Concepts:** Structs, `impl` blocks, methods, and mutability.
- **Completion criteria:** Tests demonstrate initial state, an operation that
  updates state, and clearing back to initial state. The learner can explain
  which methods mutate the calculator and why.

### 4. Error handling

- **Working capability:** Report division by zero without panicking and represent
  absent values or operations explicitly.
- **Concepts:** `Result`, `Option`, and handling success, absence, and failure.
- **Completion criteria:** Tests cover successful division, division by zero,
  and missing input or operation. The learner can explain when to use `Result`
  versus `Option` and how callers handle each case.

### 5. Input sequences

- **Working capability:** Build numeric input from successive characters and
  process a sequence of inputs through evaluation and clearing.
- **Concepts:** `String`, ownership, and borrowing.
- **Completion criteria:** Tests cover multi-digit input, a complete calculation
  sequence, clearing, and invalid input. The learner can explain where input
  strings are owned, borrowed, and changed.

### 6. COSMIC integration

- **Working capability:** Use the tested engine from a native COSMIC calculator
  interface with a display and calculator controls.
- **Concepts:** libcosmic, widgets, messages, and application updates.
- **Completion criteria:** The application launches in the target COSMIC
  environment. Manual checks demonstrate entering a calculation, displaying its
  result, clearing, and showing a division-by-zero error. Engine tests still pass,
  and the learner can trace a button action through a message to a display update.

## Progress checklist

- [x] Create the Cargo binary project and verify Hello World runs.
- [x] Write the initial README and learning plan.
- [ ] Stage 1: Addition.
- [ ] Stage 2: Arithmetic operations.
- [ ] Stage 3: Calculator state.
- [ ] Stage 4: Error handling.
- [ ] Stage 5: Input sequences.
- [ ] Stage 6: COSMIC integration.

Mark a stage complete when its observable criteria are met and the learner can
explain the relevant code.

## Discoveries and changes

Record surprises, questions, and plan adjustments here as we learn. Note why a
decision changed and which stage it affects.

| Date / stage | Discovery or question | Decision or next step |
| --- | --- | --- |
| — | — | — |
