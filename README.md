Puzzle screening tools
======================

Preparation
-----------

### `make-rocksdb`

Prepare an offline database to lookup game moves by id from
https://database.lichess.org.

### `add-game-moves`

Adds game moves from the database as an
[additional column to the puzzle CSV](https://drive.google.com/file/d/1kjfQPQ-CjSSER_JOHa_NvY38xK4JsHOc/view?usp=sharing).

Criteria
--------

### `repetitions`

Find repetitions in the final phase before the puzzle position.
See https://github.com/lichess-org/tactics/issues/101#issuecomment-909176503.

### `implied-castling-rights`

Find puzzle positions that look like they have more castling rights than they
do. See https://github.com/lichess-org/tactics/issues/101#issuecomment-909246532.

### `duplicate-position`

There is already a puzzle with the same position.
