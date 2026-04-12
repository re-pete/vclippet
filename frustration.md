# Why Rust Why

Rust doesn't know how to concatenate strings. It always destroys the first one. You can pick the order, but it always destroys the first one. 
For no reason. And not only that, it **CAN'T** destroy the second one.  That one **must** survive. So you can't even add two strings, keep the third,
and let the first two go. 
```rust
    let s1 = String::from("a");
    let s2 = String::from("b");
    let s3 = s1 + s2;   // Error
    let s3 = s2 + s1;   // Error
    let s3 = &s1 + s2;  // Error
    let s3 = &s1 + &s2; // Error
    let s3 = s1 + &s2;  // Ok
    let s3 = s2 + &s1;  // Ok
```

---

Rust puts the function return type at the end, so it scrolls off and you can't tell what your functions do
```rust
    pub fn my_really_long_function_yeah(argument_one: Type_One_Two, argument_two: Type_Two_Three, argument_three: Type_Three_Four) -> Finally_A_Return_Type {
        // Good luck reading that return type without a scroll bar
    }
```

---

Rust forces you to put & everywhere, even though it should be able to infer you're borrowing by the function signature, but then
it does stuff like this:
```rust
    let my_val = some_function().returns_who_knows_what();
```
What type is ```my_val```? Who knows! So weird that it forces you to spell things out elsewhere, then completely hides types in basic variable declaration. 

---

Almost every macro I've seen feels like it should be native to the language. vec! is pointing out a flaw in not being able to declare vectors
with normal language features. Obviously the language should have had a println function. Variadic arguments in macros shows how useful they are
but they aren't allowed in the language. Having to apply a derive debug macro to structs just so they're printable. format! shows how useful
string concatenation is. 

---

Rust doesn't even have a normal for loop. I'm not knocking the current one - it's nice to have iterators natively supported by
for loop syntax. But why don't they support normal for loops yet? I looked it up: It took them 6 years to implement step_by(). And that only alows
incrementing by non-1 values. 
```rust
    for i in 0..100.step_by(2) { }
```
Yet somehow they added ```loop```.  Was ```while(true)``` too difficult?
