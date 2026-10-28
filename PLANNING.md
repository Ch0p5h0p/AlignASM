# AlignASM Planning

AlignASM is a grid-style assembly formatter.

## v1.0
- Formatting capabilities for Intel ASM

## v1.5
- Better command interface
- Abilitiy to be used with Intel ASM in services that use formatters (testing with conform.nvim)

### Commands
| Invocation | Function |
| ---------- | -------- |
| `alignasm` | Read ASM from stdin, write formatted ASM to stdout |
| `alignasm file` | Read ASM from a file, write formatted ASM to stdout |
| `alignasm --in_place file` | Format a file in place |
| `alignasm --check file` | Exit with a nonzero status if the file needs formatting changes |

## v2.0
- Formatting capabilities for AT&T ASM, basic ARM support
