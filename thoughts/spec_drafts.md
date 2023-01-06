## design philosophy
- keep the simple things simple
- fundamental ease of use is more important than familiarity.
- the language should provide you with all possible support in writing a correct program.
- verification shouldn't get in the way of prototyping.
- if it looks like you can do it, you should be able to do it, unless there's a good reason not to allow it.
- if you need it, you should be able to write fast code without resorting to FFI.
- FFI must be easy in order to allow using the vast amounts of existing code.


## goals
- be easier to use than Rust.
- have all the safety guarantees of Rust.
- avoid all panicking behavior, including overflows, division by zero, and out of bounds access, by having a compile-time check with runtime fallback available.
- eliminate, when possible, unnecessary runtime checks by having the compiler aware of the possible values of variables.
- make stack overflow impossible by producing an estimate of the maximum stack size.
- decrease the likelihood of unwanted infinite loops by having potentially diverging functions be clearly marked, and having ways of proving termination.
- have an effects system to clearly indicate which functions are pure and easily swap the handlers for the effects.


## code sample
```
use rand.Rng;
use std.cmp.Ordering;
use std.io;

print("Guess the number!");

$secret_number = rand.thread_rng.gen_range(1=:=100);

loop (
    print("Please input your guess.");
    
    $guess = io.stdin.read_line.parse.(
        | Ok($num) -> num
        | Err(_) -> (continue loop)
    );
    
    print(f!"You guessed: {guess}");
    
    guess.cmp(&secret_number).(
        | Ordering.Less -> print("Too small!")
        | Ordering.Greater -> print("Too big!")
        | Ordering.Equal -> (break loop)
    );
)

print("You win!")

```