module ArmstrongNumbers exposing (isArmstrongNumber)


isArmstrongNumber : Int -> Bool
isArmstrongNumber nb =
    let
        len =
            nb
                |> String.fromInt
                |> String.length
    in
    nb
        |> String.fromInt
        |> String.toList
        |> List.map String.fromChar
        |> List.map String.toInt
        |> List.map (Maybe.withDefault 0)
        |> List.map (\x -> x ^ len)
        |> List.sum
        |> (==) nb
