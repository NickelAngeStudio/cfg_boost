####################################################
# FILE
# cfg_boost_tests.sh
#
# DESCRIPTION
# Generate project `cfg_boost_test` to test cfg_boost proc macros.
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

########
# INIT #
########
# 0. Change working directory for where the script is located.
cd "$(dirname "$0")"

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


##############################
# Run test and get exit code #
##############################
if [[ "$1" == "-p" ]]; then
	bash ../tests/cfg_boost_performance.sh $PRJ_TEST_NAME
elif [[ "$1" == "-s" ]]; then
	bash ../tests/cfg_boost_stress.sh $PRJ_TEST_NAME
else
	bash ../tests/cfg_boost_integration.sh $PRJ_TEST_NAME
fi

# Get running status result
status=$?

############
# CLEAN UP #
############
# 1. Go back to cfg_boost root
cd ..

# 2. Delete PRJ_TEST_NAME folder.
rm -r $PRJ_TEST_NAME

# 3. Delete package directory
rm -r target/package


########
# EXIT #
########
exit $status
