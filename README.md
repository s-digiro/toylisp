# To Do
[ ] pass all args to macro
[ ] Pass env to macro
[ ] Move RustMacro outside of weird meta module.
[ ] Add line and column to errors where applicable
[ ] (quote (1 2)) behaves differently from ('(1 2)) and it shouldn't
[ ] Passing lambdas to other lambdas is a lil weird... figure it out
[ ] McCarthy things need to be implemented
    [*] Car
    [*] Cons
    [*] Cdr
    [*] Equal
    [*] quote
    [ ] Maybe more
    [*] McCarthy context??? as a type of Rust context?
    [ ] Make sure all McCarthy things are added
[ ] Lib user defined Rust Contexts
[ ] Rust context of lib that I find useful
    [ ] Like print
    [ ] Add/Subtract
    [ ] Stuff that isn't pure McCarthy
[ ] . infix operator
[ ] Garbage collector
[ ] Add tests
    [ ] evaluate
        [ ] environment
            [ ] mccarthy_scope
            [ ] mod
        [ ] mod
    [ ] s_expression
        [*] cons_cell
            [*] iter
            [*] mod
        [ ] meta
            [ ] lambda
            [ ] macro
            [ ] mod
            [ ] rust_lambda
            [ ] rust_macro
        [ ] atom
        [ ] list
        [ ] mod
    [*] parse
    [ ] lib
[ ] Lisp macro should only recieve one arg, which is arg list
[ ] Lisp lambda should get args like normal
[ ] Evaluate List should be refactored into more functions so its easier to read
[ ] Allow multiple lists in root, without tacking on a root list as a new
    function. Maybe as a vector of refvals
[ ] Everything should be implementable in lisp, other than Mccarthy basics
[ ] Maybe strings should be lists of chars? Or maybe permanently quoted symbols?
    Pick one
[ ] List macro
[ ] Label macro
[ ] let macro
[ ] arbitrary cdaddr function
[ ] Maybe mccarthy functions should be "specials", separate from macros?
    decide.
[ ] Invalid function calls should fail spectacularly rather than returning nil
[ ] NIL keyword
[*] Rename lambda to function to make clearer the difference between lambda
    macro and actual data structure
[*] Lambda should be its own value, not a weird wrapper
[*] Try making SExpressionRef a singleton tuple that derefs to Rc
[*] Rename RustLambda to RustFunction for consistency
[*] Move Macro outside of weird macro module
[*] Macro should be its own value, not a weird wrapper
[*] From<SXRef> for Function
