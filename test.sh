#! bash

# ========== Running the default test ========== #

echo "Testing the default test suit"

# copying unitt to tests

cp unitt.art tests/test-default
cd tests/test-default

# running

./tester.art > output

# checking the return

diff --brief sample output ||           \
    diff --side-by-side sample output

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

diff --brief sample output ||           \
    diff --side-by-side sample output 

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

diff --brief sample output ||           \
    diff --side-by-side sample output

# cleaning test
rm unitt.art
rm output
cd ../..

# ========== Running the test suite ========== #

echo "Testing the test grouping via suite"

# copying unitt to tests

cp unitt.art tests/test-suite
cd tests/test-suite

# testing assertions
./suite.test.art > output 

diff --brief sample output ||           \
    diff --side-by-side sample output

# cleaning test
rm unitt.art
rm output
cd ../..