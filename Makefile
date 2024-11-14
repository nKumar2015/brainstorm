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
			--all-features \
			-- \
			--no-deps \
			--deny warnings \
			--deny clippy::pedantic \
			--deny clippy::cargo \
			--allow clippy::trivially_copy_pass_by_ref \
			--allow clippy::missing_errors_doc \
			--allow clippy::module_name_repetitions \
			--allow clippy::cloned_instead_of_copied \
			--allow clippy::uninlined_format_args \
			--allow clippy::no_effect_underscore_binding \
			--allow clippy::cast_sign_loss \
			--allow clippy::must_use_candidate \
			--allow clippy::match_same_arms \
			--allow clippy::too_many_lines \
			--allow clippy::needless_pass_by_value \
			--allow clippy::unnested_or_patterns
						
	python3 scripts/check_line_length.py \
		'src/*.rs' \
		80
