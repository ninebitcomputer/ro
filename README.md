```
Expression: 
	expr: t0
	
	digit: 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9
	number: number digit | digit
	
	// 1 - 8 + 5 * 32 / 4 - 3
	// 5*3+8 + 3
	// 5 * 7 + 3
	
	expr: t0
	
	// 5 + 4 - 3 - 2 + 8
	// 
	t0: add | sub | t1
	add: t0 + t1 
	sub: t0 - t1
	
	t1: mul | div | atomic
	mul: t1 * atomic
	div: t1 / atmoic
	
	
	t2: add | sub | t3
	atomic: ( t0 ) | number
	

```

