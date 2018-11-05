
f :: A -> B
g :: B -> C
g . f

h :: C -> D
-- pseudo code for associativity because equality isn't defined for functions:
-- h . (g . f) == (h . g) . f == h . g . f

-- the identity function is part of the standard library (Prelude)
-- id :: a -> a     -- id x = x