# Galvan-compiler
This is a basic (WIP) compiler for a language I call Galvan. By syntax the language is basically just C or Rust, a bit of both but not really either. It's just a practice language so there's nothing revolutionary, see capabilities for up-to-date details on what it can do. 

## Current capability
The language isn't done yet, so this'll update with time.

### Lexer
The lexer is mostly running and in prod order, I'll probably keep it as-is until I finish the rest of the language. It can separate text into symbols, that are then gonna be used in the parser and so on. I think it works fairly well and I'm somewhat proud of it, however the structs (especially LexSymbol) is kind-of ass. 

### Parser
The parser is what I'm currently working on (as of writing, 30/10/2025), it can separate variables and make them into expressions. That's about it. Most of the parser structure comes from its Structs and Enums anyway so the work is there to continue. 

## Future capability
I'm hoping to be able to make the language into a somewhat functional system, with decent enough power on the computer it's running on, some form of embedded ASM just to give it a small boost or something, or maybe embedded C because that'd be funny (probably not). In addition to that, I wish I could make everything as modular as possible, try to get a good standard package running, maybe some kind of imports, I don't know yet. I was hoping to be able to make it into a decent Embedded Systems Language, but we'll see how that goes.