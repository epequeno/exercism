module NucleotideCount exposing (nucleotideCounts)

import Task exposing (sequence)


type alias NucleotideCounts =
    { a : Int
    , t : Int
    , c : Int
    , g : Int
    }


nucleotideCounts : String -> NucleotideCounts
nucleotideCounts sequence =
    let
        chars =
            sequence |> String.toList
    in
    { a = count 'A' chars
    , t = count 'T' chars
    , g = count 'G' chars
    , c = count 'C' chars
    }


count : Char -> List Char -> Int
count char chars =
    List.map
        (\c ->
            if char == c then
                1

            else
                0
        )
        chars
        |> List.sum
