(* Test file for Wolfram Language syntax highlighting and LSP tokens *)

(* Built-in functions *)

Print["Hello, World!"]

Plot[Sin[x], {x, 0, 2 Pi}]

Solve[x^2 + 3 x + 2 == 0, x]

(* Symbols and patterns *)

mySymbol = 42

Print["x"];

Module[{localVar = 42},
    localVar^2 + 5
]

parameter_Integer :=
    parameter + 1

pattern_?NumberQ :=
    pattern + 1

(* Mathematical expressions *)

result = Integrate[x^2 + Sin[x], x]

derivative = D[x^3 + Cos[x], x]

(* Lists and associations *)

myList = {1, 2, 3, "string", True}

myAssoc = <|"key1" -> "value1", "key2" -> 42|>

(* Function definitions *)

factorial[n_Integer] :=
    If[n <= 1,
        1
        ,
        n * factorial[n - 1]
    ]

fibonacci[n_] :=
    Fibonacci[n]

(* Pure functions *)

plusOne = # + 1&

mapFunction = Map[#^2&, {1, 2, 3, 4}]

(* Rules and replacements *)

expr = x + y + z

newExpr = expr /. x -> 5

(* String manipulation *)

stringVar = "This is a test string"

StringLength[stringVar]

StringReplace[stringVar, "test" -> "sample"]

(* Conditionals and loops *)

If[True,
    Print["True branch"]
    ,
    Print["False branch"]
]

Do[Print[i], {i, 1, 5}]

While[x < 10, x++]

(* Module and local variables *)

Module[{localVar = 10},
    localVar^2 + 5
]

(* Context and package notation *)

Begin["MyContext`"]

MyContext`myFunction[x_] :=
    x^2

End[]

(* Options and settings *)

SetOptions[Plot, PlotStyle -> Red, PlotRange -> All]

Options[Plot]

(* Graphics and visualization *)

Graphics[Circle[{0, 0}, 1]]

ListPlot[Table[{i, i^2}, {i, 1, 10}]]
