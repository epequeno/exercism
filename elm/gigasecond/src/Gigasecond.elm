module Gigasecond exposing (add)

import Time


add : Time.Posix -> Time.Posix
add timestamp =
    Time.posixToMillis timestamp
        + (10 ^ 9)
        * 1000
        |> Time.millisToPosix
