f :: Bool -> Bool
f x = undefined -- equivalent to f = undefined
-- this allows functions to return the bottom explicitly

fact n = product [1..n] -- factorial function is amenable to denotional semantics

absurd :: Void -> a -- function that is polymorphic in the return type (but can't be called)
-- this function can't be called because it takes Void (which is a type not inhabited by any values)

-- () is the type (called "unit") that corresponds to the singleton set
-- it's a type that has only one possible value
f44 :: () -> Integer -- declares that f44 takes "unit" into the type Integer
f44 () = 44 -- pattern matching the only constructor for unit `()` and producing 44
-- call this ia `f44 ()` and it'll always return 44

fInt :: Integer -> ()
fInt _ = () -- the wildcard pattern `_` represents an argument that is discarded.
-- give the above function any integer and it gives you back a unit

unit :: a -> () 
unit _ = ()
-- this is a polymorphic function from any type to unit type

-- Bool type definition in Haskell
data Bool = True | False