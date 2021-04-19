module Leap exposing (isLeapYear)


isLeapYear : Int -> Bool
isLeapYear year =
    isDivisibleBy 4 year
        && not
            (isDivisibleBy 100 year
                && not (isDivisibleBy 400 year)
            )


isDivisibleBy divisor n =
    modBy divisor n == 0
