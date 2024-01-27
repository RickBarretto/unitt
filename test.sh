#! bash

# ========== Running the default test ========== #

# copying unitt to tests

cp unitt.art tests/testDefault
cd tests/testDefault

# running

./tester.art > output

# checking the return

cmp -- ./sample ./output    \
    && echo sucess          \
    || echo failed

# cleaning test
rm unitt.art
rm output

