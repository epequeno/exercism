module Anagram exposing (detect)


detect : String -> List String -> List String
detect word candidates =
    candidates
        |> List.filter
            (\c ->
                (sorted c == sorted word)
                    && (String.toLower c /= String.toLower word)
            )


sorted : String -> String
sorted =
    String.toLower
        >> String.toList
        >> List.sort
        >> String.fromList
