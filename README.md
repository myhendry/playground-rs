NEXT: L45

cargo new <name> --bin

- [x] Set Env File in Rust
      In terminal,
      PUBLIC_PATH=$(pwd)/public cargo run

**FAQ**

- [ ] Why use String in server struct but use 'str in request struct
      In summary, use String if you need owned string data (like passing strings to other threads, or building them at runtime), and use &str if you only need a view of a string.

- [x] Difference between using super:: vs crate::

- [ ] Result (Ok, Err) and Option (Some, None) are type definitions

- [x] Use unimplemented!() macro for functions you are not ready to implement yet. However, it cannot be used in production because it will panic

_Request_

- [ ] Result can always use the shortcut operator?

_Method_

- [ ] Why need to parse method?

- [ ] Why need to implement From<MethodError>, From<Utf8Error>, Display and Debug for ParseError?

- [ ] Why use empty struct for MethodError?

- [ ] // unwrap()
      // expect()
      // match()
      // ?
      // unwrap_or()

- [ ] When to use Err(ParseError::InvalidRequest) and when just use ParseError::InvalidRequest

[http://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/first-edition/error-handling.html]

Generally speaking, error handling is divided into two broad categories: exceptions and return values. Rust opts for return values.
the key to ergonomic error handling is reducing the amount of explicit case analysis the programmer has to do while keeping code composable.

Keeping code composable is important, because without that requirement, we could panic whenever we come across something unexpected. (panic causes the current task to unwind, and in most cases, the entire program aborts.)

The Option type is a way to use Rust's type system to express the possibility of absence. Encoding the possibility of absence into the type system is an important concept because it will cause the compiler to force the programmer to handle that absence.

The unwrap method abstracts away the case analysis. This is precisely the thing that makes unwrap ergonomic to use. Unfortunately, that panic! means that unwrap is not composable: it is the bull in the china shop.

Rust has parametric polymorphism, so it is very easy to define a combinator that abstracts this pattern

The Option type has many other combinators defined in the standard library. It is a good idea to skim this list and familiarize yourself with what's available—they can often reduce case analysis for you. Familiarizing yourself with these combinators will pay dividends because many of them are also defined (with similar semantics) for Result, which we will talk about next.

Combinators make using types like Option ergonomic because they reduce explicit case analysis. They are also composable because they permit the caller to handle the possibility of absence in their own way. Methods like unwrap remove choices because they will panic if Option<T> is None.

[https://medium.com/@knoldus/combinators-a-functional-approach-of-error-handling-in-rust-4580fb841cb5]

The Result type is a richer version of Option. Instead of expressing the possibility of absence like Option does, Result expresses the possibility of error. Usually, the error is used to explain why the execution of some computation failed. This is a strictly more general form of Option.
The Result type is a way of representing one of two possible outcomes in a computation. By convention, one outcome is meant to be expected or “Ok” while the other outcome is meant to be unexpected or “Err”.

Just like Option, the Result type also has an unwrap method defined in the standard library.

This is effectively the same as our definition for Option::unwrap, except it includes the error value in the panic! message. This makes debugging easier, but it also requires us to add a Debug constraint on the E type parameter (which represents our error type). Since the vast majority of types should satisfy the Debug constraint, this tends to work out in practice. (Debug on a type simply means that there's a reasonable way to print a human-readable description of values with that type.)

This is rather unsightly, and if this happened inside a library you're using, you might be understandably annoyed. Instead, we should try to handle the error in our function and let the caller decide what to do. This means changing the return type of double_number. But to what? Well, that requires looking at the signature of the parse method in the standard library:

Certainly, it's possible that this could have returned an Option. After all, a string either parses as a number or it doesn't, right? That's certainly a reasonable way to go, but the implementation internally distinguishes why the string didn't parse as an integer. (Whether it's an empty string, an invalid digit, too big or too small.) Therefore, using a Result makes sense because we want to provide more information than simply “absence.” We want to say why the parsing failed. You should try to emulate this line of reasoning when faced with a choice between Option and Result. If you can provide detailed error information, then you probably should.

pretty hard line against calling methods like unwrap that could panic and abort your program.

However, unwrap can still be used judiciously. What exactly justifies use of unwrap is somewhat of a grey area and reasonable people can disagree. I'll summarize some of my opinions on the matter.

In examples and quick 'n' dirty code. Sometimes you're writing examples or a quick program, and error handling simply isn't important. Beating the convenience of unwrap can be hard in such scenarios, so it is very appealing.
When panicking indicates a bug in the program. When the invariants of your code should prevent a certain case from happening (like, say, popping from an empty stack), then panicking can be permissible. This is because it exposes a bug in your program. This can be explicit, like from an assert! failing, or it could be because your index into an array was out of bounds.

expect does exactly the same thing as unwrap, except it prints a message you give to expect. This makes the resulting panic a bit nicer to deal with, since it will show your message instead of “called unwrap on a None value.”
