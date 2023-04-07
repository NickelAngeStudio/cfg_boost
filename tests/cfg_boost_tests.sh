####################################################
# FILE
# cfg_boost_tests.sh
#
# DESCRIPTION
# Generate project `cfg_boost_test` to test cfg_boost proc macros.
#
# PARAMETERS
# `-i` for integration test
# `-p` for performance
# `-s` for stress test
# `-v` for verbose
# `N` for test loop count
#
# USAGE
# $ bash cfg_boost_tests.sh [test_type] (verbose) [loop_count]
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

# Help showed when syntax incorrect
show_help() {
	echo "cfg_boost_tests Usage"
	echo "bash cfg_boost_tests.sh [test_type] (options) [loop_count]"
	echo ""
	echo "Test type"
	echo "-i : Integration"
	echo "-p : Performance"
	echo "-s : Stress"
	echo ""
	echo "Options"
	echo "-v : Verbose"
	echo "-x : Quit on error without cleaning"
	echo ""
	echo "Loop count"
	echo "1..N Loop count for performance or stress test"
	
	exit 0
}

# Return 1 in IS_NUMBER if number, 0 otherwise
# Ref : https://stackoverflow.com/questions/806906/how-do-i-test-if-a-variable-is-a-number-in-bash
is_number() {
	local re='^[0-9]+$'
	if ! [[ $1 =~ $re ]] ; then
		IS_NUMBER=0
	else
		IS_NUMBER=1
	fi
}

# Initialize loop count
init_loop_count() {
	is_number $1	# Verify if 2nd parameter is number
	if (( $IS_NUMBER == 1 )) ; then
		LOOP_COUNT=$1
	else
		is_number $2 # Verify if 3rd parameter is number
		if (( $IS_NUMBER == 1 )) ; then
			LOOP_COUNT=$2
		else
			is_number $3 # Verify if 4th parameter is number
			if (( $IS_NUMBER == 1 )) ; then
				LOOP_COUNT=$3
			else
				show_help # incorrect syntax
			fi
		fi
	fi
}

########
# INIT #
########
# 1. Change working directory for where the script is located.
cd "$(dirname "$0")"


# 2. Delete test folder if already exists
rm -r ../$PRJ_TEST_NAME

# 3. Global variables from arguments
TEST_TYPE=""
VERBOSE="N"
LOOP_COUNT=""
NO_CLEAN="N"
if [[ "$1" == "-i" ]]; then
	TEST_TYPE="INTEGRATION"
fi
if [[ "$1" == "-p" ]]; then
	TEST_TYPE="PERFORMANCE"
	init_loop_count $2 $3 $4
fi

if [[ "$1" == "-s" ]]; then
	TEST_TYPE="STRESS"
	init_loop_count $2 $3 $4
fi

if [[ "$2" == "-v" ]]; then
	VERBOSE="Y"
fi
if [[ "$3" == "-v" ]]; then
	VERBOSE="Y"
fi
if [[ "$2" == "-x" ]]; then
	NO_CLEAN="Y"
fi
if [[ "$3" == "-x" ]]; then
	NO_CLEAN="Y"
fi

if [[ "$TEST_TYPE" == "" ]]; then
	show_help # incorrect syntax
fi


# 4. Clear screen
printf "\033c"

# 5. Go to cfg_boost root
cd ..

# 6. Generate target_cfg package, allowing dirty
cargo package --allow-dirty

# 7. Get package version from Cargo.toml
package_version=""
while read line; do    
    if [[ "$line" == *"version ="* ]]; then
    	package_version=$(remove_quotes $(echo $line | awk '{print $3}'))
	fi
done < Cargo.toml

# 8. Generate blank project for test
cargo new $PRJ_TEST_NAME

# 9. Copy package into project
cp -r "target/package/cfg_boost-$package_version" "$PRJ_TEST_NAME/cfg_boost-$package_version"


# 10. Add depedency to new project cargo.toml
echo "cfg_boost = { path = \"cfg_boost-$package_version\", version = \"$package_version\" }" >> $PRJ_TEST_NAME/Cargo.toml
echo "" >> $PRJ_TEST_NAME/Cargo.toml


# 11. Move to test project folder
cd $PRJ_TEST_NAME


##############################
# Run test and get exit code #
##############################
if [[ "$TEST_TYPE" == "PERFORMANCE" ]]; then
	bash ../tests/cfg_boost_performance.sh $PRJ_TEST_NAME $VERBOSE $NO_CLEAN $LOOP_COUNT
elif [[ "$TEST_TYPE" == "STRESS" ]]; then
	bash ../tests/cfg_boost_stress.sh $PRJ_TEST_NAME $VERBOSE $NO_CLEAN $LOOP_COUNT
else
	bash ../tests/cfg_boost_integration.sh $PRJ_TEST_NAME $VERBOSE $NO_CLEAN
fi

# Get running status result
status=$?

if (( $status != 0 )) ; then
	if [[ "$NO_CLEAN" == "Y" ]]; then
		exit $status
	fi
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


########
# EXIT #
########
exit $status
