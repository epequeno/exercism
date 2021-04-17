module Hamming exposing (distance)


distance : String -> String -> Result String Int
distance left right =
    if String.length left /= String.length right then
        Err "left and right strands must be of equal length"

    else
        Ok
            (List.map2 areDifferent
                (left |> String.toList)
                (right |> String.toList)
                |> List.sum
            )


areDifferent : Char -> Char -> Int
areDifferent l r =
    if l /= r then
        1

    else
        0
