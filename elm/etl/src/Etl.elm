module Etl exposing (transform)

import Dict exposing (Dict)


transform : Dict Int (List String) -> Dict String Int
transform input =
    input
        |> Dict.toList
        |> List.map translate
        |> List.concat
        |> Dict.fromList


translate : ( Int, List String ) -> List ( String, Int )
translate ( points, strings ) =
    strings
        |> List.map String.toLower
        |> makePairs points


makePairs : Int -> List String -> List ( String, Int )
makePairs points letters =
    List.map (\l -> Tuple.pair l points) letters
