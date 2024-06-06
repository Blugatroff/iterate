# iterate

Iterate will repeat the command its given whenever it receives "run" on port `7821`.
This is useful if you want to have a global shortcut to repeat an arbitrary command.

I like to have a terminal on my second monitor where i repeatedly typecheck my code,
using iterate, i don't have to manually focus this terminal window and press `↑+↵`.
Instead i can just press [my global shortcut](https://github.com/Blugatroff/diversions/blob/43552cfdd7ac481bb968fac6ab17e29b428db9be/main.lua#L230) for running `echo "run" | nc 127.0.0.1 7821`
and trigger iterate.

This is especially useful when you don't have a lsp server or some sort of builtin
--watch flag for your tool.

## Example
```bash
iterate cargo check
```

## How to trigger
```bash
echo "run" | nc 127.0.0.1 7821
```

## Configuration
If you don't want iterate to print the exit code of your command, you can set the `ITERATE_QUIET` variable.
```bash
ITERATE_QUIET=1 iterate launch nukes
```

