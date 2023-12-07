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

There's a template for new days, `src/puzzles/template.rs`, as well as a justfile to generate the relevant files for the next day.
To generate the file for e.g. day 4, make sure you have [just](https://github.com/casey/just) installed and use `just new 4`.
This will create `src/puzzles/day4.rs`, populate it with the right code from the template, and add it to the modules.
You're ready to start working on day 4 with no boilerplate to write.

Enjoy your coding challange!
