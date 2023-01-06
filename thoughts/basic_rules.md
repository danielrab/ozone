## types
- int and bool are basic types

## permissions
- there are 3 permissions:
  1. read
  2. write
  3. drop
- that produces 5 meaningful kinds of values
  1. `own T` has all permissions. 
  2. `empty T` has `write` and `drop` permissions, which means you don't have to write to it.
  3. `read T` has just `read` and as such can't be moved out of.
  4. `write T` has just `write` it must be written to, producing `borrow T`.
  5. `borrow T` has `read` and `write`.
- moving out of a variable of type T requires `read` amd `write`, leaves behind the same type with no `read` unless T is copy, and produces `own T`.
- assigning to a value grants `read`.
- T that has `read` can produce `read T` losing all permissions but `read` for that duration.
- T that has `write` can produce `write T` losing all permissions for the duration.
- T with `read` and `write` can produce `borrow T` losing all permissions for the duration.
- references are always created with the lifetime at most of the scope that the value being referenced is located in.
- in the four cases above the lifetime of the created value is limited to the lifetime of the original value.