module PhoneNumber exposing (getNumber)


getNumber : String -> Maybe String
getNumber phoneNumber =
    let
        number =
            phoneNumber |> String.filter Char.isDigit
    in
    case String.length number of
        11 ->
            if String.startsWith "1" number then
                check (String.dropLeft 1 number)

            else
                Nothing

        10 ->
            check number

        _ ->
            Nothing


check : String -> Maybe String
check number =
    let
        ( a, b ) =
            ( getIx 0 number, getIx 3 number )
    in
    if (a < 2) || (b < 2) then
        Nothing

    else
        Just number


getIx : Int -> String -> Int
getIx ix s =
    s
        |> String.slice ix (ix + 1)
        |> String.toInt
        |> Maybe.withDefault 0
