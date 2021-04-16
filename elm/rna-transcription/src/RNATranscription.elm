module RNATranscription exposing (toRNA)


toRNA : String -> Result String String
toRNA dna =
    let
        translated =
            dna
                |> String.toList
                |> List.map translateChar
    in
    if List.any isError translated then
        Err "Invalid input."

    else
        Ok (String.fromList <| List.map unwrap translated)


unwrap : Result error Char -> Char
unwrap res =
    case res of
        Ok c ->
            c

        Err _ ->
            ' '


isError : Result error value -> Bool
isError res =
    case res of
        Ok _ ->
            False

        Err _ ->
            True


translateChar : Char -> Result String Char
translateChar c =
    case c of
        'C' ->
            Ok 'G'

        'G' ->
            Ok 'C'

        'T' ->
            Ok 'A'

        'A' ->
            Ok 'U'

        _ ->
            Err "cannot translate"
