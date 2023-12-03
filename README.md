# Advent of Code 2023

**In Rust :crab: !**

To get your set of inputs, you'll need to create a file called `cookie.toml` in the root of the repo, and add a single key
`cookie` with the string value of your session cookie. You can get that from the developper tools, once you're logged in.


My own inputs are in separate files in `inputs/day_{day}.txt`. Your own test inputs may differ, so if you want to use
your own inputs, delete these files and regenerate them by running the tests (after settting your session cookie
as described above).

My solutions are in `src/puzzles/day{day}.rs`. Each one has tests for the different parts and examples, with asserts 
marking the correct test values. If you'd like to run this yourself, it's nice to see the output as you go, so I use 
the `k9::snapshot!` macro to generate the test results while i'm writing up the tests, and switch it over to an assert
when I've submitted my solution and confirmed that it's correct.

Enjoy your coding challange!
