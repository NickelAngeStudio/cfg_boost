####################################################
# FILE
# cfg_boost_tests.sh
#
# DESCRIPTION
# Generate project `cfgb_test` to test cfg_boost proc macros.
#
# PARAMETERS
# n/a
#
# USAGE
# $ bash cfg_boost_tests.sh
#
# NOTE
#
# DEPENDENCIES
# 32 bits compiler : sudo apt install gcc-multilib
# i686 linux toolchain : rustup target add i686-unknown-linux-gnu
#
# REFERENCES
#
# COPYRIGHT
# MIT
#
# NickelAnge.Studio
# 2023-03-23
####################################################

# Project test name
PRJ_TEST_NAME="cfg_boost_test"

# Count of tests passed
TOTAL_PASSED=0

# Count of total tests
TOTAL_TESTS=0

#############
# FUNCTIONS #
#############
# Remove quotes ""
remove_quotes() {
	echo $(echo $(echo $1 | sed 's/"//g'))
}

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
		echo -e "\033[1;34mTEST $1\033[0m        [\033[1;32mPASS\033[0m]"
	else
		echo $result
		echo -e "\033[1;34mTEST $1\033[0m        [\033[1;31mFAIL\033[0m]"
	fi

}


########
# INIT #
########
# 1. Clear screen
printf "\033c"

# 2. Go to cfg_boost root
cd ..

# 3. Generate target_cfg package, allowing dirty
cargo package --allow-dirty

# 4. Get package version from Cargo.toml
package_version=""
while read line; do    
    if [[ "$line" == *"version ="* ]]; then
    	package_version=$(remove_quotes $(echo $line | awk '{print $3}'))
	fi
done < Cargo.toml

# 5. Generate blank project for test
cargo new $PRJ_TEST_NAME

# 6. Copy package into project
cp -r "target/package/cfg_boost-$package_version" "$PRJ_TEST_NAME/cfg_boost-$package_version"


# 7. Add depedency to new project cargo.toml
echo "cfg_boost = { path = \"cfg_boost-$package_version\", version = \"$package_version\" }" >> $PRJ_TEST_NAME/Cargo.toml
echo "" >> $PRJ_TEST_NAME/Cargo.toml


# 8. Move to test project folder
cd $PRJ_TEST_NAME


########
# TEST #
########
# T1~T2 CfgBoostError::MissingOperator error.
run_test 001.rs "Target must not contain space."
run_test 002.rs "This is hello world from cfg_boost!"

# T3~T4 CfgBoostError::EmptyNode error.
run_test 003.rs "Empty node generated from attributes. Are you missing a statement between separator"
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
cargo doc

# HTML must NOT have label with class tab portability
TOTAL_TESTS=$((TOTAL_TESTS+1))
source=`cat target/doc/$PRJ_TEST_NAME/index.html`
if [[ "$source" == *"stab portability"* ]]; then
	echo -e "\033[1;34mTEST DOC.rs\033[0m        [\033[1;31mFAIL\033[0m]"
else
	TOTAL_PASSED=$((TOTAL_PASSED+1))
	echo -e "\033[1;34mTEST DOC.rs\033[0m        [\033[1;32mPASS\033[0m]"
fi

# Add documentation metadata to test project 
echo "[package.metadata.docs.rs]" >> Cargo.toml
echo "all-features = true" >> Cargo.toml
echo "rustdoc-args = [\"--cfg\", \"docsrs\"]" >> Cargo.toml


# T28 Generate documentation with [package.metadata.docs.rs] and read generated file to make sure labels are included.
run_test 028.rs "Test 028 completed!"
RUSTDOCFLAGS="--cfg docsrs" cargo +nightly doc --all-features

# HTML must have label with class tab portability
TOTAL_TESTS=$((TOTAL_TESTS+1))
source=`cat target/doc/$PRJ_TEST_NAME/index.html`
if [[ "$source" == *"stab portability"* ]]; then
	TOTAL_PASSED=$((TOTAL_PASSED+1))
	echo -e "\033[1;34mTEST DOC.rs\033[0m        [\033[1;32mPASS\033[0m]"
else
	echo -e "\033[1;34mTEST DOC.rs\033[0m        [\033[1;31mFAIL\033[0m]"
fi



# T29 Stress test. Generate main.rs with lot of valid uses.


#########
# TOTAL #
#########
if [[ $TOTAL_PASSED -eq $TOTAL_TESTS ]]; then
	echo -e "\033[1;34mRESULT : \033[0m \033[1;32m$TOTAL_PASSED of $TOTAL_TESTS passed\033[0m"
else
	echo -e "\033[1;34mRESULT : \033[0m \033[1;31m$TOTAL_PASSED of $TOTAL_TESTS passed\033[0m"
fi


############
# CLEAN UP #
############
# 1. Go back to cfg_boost root
cd ..

# 2. Delete PRJ_TEST_NAME folder.
rm -r $PRJ_TEST_NAME

# 3. Delete package directory
rm -r target/package

