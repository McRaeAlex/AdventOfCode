main = interact $ func

func :: String -> String
func = show . sum . map readToInt . words

readToInt :: String -> Int
readToInt ('+':number) = read number
readToInt number = read number
