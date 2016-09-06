module FizzBuzz where

-- |
isFizz :: Integral a => a -> Bool
isFizz n = n `mod` 3 == 0

-- |
isBuzz :: Integral a => a -> Bool
isBuzz n = n `mod` 5 == 0

-- |
isFizzBuzz :: Integral a => a -> Bool
isFizzBuzz n = isFizz n && isBuzz n

-- |
fizzBuzz :: (Integral a, Show a) => a -> [Char]
fizzBuzz n
    | isFizzBuzz n = "FizzBuzz"
    | isFizz n     = "Fizz"
    | isBuzz n     = "Buzz"
    | otherwise    = show n

-- |
fizzBuzzXs :: (Integral a, Show a) => [a] -> [[Char]]
fizzBuzzXs = map (fizzBuzz)
