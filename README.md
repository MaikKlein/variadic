## Overview

This is just a little fun project and should probably not be used in production. Currently variadics are only implemented up to a dimension of `4`. `Variadic`'s are only pssible because of default types and I consider this implementaion a `hack`.

`Variadic<T>` is the trait that makes variadic arguments possible in stable Rust. `VarArgs1`, `VarArgs2` etc implement `Variadic<T>` which allows the user the call `pop()`.

`pop` will return the first argument inside an `Option` and another `VarArgs(n-1)`. For example

~~~
    let (Some(value), rest: VarArgs2<i32, i32>) = VarArgs3(1, 2, 3).pop();
~~~

## Examples:

A simple sum example implemented with recursion.
~~~
fn sum<Args: Variadic<i32>>(args: Args) -> i32 {
   if let (Some(front), rest) = args.pop() {
       front + sum(rest)
   } else {
       0
   }
}
println!("sum: {}", sum(VarArgs4(1, 2, 3, 4)));
~~~

Here we call `pop` on `VarArgsN<i32...>` and it will return `(Option<i32>, VarArgs(N-1)<i32...>)`. The recursion stops at `VarArgs0` which returns a `(Option<i32>, VarArgs0<i32>)` where `Option<i32>` will always be `None`.

~~~
fn fact<Args: Variadic<i32>>(args: Args) -> i32 {
   if let (Some(front), rest) = args.pop() {
       front * fact(rest)
   } else {
       1
   }
}
println!("fact: {}", fact(VarArgs4(1, 2, 3, 4)));
~~~

It is also possible to use traits with `Variadic`. Here we constrain `T` with `std::fmt::Debug`, then we print out every value that we `pop` off until we reach `VarArgs0`.

~~~
fn debug_print<T, Args: Variadic<T>>(args: Args)
   where T: std::fmt::Debug
{
   if let (Some(front), rest) = args.pop() {
       println!("{:?}", front);
       debug_print(rest);
   }
}

debug_print(VarArgs3(1, 2, 3));
~~~

