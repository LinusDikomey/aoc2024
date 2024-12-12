import Data.List.Split
import Data.Function

main = do
  file <- readFile "input/day7.txt"
  let xs = lines file & map parseLine
  print $ filter (\(t, xs) -> t `elem` results [(+), (*)] xs) xs & map fst & sum
  print $ filter (\(t, xs) -> t `elem` results [(+), (*), concatInts] xs) xs & map fst & sum
  return 0

parseLine s = case splitOn ": " s of
  [a,b] -> (read a, words a & map read)
  [] -> error "invalid line"

concatInts a b = read $ show a ++ show b

perms n m = [iterate (`div` n) x & map (`mod` n) & take m | x <- [0 .. (n ^ m) - 1]]

calc (x : y : xs) (f: fns) = calc (f x y : xs) fns
calc [x] [] = x
calc a b = error "calc missing"

results fns xs = perms (length fns) (length xs - 1) & map (\p -> map (fns !!) p & calc xs)
