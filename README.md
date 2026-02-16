# text-editor

A text editor TUI in `rust`.

---

## usage

1. run the prorgam `cargo run /path/to/file` (development)
2. edit the file content, move with keyboard arrows to navigate the content, use `ENTER` for line breaks
3. use `CTRL + S` to save changes
4. use `CTRL + Q` to quit

#### to do

- undo / redo
- new file
- scroll content if content size is greater than terminal height

---

---

## requirements

|         |                                 |
| ------- | ------------------------------- |
| `rustc` | `1.91.1 (ed61e7d7e 2025-11-07)` |
| `cargo` | `1.91.1 (ed61e7d7e 2025-11-07)` |

#### systems

- Linux `x86_64` with `6.18.8` kernel

---

---

## development

```sh
# edit the file at /path/to/file
cargo run /path/to/file

# build
cargo build
```

---

---

## references

- https://docs.rs/crossterm/latest/crossterm/
- https://docs.rs/crossterm/latest/crossterm/event/enum.KeyCode.html#variants
