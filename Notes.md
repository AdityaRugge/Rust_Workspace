
## Self : The 3 Ways to Use self

Depending on what your method needs to do, you write self in one of three ways:
1. &self (Borrowing / Read-Only): The method can look at the object's data, but it cannot change it. This is the most common way to use it.
2. &mut self (Mutable Borrowing / Read & Write): The method can look at and change the object's data.
3. self (Taking Ownership): The method consumes the object completely. After this runs, the original variable can no longer be used.

# Core Rules of Rust Ownership, Borrowing, and Lifetimes

Rust manages memory safely without a garbage collector by enforcing strict ownership and borrowing rules at compile time.

## 1. Ownership & Moving
* **Single Owner:** Every value has a single owner at any given time (such as a variable, function argument, struct field, or vector element).
* **Move Semantics:** Reassigning a value, passing it to a function, or inserting it into a collection transfers ("moves") ownership. Once moved, the previous owner can no longer be used to access the value.
* **Copy Types:** Simple stack-allocated types—such as integers, floating-point numbers, booleans, characters, and tuples or fixed-size arrays containing only copyable elements—implement the `Copy` trait and are duplicated rather than moved.
* **Scope & Dropping:** When an owner goes out of scope, Rust automatically drops the value and frees its memory.

## 2. Borrowing & References
* **Multiple Immutable References:** You can create any number of read-only (`&T`) references to a value at the same time.
* **Single Mutable Reference:** You can create only one writable (`&mut T`) reference to a value at a time, and only when no immutable references are currently active.
* **No Moving While Borrowed:** You cannot move a value while any reference (mutable or immutable) to it still exists.
* **No Owner Mutation While Borrowed:** You cannot modify a value directly through its owner while any reference to it is active.

## 3. Lifetimes & Safety
* **Valid References Only:** No active references may exist when a value's owner goes out of scope.
* **No Dangling References:** A reference can never outlive the value it points to.

## Key Takeaways
* **A Different Mental Model:** Enforcing these rules at compile time changes how you structure code compared to garbage-collected or manually managed languages.
* **Predictable Data Flow:** Whenever you hit a compiler error, remember Rust's core goal—preventing data races and unexpected mutations by making data access explicit.

# Rust Function Argument Types: Choosing Ownership vs. Borrowing

When designing a function signature in Rust, choose how to accept an argument based on what the function needs to do with the data.

## 1. Need to store the argument somewhere?
* **Recommendation:** Favor taking **ownership** (receive a value, e.g., `param: T`).
* **Why:** If the function needs to keep the data alive inside a struct, vector, or another thread, taking ownership transfers control cleanly without requiring extra clones inside the function.
* **Example:** `fn save_user(user: User)`

## 2. Need to read or calculate with the value?
* **Recommendation:** Favor receiving a **read-only reference** (immutable borrow, e.g., `param: &T`).
* **Why:** Borrowing lets the function inspect or compute using the data without taking ownership, so the caller can continue using the variable afterward.
* **Example:** `fn calculate_total(items: &Vec<Item>) -> u32`

## 3. Need to modify or change the value?
* **Recommendation:** Favor receiving a **mutable reference** (mutable borrow, e.g., `param: &mut T`).
* **Why:** A mutable borrow allows the function to update the data in place without needing to return the modified value back to the caller.
* **Example:** `fn update_task_id(task: &mut Task)`

---

## Quick Decision Table

| Goal inside the function | Argument Type | Syntax | Caller keeps ownership? |
| :--- | :--- | :--- | :--- |
| **Store or consume** the data | Owned value | `T` | No (moved) |
| **Read or inspect** the data | Immutable reference | `&T` | Yes |
| **Modify or update** the data | Mutable reference | `&mut T` | Yes |

# Enums vs. Structs

Deciding when to use enums vs. structs is tricky. In many cases, you can use either.

## Decision Questions

- **Does each thing you're modeling have the same methods?**
  - **You might want to use structs.**
- **Does each thing have some same, but some different methods?**
  - **You might want to use an enum.**

# Rust `Option<T>` Enum

Rust does not have `null`, `nil`, or `undefined`. Instead, it provides the built-in `Option<T>` enum to represent a value that may or may not exist.

## Variants

- **`Some(value)`**: Contains a wrapped value of type `T`.
- **`None`**: Represents the absence of a value.

## Working with `Option`

Because `Option` is an enum rather than a nullable reference, you cannot use the inner value directly without unpacking it safely:

- **`match` statement**: Exhaustively handles both `Some(value)` and `None`. The compiler enforces that every possible case is covered.
- **`if let`**: A concise way to handle only one variant (such as `Some(value)`) while ignoring the other.

```rust
enum Option<T> {
    Some(T),
    None,
}

fn describe_number(num: Option<i32>) {
    match num {
        Some(value) => println!("Got a value: {value}"),
        None => println!("No value present"),
    }

    if let Some(value) = num {
        println!("Matched with if let: {value}");
    }
}