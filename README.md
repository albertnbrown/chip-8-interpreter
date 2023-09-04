# chip-8-interpreter
rust interpreter/emulator for chip-8 code

check if you have the c packages for the dependencies!

run as so from root directory: `cargo run path/to/file mode`
the mode can be omitted for default of `0`

available modes are:
```
0 -> chip-8
1 -> chip-8 with schip quirks
2 -> chip-8 with x0-chip quirks
```

I don't do chip-8 level display wait (boring)
looking to add actual schip and x0-chip opcode handling
