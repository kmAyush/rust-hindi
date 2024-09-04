# Toy implementation of Rust in Hindi
As a way to learn lexer, parser and abstract syntax tree to learn about the working of compiler. Still a lot code implementation is required to cover all cases, it works for some simple case right now.


<strong>Source Code -> Lexer/Lexical Analyzer [Tokens] -> Parser [Statements] -> Abstract Syntax Tree</strong>

### Code input in Hindi 
```commandline
जब संख्या = 10;
यदि संख्या > 5 {
    तब("संख्या बड़ी है 5 से");
}
```

### Output
```commandline
/home/$ cargo run 
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/hindi`
----------- Tokens -----------
Let
Identifier("स\u{902}ख\u{94d}या")
Equals
Number(10)
Semicolon
If
Identifier("स\u{902}ख\u{94d}या")
GreaterThan
Number(5)
LeftBrace
PrintlnMacro
StringLiteral("स\u{902}ख\u{94d}या बड\u{93c}ी ह\u{948} 5 स\u{947}")
RightParen
Semicolon
RightBrace
EndOfInput

 --------- Statements ---------
VariableDeclaration("स\u{902}ख\u{94d}या", Number(10))
IfStatement(BinaryOperation(Variable("स\u{902}ख\u{94d}या"), GreaterThan, Number(5)), PrintStatement("स\u{902}ख\u{94d}या बड\u{93c}ी ह\u{948} 5 स\u{947}"))

------------ Output ------------
संख्या बड़ी है 5 से

```