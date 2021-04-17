module Raindrops exposing (raindrops)


raindrops : Int -> String
raindrops number =
    if not (List.any (isDivisible number) [ 3, 5, 7 ]) then
        String.fromInt number

    else
        List.map (makeString number) [ 3, 5, 7 ] |> String.join ""


makeString : Int -> Int -> String
makeString n divisor =
    if isDivisible n divisor then
        case divisor of
            3 ->
                "Pling"

            5 ->
                "Plang"

            7 ->
                "Plong"

            _ ->
                ""

    else
        ""


isDivisible : Int -> Int -> Bool
isDivisible x y =
    modBy y x == 0
