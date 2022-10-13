use lexer::Dictionary;

/* Parser
Program: Pipe
       | Pipe &
Pipe: IO
    | IO | IO
IO: CMD
  | CMD > Text
  | CMD >> Text
  | CMD < Text
CMD: Exec
   | echo
   | [etc.]

Each category in CMD:
= [keyword]
| [cmd] text



*/

struct AST {
    tokens: vec<Dictionary>,

}