module Series exposing (slices)


slices : Int -> String -> Result String (List (List Int))
slices size input =
    if input == "" then
        Err "series cannot be empty"

    else if size == 0 then
        Err "slice length cannot be zero"

    else if size < 0 then
        Err "slice length cannot be negative"

    else if size > String.length input then
        Err "slice length cannot be greater than series length"

    else
        Ok (getSlices size input)


getSlices : Int -> String -> List (List Int)
getSlices size input =
    List.range 0 (String.length input)
        |> List.map (\i -> String.slice i (i + size) input |> convertChunk)
        |> List.filter (\l -> not (List.isEmpty l))
        |> List.filter (\l -> List.length l >= size)


convertChunk : String -> List Int
convertChunk input =
    input
        |> String.toList
        |> List.map String.fromChar
        |> List.map String.toInt
        |> List.map (Maybe.withDefault 0)
