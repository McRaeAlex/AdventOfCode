{-
    based on reddit comments saying it can be done in one pass
-}

module Main (main) where 
import Data.Char

main = interact $ func . head . words
    
func :: String -> String
func = show . length . foldr step ""
    where step x (y:ys) | y /= x && toUpper y == toUpper x = ys
          step x ys                                        = x : ys
