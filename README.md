
# Rewrite

- [ ] Support floats
- [ ] Lexer parses numbers
- [ ] Parans parsing broken
- [ ] AST doesn't need to know about operator prec
- [ ] Environments


# Design

```
Expression: 
	digit: 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9
	number: number digit | digit
	
	// 1 - 8 + 5 * 32 / 4 - 3
	// 5*3+8 + 3
	// 5 * 7 + 3
	
	expr: binary | atomic
    binary: mul | div | add | sub
    mul | expr * atomic
    div | expr / atomic

    add | expr + atomic
    sub | expr - atomic
	
	
	atomic: ( t0 ) | number
	

```

