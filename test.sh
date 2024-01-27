#! bash

# ========== Running the default test ========== #

# copying unitt to tests

cp unitt.art tests/testDefault
cd tests/testDefault

# running

./tester.art > output

# checking the return

cmp -- sample output && echo "Sucess!"   \
    || (
        echo "Failed!" &&
        echo "" && 
        diff sample output
    ) 

# cleaning test
rm unitt.art
rm output

