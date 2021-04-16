module Leap exposing (isLeapYear)


isLeapYear : Int -> Bool
isLeapYear year =
    if isDivisibleBy4 year then
        if
            isDivisiblyBy100 year
                && not (isDivisibleBy400 year)
        then
            False

        else
            True

    else
        False


isDivisibleBy4 n =
    modBy 4 n == 0


isDivisiblyBy100 n =
    modBy 100 n == 0


isDivisibleBy400 n =
    modBy 400 n == 0
