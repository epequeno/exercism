module Triangle exposing (Triangle(..), triangleKind)

import Set


type Triangle
    = Equilateral
    | Isosceles
    | Scalene


triangleKind : number -> number -> number -> Result String Triangle
triangleKind x y z =
    let
        candidate =
            [ x, y, z ] |> List.sort
    in
    if not (isValidLengths candidate) then
        Err "Invalid lengths"

    else if not (isValid candidate) then
        Err "Violates inequality"

    else if isEquilateral candidate then
        Ok Equilateral

    else if isIsosceles candidate then
        Ok Isosceles

    else if isScalene candidate then
        Ok Scalene

    else
        Err "invalid input"


isValid : List number -> Bool
isValid sides =
    case sides of
        [ a, b, c ] ->
            (a + b) >= c

        _ ->
            False


isValidLengths : List number -> Bool
isValidLengths sides =
    not (isZeros sides) && not (anyNegative sides)


isZeros : List number -> Bool
isZeros =
    List.sum
        >> (==) 0


anyNegative : List number -> Bool
anyNegative =
    List.any (\n -> n < 0)


setSize : List number -> Int
setSize =
    Set.fromList
        >> Set.size


isEquilateral : List number -> Bool
isEquilateral =
    setSize
        >> (==) 1


isIsosceles : List number -> Bool
isIsosceles =
    setSize
        >> (>=) 2


isScalene : List number -> Bool
isScalene =
    setSize
        >> (==) 3
