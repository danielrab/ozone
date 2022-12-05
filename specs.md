## design philosophy
- things should have only a single way to read them
- everything should be composed of simple building blocks with concrete meaning
- if it looks the same it means the same


## basic meaning of things
- `:` means "is":
  - `x: String` means "x is a String"
  - `Type: Trait` means "Type implements Trait"
  - `Trait1: Trait2` means "Trait1 requires Trait2"


- `=` means assignment
  - `a = 1` means "set a to 1"
  - `{x=1}` means "an object with x set to 1"

- `$` means "declare identifier"
  - `$a = 1` declares a variable called a and assigns to it
  - `$T` in `(x: $T y: $T) -> {...}` declares a generic type parameter
  - `$Point {x: i32, y: i32}` creates a wrapper type Point around `{x: i32, y: i32}`
  - `$Point $T {x: T, y: T}` creates a generic Point, which can be specified with `Point[T: i32]` (Point such that T is constrained to i32), for example: `$p: Point[T: i32] = Point[T: i32] {x=1, y=1}` is a full expansion of `$p = Point {x=1, y=1}`



## types
- known: a variable with a known value, for example the type of `a` in `let a = 5;` is "5"
- range: a number that is between two other numbers, for example `0..100`
- record: things like `{x: 0..100, y: 0..100}`
- tuple: `(T, U, V)` is syntax sugar for `{0: T, 1: U, 2: V}`
- joined types: `{x: T, y: U} & {y: U, z: V}` = `{x: T, y: U, z: V}`
- union types: value of type `T|U` can be either `T` or `U`
- values of union types are implicitly tagged allowing to use match statements to get the exact variations

## operators and math
- default operators:
  - `+` addition
  - `-` subtraction
  - `*` multiplication
  - `/` division
  -  dereference
  -  shared reference
  -  unique reference

- operators must guarantee non-panicking
- operators bind more strongly than functions
- default integer type is unbound
- division of integers returns fractions
- .function is an operator that applies said function

- #### custom operators
  - each operator has a single corresponding named function or function-like macro.
  - a single function can't have two operators reference it in the same scope.
  - you can export the operator, but the function must then be exported too.
  - when you write `use module::function;` you are importing just the function.
  - you can write `use module::function with <operator>;` where `<operator>` must match what is defined by the module (and will probably be suggested and/or auto-filled by the IDE).
  - you can write `<operator> is function;` to bind a new operator to a function for which an operator currently doesn't exist in scope.

## macros
- macros are expanded at definition site (maybe provide an escape hatch for local)
- operator definitions aren't used inside macros (this allows to reserve operators to mean specific things in a macro)
- simple macros should look almost like a function

## code maybe
```
0 -> MIN
100 -> MAX
loop (
  print!("input a number between {} and {}" MIN MAX end=" ")
  std::io::stdin.parse Int.match! (
    Ok($n: 0=:=100) => break n;
    Ok(:<0) => print! "the number can't be negative";
    Ok(100<:) => print! "the number can't be larger than 100";
    Err => print! "{self} is not a number";
  )
) -> $number
print! "you chose {number}, hope you chose wisely"
```