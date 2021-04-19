module Leap exposing (isLeapYear)


isLeapYear : Int -> Bool
isLeapYear year =
    if isDivisibleBy 4 year then
        not
            (isDivisibleBy 100 year
                && not (isDivisibleBy 400 year)
            )

    else
        False


isDivisibleBy n divisor =
    modBy n divisor == 0
