#! bash

# ========== Running the default test ========== #

cp unitt.art tests/test-default
cd tests/test-default

./tester.art > sample

rm unitt.art
cd ../..

# ========== Running the finder test ========== #

cp unitt.art tests/test-finder
cd tests/test-finder

./tester.art > sample

rm unitt.art
cd ../..

# ========== Running the failfast ========== #

cp unitt.art tests/test-failfast
cd tests/test-failfast

./tester.art > sample

rm unitt.art
cd ../..

# ========== Running the test suite ========== #

cp unitt.art tests/test-suite
cd tests/test-suite

./suite.test.art > sample

rm unitt.art
cd ../..

# ========== Running the test eval ========== #

cp unitt.art tests/test-eval
cd tests/test-eval

./test.art > sample

rm unitt.art
cd ../..