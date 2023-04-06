####################################################
# FILE
# cfg_boost_integration.sh
#
# DESCRIPTION
# Integration tests for target_cfg!, match_cfg! and meta_cfg!
#
# PARAMETERS
# n/a
#
# USAGE
# $  Must be called by cfg_boost_tests.sh with no arguments.
#
# NOTE
# 
#
# REFERENCES
#
# COPYRIGHT
# MIT
#
# NickelAnge.Studio
# 2023-04-03
####################################################

# Validate that call comes from cfg_boost_tests.sh
if [[ "${PWD##*/}" != "$1" ]]; then
    echo "ERROR : Stress test must be executed by cfg_boost_tests.sh"
    exit 1
fi

# Project test name
PRJ_TEST_NAME=$1

# Verbose
VERBOSE=$2

# Exit on error
EXIT_ON_ERROR=$3

#############
# FUNCTIONS #
#############

# Run a test and return result.
# $1 = script.rs to copy to main.rs
# $2 = result to expect, find
# $3+ = build arguments
run_test() {
	TOTAL_TESTS=$((TOTAL_TESTS+1))
	
	cp -r "../tests/rs/$1" "src/main.rs"  
	result="$(cargo run $3 $4 $5 $6 $7 2>&1)"
	
	# Evaluate result
	if [[ "$result" == *"$2"* ]]; then
		TOTAL_PASSED=$((TOTAL_PASSED+1))
		if [[ "$VERBOSE" == "Y" ]]; then
			echo -e "\033[1;34mTEST $1\033[0m        [\033[1;32mPASS\033[0m]"
		fi
	else
		echo $result
		if [[ "$VERBOSE" == "Y" ]]; then
			echo -e "\033[1;34mTEST $1\033[0m        [\033[1;31mFAIL\033[0m]"
		fi
		if [[ "$EXIT_ON_ERROR" == "Y" ]]; then
			exit 1
		fi
	fi

}

# Read documentation and verify if it contains $2. $1 is used to label test.
doc_test_has() {
	TOTAL_TESTS=$((TOTAL_TESTS+1))
	source=`cat target/doc/$PRJ_TEST_NAME/index.html`
	if [[ "$source" == *"$2"* ]]; then
		TOTAL_PASSED=$((TOTAL_PASSED+1))
		if [[ "$VERBOSE" == "Y" ]]; then
			echo -e "\033[1;34mTEST $(echo $1)\033[0m        [\033[1;32mPASS\033[0m]"
		fi
	else
		if [[ "$VERBOSE" == "Y" ]]; then
			echo -e "\033[1;34mTEST $(echo $1)\033[0m        [\033[1;31mFAIL\033[0m]"
		fi
		if [[ "$EXIT_ON_ERROR" == "Y" ]]; then
			exit 1
		fi
	fi
}

# Read documentation and verify that it doesn't contains $2. $1 is used to label test.
doc_test_hasnt() {
	TOTAL_TESTS=$((TOTAL_TESTS+1))
	source=`cat target/doc/$PRJ_TEST_NAME/index.html`
	if [[ "$source" == *"stab portability"* ]]; then
		if [[ "$VERBOSE" == "Y" ]]; then
			echo -e "\033[1;34mTEST $(echo $1)\033[0m        [\033[1;31mFAIL\033[0m]"
		fi
		if [[ "$EXIT_ON_ERROR" == "Y" ]]; then
			exit 1
		fi
	else
		TOTAL_PASSED=$((TOTAL_PASSED+1))
		if [[ "$VERBOSE" == "Y" ]]; then
			echo -e "\033[1;34mTEST $(echo $1)\033[0m        [\033[1;32mPASS\033[0m]"
		fi
	fi
}

########
# TEST #
########
# T1~T2 CfgBoostError::MissingOperator error.
run_test 001.rs "Target must not contain space."
run_test 002.rs "This is hello world from cfg_boost!"

# T3~T4 CfgBoostError::EmptyNode error. * NOT TESTED ANYMORE SINCE CATCHED EARLIER THAN SYNTAX TREE *
# run_test 003.rs "Empty node generated from attributes. Are you missing a statement between separator"
# run_test 004.rs "Test 004 completed!"

# T5~T6 CfgBoostError::InvalidCharacter error.
run_test 005.rs "Invalid character"
run_test 006.rs "Test 006 completed!"

# T7~T8 CfgBoostError::AliasNotFound error.
run_test 007.rs "has no match! Is it added in config.toml"
run_test 008.rs "Test 008 completed!"

# T9~T10 CfgBoostError::InvalidConfigurationPredicate error.
run_test 009.rs "Configuration predicate"
run_test 010.rs "Test 010 completed!"

# T11~T12 CfgBoostError::EmptyArm error.
run_test 011.rs "Empty arm with no attributes detected!"
run_test 012.rs "Test 012 completed!"

# T13~T14 CfgBoostError::WildcardArmNotLast error.
run_test 013.rs "must ALWAYS be the last branch"
run_test 014.rs "Test 014 completed!"

# T15~T16 CfgBoostError::ArmSeparatorMissing error.
run_test 015.rs "Arm syntax incorrect. Are you missing a separator"
run_test 016.rs "Test 016 completed!"

# T17~T18 CfgBoostError::ContentSeparatorError error.
run_test 017.rs "Arm syntax incorrect. Is your arm separator"
run_test 018.rs "Test 018 completed!"

# T19~T20 CfgBoostError::WildcardArmMissing error.
run_test 019.rs "Ensure that all possible cases are being handled by adding a match arm with a"
run_test 020.rs "Test 020 completed!"


# T21 Test all predefined predicates (value:predicate)
run_test 021.rs "Test 021 completed!"


# T22 Custom predicates missing
run_test 022.rs "has no match! Is it added in config.toml"

# Copy predicates
mkdir ".cargo"
cp -r "../tests/rs/pred.toml" ".cargo/config.toml"  

# T23 Custom predicates added
run_test 023.rs "Test 023 completed!"

# T24 Test all predefined aliases
run_test 024.rs "Test 024 completed!"


# T25 Custom aliases missing
run_test 025.rs "has no match! Is it added in config.toml"

# Copy aliases
cp -r "../tests/rs/alias.toml" ".cargo/config.toml" 

# T26 Custom aliases added
run_test 026.rs "Test 026 completed!"

# T27 Generate documentation without [package.metadata.docs.rs] and read generated file to make sure labels are NOT included.
run_test 027.rs "Test 027 completed!"
result="$(cargo doc 2>&1)"

# HTML must NOT have label with class tab portability
doc_test_hasnt "DOC001" "stab portability"

# Add documentation metadata to test project 
echo "[package.metadata.docs.rs]" >> Cargo.toml
echo "all-features = true" >> Cargo.toml
echo "rustdoc-args = [\"--cfg\", \"docsrs\"]" >> Cargo.toml


# T28 Generate documentation with [package.metadata.docs.rs] and read generated file to make sure labels are included.
run_test 028.rs "Test 028 completed!"
result="$(RUSTDOCFLAGS="--cfg docsrs" cargo +nightly doc --all-features 2>&1)"


# HTML must have label with class tab portability
doc_test_has "DOC002" "stab portability"


# T29~T30 CfgBoostError::WildcardArmOnTarget error.
run_test 029.rs "target_cfg! macro cannot have a"
run_test 030.rs "Test 030 completed!"

# T31 Generate documentation with [package.metadata.docs.rs] while deactivating documentation. Read generated file to make sure labels are NOT included.
run_test 031.rs "Test 031 completed!"
result="$(RUSTDOCFLAGS="--cfg docsrs" cargo +nightly doc --all-features 2>&1)"

# HTML must NOT have label with class tab portability
doc_test_hasnt "DOC003" "stab portability"

# T32~T33 CfgBoostError::TargetInFunction error.
run_test 032.rs "target_cfg! macro cannot be used inside a function. Use match_cfg! instead."
run_test 033.rs "Test 033 completed!"

# T34-T35 Auto documentation config
cp -r "../tests/rs/034.rs" "src/main.rs"  
result="$(RUSTDOCFLAGS="--cfg docsrs" cargo +nightly doc --all-features 2>&1)"

doc_test_has "DOC004" "stab portability"

# Copy autodoc ovveride config.toml
cp -r "../tests/rs/autodoc.toml" ".cargo/config.toml"  

# Clean cargo
result="$(cargo clean 2>&1)"

# Rerun doc
result="$(RUSTDOCFLAGS="--cfg docsrs" cargo +nightly doc --all-features 2>&1)"

# Previously documented elements should now be gone.
doc_test_hasnt "DOC005" "stab portability"



#########
# TOTAL #
#########
if [[ $TOTAL_PASSED -eq $TOTAL_TESTS ]]; then
	echo -e "\033[1;34mRESULT : \033[0m \033[1;32m$TOTAL_PASSED of $TOTAL_TESTS passed\033[0m"
else
	echo -e "\033[1;34mRESULT : \033[0m \033[1;31m$TOTAL_PASSED of $TOTAL_TESTS passed\033[0m"
fi

# Exit with total - passed
status=$((TOTAL_TESTS-TOTAL_PASSED))
exit $status
