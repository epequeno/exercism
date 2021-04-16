module Bob exposing (hey)


hey : String -> String
hey remark =
    let
        r =
            String.trim remark
    in
    if isQuestion r && isAllCaps r then
        "Calm down, I know what I'm doing!"

    else if isQuestion r then
        "Sure."

    else if isAllCaps r then
        "Whoa, chill out!"

    else if "" == r then
        "Fine. Be that way!"

    else
        "Whatever."


isQuestion w =
    String.endsWith "?" w


isAllCaps w =
    let
        chars =
            w
                |> String.toList
                |> List.filter Char.isAlpha
    in
    if List.isEmpty chars then
        -- Because List.all [] == True we have to
        -- say an empty list is not all caps ourselves.
        False

    else
        chars |> List.all Char.isUpper
