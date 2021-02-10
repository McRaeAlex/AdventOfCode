module Main where

import qualified Data.Sequence as Seq
import Data.List
import Data.Monoid
import Data.Digits (digits)

main :: IO ()
main = do
    putStrLn "Part 1:"
    putStrLn "Tests"
    putStrLn . intercalate "" $ map show $ part1 9
    putStrLn "5158916779"
    putStrLn . intercalate "" $ map show $ part1 5
    putStrLn "0124515891"
    putStrLn . intercalate "" $ map show $ part1 18
    putStrLn "9251071085"
    putStrLn . intercalate "" $ map show $ part1 2018
    putStrLn "5941429882"
    putStrLn "Output"
    putStrLn . intercalate "" $ map show $ part1 824501
    putStrLn "\nPart 2:"
    putStrLn "Tests"
    putStrLn . show $ part2 51589
    putStrLn "9"
    putStrLn . show $ part2 01245
    putStrLn "5"
    putStrLn . show $ part2 92510
    putStrLn "18"
    putStrLn . show $ part2 59414
    putStrLn "2018"
    putStrLn "Output"
    putStrLn . show $ part2 824501


-- the issue i am running into is that we cannot see the start of the list we create
-- on any recursive call but the first basically meaning that we cannot do it this way
-- unless there is a way to pass both but then i cannot do it with infinite lists
recipe' y x xs =
        newDigs ++ recipe' recipe1 recipe2 newSeq
        where
            score1 = xs `Seq.index` y
            score2 = xs `Seq.index` x
            newDigs = digits' (score1 + score2)
            newSeq = xs <> Seq.fromList newDigs
            recipe1 = ((y + score1 + 1) `mod` (Seq.length newSeq))
            recipe2 = ((x + score2 + 1) `mod` (Seq.length newSeq))

-- turns out the digits function haskell has does not consider 0 a digit...
-- so i wrote my own which works because the max sum is 18 which can only have 2 digits
digits' :: Int -> [Int]
digits' n
    | n == 0 = [0]
    | n < 10 = [n]
    | otherwise = [x, y]
        where (x, y) = divMod n 10

part1 n = take 10 (drop n list)
        where
            list =  3 : 7 : recipe' 0 1 sq
            sq = Seq.fromList [3, 7]

part2 n = length . takeWhile (not . ((digits 10 n) `isPrefixOf`)) . tails $ list
        where 
            list =  3 : 7 : recipe' 0 1 sq
            sq = Seq.fromList [3, 7]


