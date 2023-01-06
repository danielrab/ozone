Problem:
NewType pattern often requires to reimplement a bunch of traits and functions, field access also kinda sucks.

Solution:
Either opt-in or opt-out (TBD) delegation of all fields and of all the methods that weren't overridden.
In addition to that, syntax to explicitly delegate and maybe also explicitly hide fields and methods.
Arguments of type Self get automatically unwrapped, and returns of type Self get automatically wrapped.


Problem:
Being able to require the presence of some fields is convenient, but implementation details should be hidden.

Solution:
Getters and setters should act indistinguishably from fields.
Notably, this means that the following properties must be upheld:
- the parts of the structure borrowed by getters/setters must never intersect with other fields.
- getters must be pure functions.
- setters must not have effect on anything other than the object they belong to.
- having a getter and a setter must allow you to produce a mutable borrow.
- using a setter must not have effects on other fields or getters.
- using a setter and then a getter for the same field without anything in between should give the value that was set.


Problem:
Functions often borrow the entire object when they only need some fields from it.

Solution:
Each field of an object can be referred to in lifetimes and this can be done deeply.
This is also required for getters/setters to work properly.


Problem:
Moving out of a reference or even an owned object isn't represented properly.

Solution:
Owed and empty types, in addition to owned, borrowed, and read-only.
This also allows for construction in place and empty boxes.


Problem:
Moving an object can't be done by reference.

Solution:
Owned references.
Notably their lifetime is still bound by the storage.


Problem:
To borrow you need to introduce a layer of indirection.

Solution:
Borrows by value.
When a mutable borrow by value is dropped, it's merged into the object it's borrowed from.
Tracking of lifetimes is done on permissions rather than references.
Types with interior mutability can't be borrowed by value.
Borrows by value with write access can't be saved into collections or conditionally swapped with other borrows in any other way, that is you must be able to tell at compile time which object was borrowed from.


Problem: Overflow
Solution: BigInt by default


Problem: floating point is imprecise
Solution: fractions by default


Problem:
out of bounds access and division by zero panic, there are also other things that a function might require.

Solution:
Functions with prerequisites and promises, where prerequisites are checked at compile time by default, but can be moved to runtime with the an operator.