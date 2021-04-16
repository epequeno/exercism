module SumOfMultiples exposing (sumOfMultiples)

import Set


sumOfMultiples : List Int -> Int -> Int
sumOfMultiples divisors limit =
    divisors
        |> List.map (findDivisible limit)
        |> List.concat
        |> Set.fromList
        |> Set.toList
        |> List.sum


findDivisible limit divisor =
    List.range 1 (limit - 1)
        |> List.filter (\x -> modBy divisor x == 0)
