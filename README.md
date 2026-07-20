# Computer Geometry System

A geometry equation solver that runs on Numworks calculators (Rust) with a reference implementation in Python.

## Features

- Equation solving using a built-in CAS (symbolic manipulation, solving for any variable)
- Multi-line text editor for entering equations on the calculator
- Interactive variable selection (choose which variable to solve for and enter values for unknowns)
- Methods-driven geometry system (geometry shapes and their properties/relations defined in JSON method files)
- Python reference implementation (TUI app for testing the methods system and input parsing)

## Python (reference)

```bash
cd python
uv sync
uv run python main.py
```

A 3-tab TUI: **Given** (enter geometry facts like `circle:radius=5`), **Find** (what to solve for), **Run** (parse input and view results).

## Rust (calculator app)

Requires Rust, ARM embedded toolchain, the Epsilon simulator, and other dependencies. On Debian-based Linux:

```bash
cd rust
bash setup.sh
just sim           # build and run in the Epsilon simulator
```

### UI navigation (calculator)

| Action | Key |
|---|---|
| Type equation | Calculator keys (Alpha for letters, Shift for symbols) |
| Move cursor | Left / Right / Up / Down |
| Backspace | Backspace |
| New line | Ans |
| Solve | Back or Exe |
| Select variable | Up / Down + Ok |
| Enter numeric value | Calculator keys + Ok |
| Dismiss result | Ok / Back / Exe |

The app boots into the equation editor. Type an equation (e.g. `A = pi * r^2`), press **Exe**, choose a variable to solve for, enter values for remaining unknowns, and view the result.

### Other commands

See `rust/justfile`: `just build-epsilon`, `just build-upsilon`, `just send-epsilon`, `just send-upsilon`, `just check`.

## Known Issues

- Solving some equations may not yield all possible results.

## AI Declaration

I don't think I really used AI to help develop this.