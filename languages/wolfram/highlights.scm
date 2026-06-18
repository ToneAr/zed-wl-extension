(call
  head: (symbol) @function)

(pattern
  name: (symbol) @variable.parameter)

(symbol) @variable

(integer) @number
(real) @number

(string) @string
(comment) @comment

[
  "("
  ")"
  "["
  "]"
  "[["
  "]]"
  "{"
  "}"
  "<|"
  "|>"
] @punctuation.bracket

[
  ","
  ";"
  ";;"
] @punctuation.delimiter

[
  (blank)
  (blank_default)
  (blank_sequence)
  (blank_null_sequence)
] @tag

[
  "!"
  "!!"
  "!="
  "&"
  "&&"
  "'"
  "*"
  "**"
  "*="
  "+"
  "++"
  "+="
  "-"
  "--"
  "-="
  "->"
  "."
  ".."
  "..."
  "/"
  "/*"
  "/."
  "//"
  "//."
  "//="
  "//@"
  "/:"
  "/;"
  "/="
  "/@"
  ":"
  ":="
  ":>"
  "<"
  "<->"
  "<="
  "<>"
  "="
  "=."
  "=="
  "==="
  "=!="
  ">"
  ">="
  "?"
  "@"
  "@@"
  "@@@"
  "@*"
  "^"
  "^:="
  "^="
  "|"
  "|->"
  "||"
  "~~"
] @operator
