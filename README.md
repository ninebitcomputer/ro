# Language

## Expressions

### Binary

`*`: Multiplication
`/`: Division

`+`: Addition
`-`: Subtraction

### Unary

`-`: Negation
`+`: Plus

## Statements

```
Type ident [ = Expression ] ;
if Expression Block [ else if Block ] [ else Block ] ;
```

### Blocks

```
{ [ Statement* ] }
```

## Types

natural
real

# Testing

`cargo test -- --nocapture` for ast parsing display

# Todo

- [ ] Boolean type
- [ ] Bound to unbound conversion
- [ ] Environments within AST
