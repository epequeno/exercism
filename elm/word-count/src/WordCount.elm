module WordCount exposing (wordCount)

import Dict exposing (Dict)


wordCount : String -> Dict String Int
wordCount sentence =
    sentence
        |> String.toLower
        |> String.replace "," " "
        |> String.words
        |> List.map (\w -> clean w)
        |> List.filter (\w -> w /= "")
        |> List.map (\w -> Tuple.pair w (String.indices w sentence |> List.length))
        |> Dict.fromList


clean word =
    if String.contains "'" word then
        word

    else
        String.filter (\c -> Char.isAlpha c) word
