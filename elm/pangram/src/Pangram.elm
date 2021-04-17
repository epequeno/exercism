module Pangram exposing (isPangram)

import Set


isPangram : String -> Bool
isPangram sentence =
    sentence
        |> String.toLower
        |> String.toList
        |> List.filter Char.isLower
        |> Set.fromList
        |> Set.toList
        |> List.length
        |> (==) 26
