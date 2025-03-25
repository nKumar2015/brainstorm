export BRNSTM_LIB="/home/nakul/projects/brainstorm/test_lib"
find src | grep -v '^src/\.' | entr -c sh -c 'make check_lint && cargo build && target/debug/brainstorm ./src/test.txt'