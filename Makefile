.PHONY: check
check: \
	check_intg \
	check_lint

# We run integration tests in sequence because some tests in `tests/cli.rs` run
# a Git daemon during testing and running multiple instances of the Git daemon
# at the same will result in port collisions and will cause failures.
.PHONY: check_intg
check_intg: $(tgt_test_dir)
	TEST_DIR='$(shell pwd)/$(tgt_test_dir)' \
		cargo test \
			-- \
			--show-output \
			--test-threads=1 \
			$(TESTS)

.PHONY: check_lint
check_lint:
	TEST_DIR='$(shell pwd)/$(tgt_test_dir)' \
		cargo clippy \
			--all-targets \
			--all-features
	python3 scripts/check_line_length.py \
		'src/*.rs' \
		80
