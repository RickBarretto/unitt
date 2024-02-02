#! bash

# ========== Running the default test ========== #

echo "Testing the default test suit"

# copying unitt to tests

cp unitt.art tests/testDefault
cd tests/testDefault

# running

./tester.art > output

# checking the return

diff --brief --side-by-side sample output && echo -e "Sucess!\n" \
    || (
        echo "Failed!" &&
        echo ""
        # diff sample output
    ) 

# cleaning test
rm unitt.art
rm output

cd ../..


# ========== Running the finder test ========== #

echo "Testing the finder test suit"

# copying unitt to tests

cp unitt.art tests/test-finder
cd tests/test-finder

# running

./tester.art > output

# checking the return

diff --brief --side-by-side sample output && echo -e "Sucess!\n"   \
    || (
        echo -e "Failed!\n"
        # diff sample output
    ) 

# cleaning test
rm unitt.art
rm output

cd ../..

# ========== Running the failfast ========== #

echo "Testing the failfast for assertions"

# copying unitt to tests

cp unitt.art tests/test-failfast
cd tests/test-failfast

# testing assertions
./tester.art > output 

diff --brief --side-by-side sample output && echo "Sucess!"   \
    || (
        echo -e "Failed!\n"
    )

# cleaning test
rm unitt.art
rm output
cd ../..