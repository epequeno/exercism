module CollatzConjecture exposing (collatz)

-- from https://exercism.io/tracks/elm/exercises/collatz-conjecture/solutions/86afd7251cc2422092097e0588adb5db


collatz : Int -> Result String Int
collatz start =
    if start <= 0 then
        Err "Only positive numbers are allowed"

    else
        Ok (collazPos start)


collazPos n =
    if n == 1 then
        0

    else
        1
            + (if isEven n then
                collazPos (n // 2)

               else
                collazPos (3 * n + 1)
              )


isEven n =
    modBy 2 n == 0


isOdd n =
    not (isEven n)
