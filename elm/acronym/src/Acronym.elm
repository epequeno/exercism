module Acronym exposing (abbreviate)


abbreviate : String -> String
abbreviate phrase =
    phrase
        |> String.replace "-" " "
        |> String.toUpper
        |> String.words
        |> List.map (String.left 1)
        |> String.join ""
