notation:
A:B - subtype, A can be used where B is expected
A = B - A and B are interchangeable
A|B - union, either A or B
A&B - intersection, both A and B
(A -> B) - a function from A to B
@A - a shared reference to A
%A - a unique (aka mutable) reference to A
F(A) - calling F with A as an argument

A && B - A and B
A || B - A or B
A => B - if A then B
A <=> B - A if and only if B

rules:
A = A
A = B <=> B = A
A = B && B = C => A = C
A = B => A : B

A|A = A
A&A = A
A : (A|B)
(A&B) : A
(A:B) && (B:C) => (A:C)

A:B <=> @A : @B
@(A|B) = @A | @B
@(A&B) = @A & @B

%A : %B <=> A = B
%(A|B) : %A | %B

A : B && C : D <=> (B -> C) : (A -> D)
A : B && F : G => F(A) : G(B)
(A -> C) & (B -> C) = ((A|B) -> C)
(A -> C) | (B -> C) = ((A&B) -> C)