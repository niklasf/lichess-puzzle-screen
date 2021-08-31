Puzzle screening tools
======================

`make-rocksdb`
--------------

Prepare an offline database to lookup game moves by id from
https://database.lichess.org.

`add-game-moves`
----------------

Adds game moves from the database as an additional column to the puzzle CSV.

`repetitions`
-------------

Find repetitions in the final phase before the puzzle position.

`implied-castling-rights`
-------------------------

Find puzzle positions that look like they have more castling rights than they
do.

`narrow`
--------

Check if the potentially puzzle-breaking criterea above really impact the
puzzle.
