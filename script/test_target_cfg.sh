####################################################
# FILE
# test_target_cfg.sh
#
# DESCRIPTION
# Generate project `target_cfg_test` to test procedural macros.
#
# PARAMETERS
# n/a
#
# USAGE
# $ bash test_target_cfg.sh
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

# Remove quotes ""
remove_quotes() {
	echo $(echo $(echo $1 | sed 's/"//g'))
}

# 1. Clear screen
printf "\033c"

# 2. Copy target_cfg_test folder.
cp -r target_cfg_test ../target_cfg_test

# 3. Go to target_cfg root
cd ..

# 4. Generate target_cfg package, allowing dirty
cargo package --allow-dirty

# 5. Get package version from Cargo.toml
package_version=""
while read line; do    
    if [[ "$line" == *"version ="* ]]; then
    	package_version=$(remove_quotes $(echo $line | awk '{print $3}'))
	fi
done < Cargo.toml

# 6. Copy package into target_cfg_test
cp -r "target/package/target_cfg-$package_version" "target_cfg_test/target_cfg-$package_version"


# 7. Modify copied target_cfg_test cargo.toml
echo "target_cfg = { path = \"target_cfg-$package_version\", version = \"$package_version\" }" >> target_cfg_test/Cargo.toml
echo "" >> target_cfg_test/Cargo.toml
echo "[package.metadata.docs.rs]" >> target_cfg_test/Cargo.toml
echo "all-features = true" >> target_cfg_test/Cargo.toml
echo "rustdoc-args = [\"--cfg\", \"docsrs\"]" >> target_cfg_test/Cargo.toml

# 8. Move to target_cfg_test folder
cd target_cfg_test

# Execute a test and return result.
# $1 = script.rs to copy to main.rs
# $2 = result to expect, find
execute_test() {
	cp -r "../script/$1" "src/main.rs"  
	result="$(cargo build 2>&1)"
	
	# Evaluate result
	if [[ "$result" == *"$2"* ]]; then
		echo -e "\033[1;34mTEST $1\033[0m        [\033[1;32mPASS\033[0m]"
	else
		echo -e "\033[1;34mTEST $1\033[0m        [\033[1;31mFAIL\033[0m]"
	fi

}
########
# TEST #
########
# T1. Empty cfg_target ()
execute_test 001_cfgt_empty.rs "Empty node generated from attributes. Are you missing a statement between separator?"

# #. Go back to target_cfg root
cd ..

# #. Delete target_cfg_test folder.
rm -r target_cfg_test

# #. Delete package directory
rm -r target/package

