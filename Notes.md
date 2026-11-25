
## Self : The 3 Ways to Use self

Depending on what your method needs to do, you write self in one of three ways:
1. &self (Borrowing / Read-Only): The method can look at the object's data, but it cannot change it. This is the most common way to use it.
2. &mut self (Mutable Borrowing / Read & Write): The method can look at and change the object's data.
3. self (Taking Ownership): The method consumes the object completely. After this runs, the original variable can no longer be used.