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

#############
# FUNCTIONS #
#############
# Remove quotes ""
remove_quotes() {
	echo $(echo $(echo $1 | sed 's/"//g'))
}

# Build a test and return result.
# $1 = script.rs to copy to main.rs
# $2 = result to expect, find
# $3+ = build arguments
build_test() {
	cp -r "../tests/rs/$1" "src/main.rs"  
	result="$(cargo build "$3 $4 $5 $6 $7" 2>&1)"
	
	# Evaluate result
	if [[ "$result" == *"$2"* ]]; then
		echo -e "\033[1;34mTEST $1\033[0m        [\033[1;32mPASS\033[0m]"
	else
		echo $result
		echo -e "\033[1;34mTEST $1\033[0m        [\033[1;31mFAIL\033[0m]"
	fi

}

# Run a test and return result.
# $1 = script.rs to copy to main.rs
# $2 = result to expect, find
run_test() {
	cp -r "../tests/rs/$1" "src/main.rs"  
	result="$(cargo run 2>&1)"
	
	# Evaluate result
	if [[ "$result" == *"$2"* ]]; then
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
#run_test 009.rs
#run_test 010.rs

# T11~T12 CfgBoostError::InvalidPredicateFormat error.
#run_test 011.rs
#run_test 012.rs

# T13~T14 CfgBoostError::EmptyArm error.
#run_test 013.rs
#run_test 014.rs

# T15~T16 CfgBoostError::WildcardArmNotLast error.
#run_test 015.rs
#run_test 016.rs

# T17~T18 CfgBoostError::ArmSeparatorMissing error.
#run_test 017.rs
#run_test 018.rs

# T19~T20 CfgBoostError::ContentSeparatorError error.
#run_test 019.rs
#run_test 020.rs

# T21~T22 CfgBoostError::WildcardArmMissing error.
#run_test 021.rs
#run_test 022.rs


# T## Test all predefined predicates (value:predicate) and build them via arguments
# "ar" => Ok(format!("target_arch = \"{}\"", label)),
# "tf" => Ok(format!("target_feature = \"{}\"", label)),
# "os" => Ok(format!("target_os = \"{}\"", label)),
# "fm" => Ok(format!("target_family = \"{}\"", label)),
# "ev" => Ok(format!("target_env = \"{}\"", label)),
# "ed" => Ok(format!("target_endian = \"{}\"", label)),
# "pw" => Ok(format!("target_pointer_width = \"{}\"", label)),
# "vn" => Ok(format!("target_vendor = \"{}\"", label)),
# "at" => Ok(format!("target_has_atomic = \"{}\"", label)),
# "pn" => Ok(format!("panic = \"{}\"", label)),
# "ft" => Ok(format!("feature = \"{}\"", label)),

# T## Create custom predicates and build them via arguments


# T## Test all predefined aliases and build them via arguments
# "linux" => Ok(String::from("linux:os")),
# "unix" => Ok(String::from("unix:fm")),
# "windows" => Ok(String::from("windows:fm")),
# "macos" => Ok(String::from("macos:os")),
# "android" => Ok(String::from("android:os")),
# "ios" => Ok(String::from("ios:os")),
# "wasm" => Ok(String::from("wasm:fm")),
# "desktop" => Ok(String::from("linux:os | windows:os | macos:os")),
# "mobile" => Ok(String::from("android:os | ios:os")),


# T## Create custom aliases and build them via arguments


# T## Generate documentation without [package.metadata.docs.rs] and read generated file to make sure labels are NOT included.


# T## Generate documentation with [package.metadata.docs.rs] and read generated file to make sure labels are included.


# Add documentation metadata to test project 
#echo "[package.metadata.docs.rs]" >> $PRJ_TEST_NAME/Cargo.toml
#echo "all-features = true" >> $PRJ_TEST_NAME/Cargo.toml
#echo "rustdoc-args = [\"--cfg\", \"docsrs\"]" >> $PRJ_TEST_NAME/Cargo.toml


# T## Stress test. Generate main.rs with lot of valid uses.



############
# CLEAN UP #
############
# 1. Go back to cfg_boost root
cd ..

# 2. Delete PRJ_TEST_NAME folder.
rm -r $PRJ_TEST_NAME

# 3. Delete package directory
rm -r target/package

