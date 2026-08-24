# Exploring Lisp from a Terminal Inside of a Windowed Shippable View

Terminal in a window for the same reason electron and shipping your renderer makes sense.
Terminal apps that hope they work in some random terminal/tty are optimistic in the way web apps that work by exposing a port.
A practical terminal in a window allows for both.

---

## 1. Specification

1. **Windowed Virtual Terminal**: Host a 100% Ratatui virtual terminal viewport (`TestBackend`) inside a desktop window powered by Slint (`MainWindow`).
2. **Steel Scheme Engine**: Embed the Steel Scheme VM (`steel-core`) to evaluate Scheme queries in-memory and parse structured hash maps into tabular datasets.
3. **Canonical Scheme Query**: Pre-load the active query buffer with:
   ```scheme
   (define thingy
     (list (hash "sid" 1 "tag" "A123" "code" 95000)
           (hash "sid" 2 "tag" "B456" "code" 75000)
           (hash "sid" 3 "tag" "A123" "code" 60000)))

   (filter (fn (x) (equal? (hash-ref x "tag") "A123")) thingy)
   ```
4. **Outer Ring 3x3 Flexbox Layout**:
   - **North Row (1 char tall, 100% wide)**:
     - `NW` / `north-west` (3 chars wide corner badge)
     - `NC` / `north-center` (fill width header: title, engine version, quick shortcuts)
     - `NE` / `north-east` (3 chars wide corner badge)
   - **West Column (3 chars wide)**:
     - `WN` / `west-north` (top explorer toggle icon: 📁)
     - `WC` / `west-center` (mid activity stream: ⚡, 📊, 💬)
     - `WS` / `west-south` (bottom toggle icon: ◱)
   - **East Column (3 chars wide)**:
     - `EN` / `east-north` (top settings icon: ⚙)
     - `EC` / `east-center` (mid utility tools: ▤, ⌘)
     - `ES` / `east-south` (bottom help trigger icon: ?)
   - **Main Column (100% fill center container)**:
     - `main-head` (top ribbon: `[▶ RUN]`, `[↻ RESET]`, tabs `[+]`, view modes `[Table Grid]`, `[Logs]`)
     - `main-body` (active panes: editor text buffer + output data grid/logs split)
     - `main-foot` (bottom status ribbon: filename, Ln/Col, dataset rows, execution timer)
   - **South Row (1 char tall, 100% wide)**:
     - `SW` / `south-west` (3 chars wide corner tag)
     - `SC` / `south-center` (fill width status bar: active focus indicator, global keybindings)
     - `SE` / `south-east` (3 chars wide corner tag)
5. **Zero Function Keys (`F1`–`F12`)**: Use standard `Ctrl+` shortcuts (`Ctrl+E` run, `Ctrl+R` reset, `Ctrl+T` new tab, `Ctrl+W` close tab, `Ctrl+B` toggle tree, `Ctrl+H` help, `Ctrl+Q` quit) and mouse clicks.
6. **Full Mouse Support**: Every button, tab, mode toggle, text line, grid cell, and splitter bar is clickable and draggable.
7. **Strict Single Responsibility Principle (SRP) & Screaming Architecture**:
   - 1 function == 1 file matching the function name (`fn nw` $\rightarrow$ `nw.rs`, `fn nc` $\rightarrow$ `nc.rs`, `fn area` $\rightarrow$ `area.rs`).
   - Type definitions reside exclusively in `type.rs` within each namespace.
   - Real 3–4 letter English words and canonical 2-letter matrix cell identifiers.
   - Nested directory hierarchy for nested logic (`src/view/north/`, `src/view/south/`, `src/view/west/`, `src/view/east/`, `src/view/main/`, `src/view/part/`).
8. **Minimal Clean Code**: Zero comments, zero dead code, zero warnings, and zero compiler errors.
9. **Compact Release Binary**: Release profile configured with size optimizations (`opt-level = "z"`, `lto = true`, `codegen-units = 1`, `panic = "abort"`, `strip = true`) producing a minimal standalone executable (< 6 MB).

---

## 2. functional object structure

```
                                                          north
                  │north-west            │                north-center            │              north-east│
                  │┌─────────────────────│────────────────────────────────────────│───────────────────────┐│
                  ││    NW               │                NC                      │                 NE    ││ 
                  │├───┬─────────────────│────────────────────────────────────────│───────────────────┬───┤│
west-north        ││WN │ ┌───────────────│────────────────────────────────────────│─────────────────┐ │EN ││ east-north
                  ││   │ │               │                                        │                 │ │   ││
                  ││   │ ├───────────────│────────────────────────────────────────│─────────────────┤ │   ││
                  ││   │ │               │                                        │                 │ │   ││
                  ││   │ │               │                                        │                 │ │   ││
───────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────
                  ││   │ │               │                                        │                 │ │   ││
west-center       ││WC │ │               │                                        │                 │ │EC ││ east-center
                  ││   │ │               │                                        │                 │ │   ││
                  ││   │ │               │                                        │                 │ │   ││
                  ││   │ │               │                                        │                 │ │   ││
───────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────
                  ││   │ ├───────────────│────────────────────────────────────────│─────────────────┤ │   ││
                  ││   │ │               │                                        │                 │ │   ││
                  ││   │ └───────────────│────────────────────────────────────────│─────────────────┘ │   ││
west-south        ││WS │                 │                                        │                   │ES ││ east-south
                  │├───┴─────────────────│────────────────────────────────────────│───────────────────┴───┤│
                  ││    SW               │                SC                      │                 SE    ││
                  │└─────────────────────│────────────────────────────────────────│───────────────────────┘│
                  │south-west            │                south-center            │              south-east│
                  │                      │                 south                  │                        │
```
must flex as expected
must be in ratatui
must be in a slint window by default 
(must work as ratatui app in system terminal, tty required, but not until the slint window is correct)
must compile.
cargo check
must support both window or system renderer
must compile.
sequence selection iteration
the terminal needs to run in our slint window
the terminal needs to run our ratatui code/our ratatui app 
the terminal needs to support high color 
---

## 3. Strict Screaming Directory & File Hierarchy

```
src/
├── main.rs                    # Slint desktop runner & Ratatui frame bridge
├── code.rs                    # Canonical Scheme query constant (thingy)
│
├── core/                      # [Namespace: State & Event Routing]
│   ├── mod.rs
│   ├── type.rs                # App, Focus
│   ├── init.rs                # fn init
│   ├── exec.rs                # fn exec
│   ├── rset.rs                # fn rset
│   ├── keys.rs                # fn keys
│   └── mice.rs                # fn mice
│
├── eval/                      # [Namespace: lisp evaluation & table extraction]
│   ├── mod.rs
│   ├── type.rs                # Vm, Res, Grid, Col
│   ├── eval.rs                # fn eval
│   ├── form.rs                # fn form
│   └── pick.rs                # fn pick
│
├── text/                      # [Namespace: Query Text Buffer & Syntax Scanning]
│   ├── mod.rs
│   ├── type.rs                # Tab, Book
│   ├── scan.rs                # fn scan
│   ├── edit.rs                # fn put_c, put_nl, del_l, del_r
│   └── step.rs                # fn go_l, go_r, go_u, go_d, go_sol, go_eol, pg_u, pg_d
│
├── tree/                      # [Namespace: Navigation Tree]
│   ├── mod.rs
│   ├── type.rs                # Nav, Item, Kind
│   ├── look.rs                # fn look
│   ├── next.rs                # fn next
│   └── prev.rs                # fn prev
│
├── pane/                      # [Namespace: Results Output View Model]
│   ├── mod.rs
│   ├── type.rs                # Pane, Mode
│   ├── show.rs                # fn show
│   └── flip.rs                # fn flip
│
├── flex/                      # [Namespace: Yoga Flexbox Layout Calculation]
│   ├── mod.rs
│   ├── type.rs                # Node, Rect, Axis
│   ├── calc.rs                # fn calc
│   └── leaf.rs                # fn leaf
│
└── view/                      # [Namespace: Ratatui TUI Rendering Engine]
    ├── mod.rs
    ├── type.rs                # Hits
    ├── tint.rs                # Theme color tokens (ACC, BG, BAR, MUT, TXT, PAN)
    ├── draw.rs                # fn draw
    ├── help.rs                # fn help
    │
    ├── north/                 # [Namespace: North Row 3-Cell Matrix]
    │   ├── mod.rs
    │   ├── nw.rs              # fn nw (north-west corner cell)
    │   ├── nc.rs              # fn nc (north-center header cell)
    │   ├── ne.rs              # fn ne (north-east corner cell)
    │   └── area.rs            # fn area (north row composite)
    │
    ├── south/                 # [Namespace: South Row 3-Cell Matrix]
    │   ├── mod.rs
    │   ├── sw.rs              # fn sw (south-west corner cell)
    │   ├── sc.rs              # fn sc (south-center status cell)
    │   ├── se.rs              # fn se (south-east corner cell)
    │   └── area.rs            # fn area (south row composite)
    │
    ├── west/                  # [Namespace: West Column 3-Cell Matrix]
    │   ├── mod.rs
    │   ├── wn.rs              # fn wn (west-north top cell: 📁)
    │   ├── wc.rs              # fn wc (west-center mid cell: ⚡, 📊, 💬)
    │   ├── ws.rs              # fn ws (west-south bottom cell: ◱)
    │   └── area.rs            # fn area (west column composite)
    │
    ├── east/                  # [Namespace: East Column 3-Cell Matrix]
    │   ├── mod.rs
    │   ├── en.rs              # fn en (east-north top cell: ⚙)
    │   ├── ec.rs              # fn ec (east-center mid cell: ▤, ⌘)
    │   ├── es.rs              # fn es (east-south bottom cell: ?)
    │   └── area.rs            # fn area (east column composite)
    │
    ├── main/                  # [Namespace: Main Column Container]
    │   ├── mod.rs
    │   ├── head.rs            # fn head (main-head controls ribbon)
    │   ├── body.rs            # fn body (main-body active panes)
    │   ├── foot.rs            # fn foot (main-foot status ribbon)
    │   └── area.rs            # fn area (main column composite)
    │
    └── part/                  # [Namespace: Specific Pane Renderers]
        ├── mod.rs
        ├── tree.rs            # fn tree
        ├── text.rs            # fn text
        ├── grid.rs            # fn grid
        ├── logs.rs            # fn logs
        └── pane.rs            # fn pane
```

---

## 4. Release Profile Optimizations for Small Binary Size

In `Cargo.toml`:
```toml
[profile.release]
opt-level = "z"     # Optimize for size
lto = true          # Link-Time Optimization
codegen-units = 1   # Single codegen unit for max optimization
panic = "abort"     # Remove unwind tables
strip = true        # Strip all symbols
```

---

## 5. Verification

1. **Static Analysis**: `cargo check --all-targets` passes with 0 errors and 0 warnings.
2. **Release Build**: `cargo build --release` produces a standalone executable under 5.5 MB.
3. **Runtime Verification**: The application boots in a Slint window, renders the Ratatui virtual terminal with the complete 3x3 outer ring flexbox layout, runs `thingy.scm` in Steel VM, formats the results into a table grid, and processes mouse clicks and `Ctrl+` shortcuts.
