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
	cp -r "../tests/rs/$1" "src/main.rs"  
	result="$(cargo run $3 $4 $5 $6 $7 2>&1)"
	
	# Evaluate result
	if [[ "$result" == *"$2"* ]]; then
		test_passed $1
	else
		test_failed $1 "$result"
	fi
}

# Generate cargo documentation. $1 == 0 is without nightly
generate_doc(){
	# Clean before generating docs
	result="$(cargo clean 2>&1)"
	
	if (( $1 == 0 )) ; then
		# Generate normally
		result="$(cargo doc 2>&1)"
		status=$?
		if (( $status != 0 )) ; then	# If doc fail, show message and return 1.
			echo $result
			exit 1
		fi
	else
		# Generate nightly
		result="$(RUSTDOCFLAGS="--cfg docsrs" cargo +nightly doc --all-features 2>&1)"
		status=$?
		if (( $status != 0 )) ; then	# If doc fail, show message and return 1.
			echo $result
			exit 1
		fi
	fi
	
}

# Read documentation and verify if it contains $2. $1 is used to label test.
doc_test_has() {
	source=`cat target/doc/$PRJ_TEST_NAME/index.html`
	if [[ "$source" == *"$2"* ]]; then
		test_passed $1
	else
		test_failed $1 "$source"
	fi
}

# Read documentation and verify that it doesn't contains $2. $1 is used to label test.
doc_test_hasnt() {
	source=`cat target/doc/$PRJ_TEST_NAME/index.html`
	if [[ "$source" == *"stab portability"* ]]; then
		test_failed $1 "$source"
	else
		test_passed $1
	fi
}

# Test passes $1 = test name
test_passed() {
	TOTAL_TESTS=$((TOTAL_TESTS+1))
	
	TOTAL_PASSED=$((TOTAL_PASSED+1))
	if [[ "$VERBOSE" == "Y" ]]; then
		echo -e "\033[1;34mTEST $(echo $1)\033[0m        [\033[1;32mPASS\033[0m]"
	fi
}

# Test failed  $1 = test name, $2 = message
test_failed() {
	TOTAL_TESTS=$((TOTAL_TESTS+1))
	
	if [[ "$VERBOSE" == "Y" ]]; then
		echo -e "\033[1;34mTEST $(echo $1)\033[0m        [\033[1;31mFAIL\033[0m]"
		echo "$source"
	fi
	if [[ "$EXIT_ON_ERROR" == "Y" ]]; then
		exit 1
	fi
}

########
# TEST #
########
# T1~T2 CfgBoostError::MissingOperator error.
run_test 001.rs "Target must not contain space."
run_test 002.rs "This is hello world from cfg_boost!"

# T3~T4 CfgBoostError::LegacySyntaxError error.
run_test 003.rs "Legacy syntax error in"
run_test 004.rs "Test 004 completed!"

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
generate_doc 0

# HTML must NOT have label with class tab portability
doc_test_hasnt "DOC001" "stab portability"

# HTML must have function documented
doc_test_has "DOC002" "UnixOnly"
doc_test_has "DOC003" "LinuxOnly"
doc_test_has "DOC004" "windows_only"
doc_test_has "DOC005" "android_only"
doc_test_has "DOC006" "x64sse2"
doc_test_has "DOC007" "wasm_only"
doc_test_has "DOC008" "JohnDoe"
doc_test_has "DOC009" "LinuxOnly2"
doc_test_has "DOC010" "WindowsOnly2"
doc_test_has "DOC011" "NotLinux"
doc_test_hasnt "DOC012" "hidden_doc"

# Add documentation metadata to test project 
echo "[package.metadata.docs.rs]" >> Cargo.toml
echo "all-features = true" >> Cargo.toml
echo "rustdoc-args = [\"--cfg\", \"docsrs\"]" >> Cargo.toml


# T28 Generate documentation with [package.metadata.docs.rs] and read generated file to make sure labels are included.
run_test 028.rs "Test 028 completed!"
generate_doc 1

# HTML must have label with class tab portability
doc_test_has "DOC013" "stab portability"

# T29~T30 CfgBoostError::WildcardArmOnTarget error.
run_test 029.rs "target_cfg! macro cannot have a"
run_test 030.rs "Test 030 completed!"

# T31 Generate documentation with [package.metadata.docs.rs] while deactivating documentation. Read generated file to make sure labels are NOT included.
run_test 031.rs "Test 031 completed!"
generate_doc 1

# HTML must NOT have label with class tab portability
doc_test_hasnt "DOC014" "stab portability"

# T32~T33 CfgBoostError::TargetInFunction error.
run_test 032.rs "target_cfg! macro cannot be used inside a function. Use match_cfg! instead."
run_test 033.rs "Test 033 completed!"

# T34-T35 Auto documentation config
cp -r "../tests/rs/034.rs" "src/main.rs"  
generate_doc 1

doc_test_has "DOC015" "stab portability"

# Copy autodoc ovveride config.toml
cp -r "../tests/rs/autodoc.toml" ".cargo/config.toml"  

# Rerun doc
generate_doc 1

# Previously documented elements should now be gone.
doc_test_hasnt "DOC016" "stab portability"

# T36 Legacy syntax in target_cfg!
run_test 036.rs "Test 036 completed!"

# T37-T38 Legacy syntax in match_cfg!
run_test 037.rs "Test 037 completed!"
run_test 038.rs "Test 038 completed!"

# T39 Legacy syntax in meta_cfg!
run_test 039.rs "Test 039 completed!"

#T40-T42 CfgBoostError::MixedSyntaxError
run_test 040.rs "Legacy syntax and simplified syntax can't be mixed on same arm!"
run_test 041.rs "Legacy syntax and simplified syntax can't be mixed on same arm!"
run_test 042.rs "Test 042 completed!"

#T43-T44 CfgBoostError::ContentSeparatorMissing
run_test 043.rs "Arm content separator"
run_test 044.rs "Test 044 completed!"

#T45-T46 CfgBoostError::ModifierNotFirst 
run_test 045.rs "must be the first character of arm!"
run_test 046.rs "Test 046 completed!"

#T47-T48 CfgBoostError::MatchModifierMoreThanOneActivate
run_test 047.rs "match_cfg! cannot have more than one"
run_test 048.rs "Test 048 completed!"

#T49-T50 CfgBoostError::MatchDeactivatedWildArm
run_test 049.rs "match_cfg! cannot deactivate wildcard arm with"
run_test 050.rs "Test 050 completed!"

#T51-T52 CfgBoostError::ModifierPanicRelease
result="$(cargo build --release 2>&1)"
if [[ "$result" == *"will panic during release compilation by default!"* ]]; then
	test_passed "051REL"
else
	test_failed "051REL" "$result"
fi

# Override panic default value to ignore
echo "cfg_boost_release_modifier_behaviour = { value = \"ignore\", force = true }" >> .cargo/config.toml

result="$(cargo build --release 2>&1)"
if [[ "$result" == *"will panic during release compilation by default!"* ]]; then
	test_failed "052REL" "$result"
else
	test_passed "052REL"
fi

#T53 Modifier + on target_cfg!
run_test 053.rs "Test 053 completed!"

#T54 Modifier + on match_cfg!
run_test 054.rs "Test 054 completed!"

#T55 Modifier + on meta_cfg!
run_test 055.rs "Test 055 completed!"

#T56 Modifier - on target_cfg!
run_test 056.rs "Test 056 completed!"

#T57 Modifier - on match_cfg!
run_test 057.rs "Test 057 completed!"

#T58 Modifier - on meta_cfg!
run_test 058.rs "Test 058 completed!"

#T59 Modifier + and - on target_cfg!
run_test 059.rs "Test 059 completed!"

#T60 Modifier + and - on match_cfg!
run_test 060.rs "Test 060 completed!"

#T61 Modifier + and - on meta_cfg!
run_test 061.rs "Test 061 completed!"

#T62-T63 Modifier @
run_test 062.rs "Macro panicked because some arm have the"
run_test 063.rs "Test 063 completed!"


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
