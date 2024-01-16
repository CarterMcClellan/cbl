# CBL
The "C" either stands for "Chess", or "Cheeky". Either way pretty sure the "L" stands for "Lang"
<img src="readme_assets/chess_speak.jpg" alt="Chess Bot Battle" width="300" height="300"/>

## Overview
- `scanner.rs`; converting raw files into the tokens
- `ast.rs`; struct for representation for a token which is more "interpretable"
- `parser.rs`; converting tokens into syntax tree files
- `interpreter.rs`; executing the syntax tree's directly

## WASM
You can actually compile the interpreter and run it in the browser
```bash
wasm-pack build --target web
```

This should generate a directory which looks like
```bash 
tree pkg 
pkg
├── README.md
├── cbl_lib.d.ts
├── cbl_lib.js
├── cbl_lib_bg.wasm
├── cbl_lib_bg.wasm.d.ts
└── package.json
```

You should be able to copy paste `index.html` in there, then play around with the repl with
```
cd pkg
python3 -m http.server
```

## Credits
I reference the [lox-rs](https://github.com/jeschkies/lox-rs?tab=readme-ov-file) impl a lot