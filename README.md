NEXT: L43

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
