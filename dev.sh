export BRNSTM_LIB="/home/nakul/projects/rustlang/test-lib"
find src | grep -v '^src/\.' | entr -c sh -c 'make check_lint && cargo build && target/debug/brainstorm ./src/test.txt'