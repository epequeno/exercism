module Isogram exposing (isIsogram)


isIsogram : String -> Bool
isIsogram sentence =
    case sentence of
        "" ->
            True

        _ ->
            sentence
                |> String.replace "-" ""
                |> String.replace " " ""
                |> String.toLower
                |> String.toList
                |> List.sort
                |> anyRepeated
                |> not


anyRepeated : List Char -> Bool
anyRepeated xs =
    case xs of
        [ a, b ] ->
            a == b

        a :: b :: rest ->
            a == b || anyRepeated (b :: rest)

        _ ->
            False
