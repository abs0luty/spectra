# Spectra programming language

Programming language made for my tutorial videos ([my youtube channel](https://www.youtube.com/channel/UCHgP6H2lfL86qOfz8WpCliQ)):

<p align="center">
	<img alt="Untitled-1" src="https://github.com/abs0luty/spectra/assets/68709264/2ac7767c-e55c-4d40-b87b-cef9596c4ba3" width="50%">
</p>

## Syntax

Declaring a variable:

```
var a = 3;
```

Function expression:

```
var even = fun (n) {
	n % 2 == 0
};
```

If expression:

```
print("number a is " + if a % 2 == 0 {
	"even"
} else {
	"odd"
});
```

While expression:

```
while true {
	println("HELLO!");
}
```

Classes:

```
class Point {
	x, 
	y,
	constructor (x, y) {
		this.x = x;
		this.y = y;
	}
	distance_from (other_point) { todo() }
}
```

## Progress

- [ ] Lexer
  - [x] Tokenize identifiers
  - [ ] Tokenize string literals (without escape sequences)
  - [ ] Tokenize char literals (without escape sequences)
  - [ ] Process escape sequences
  - [ ] Tokenize integers
  - [ ] Tokenize floats
  - [ ] Process comments
- [ ] Parser
  - [ ] Parse expressions
    - [x] Parse binary expression
    - [x] Parse postfix expression
    - [x] Parse prefix expression
    - [ ] Parse if expression
    - [ ] Parse while expression
  - [ ] Parse statement
    - [x] Parse `break` statement
    - [x] Parse `continue` statement
    - [x] Parse expression statement
    - [ ] Parse class statement
- [ ] Walk tree interpreter
  - [ ] Implement scopes
  - [ ] Evaluate expressions
    - [ ] Parse binary expression
    - [ ] Parse postfix expression
    - [ ] Parse prefix expression
    - [ ] Parse if expression
    - [ ] Parse while expression
  - [ ] Evaluate statements
    - [ ] Evaluate `break` statement
    - [ ] Evaluate `continue` statement
    - [ ] Evaluate expression statement
    - [ ] Evaluate class statement
- [ ] VM
- [ ] VM Bytecode Compiler

