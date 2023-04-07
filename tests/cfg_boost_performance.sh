####################################################
# FILE
# cfg_boost_performance.sh
#
# DESCRIPTION
# Calculate the cost on performance from using cfg_boost proc macros.
#
# PARAMETERS
# n/a
#
# USAGE
# $ bash cfg_boost_tests.sh via -s argument
#
# NOTE
# Must be called by cfg_boost_tests.sh
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

# Loop count of performance test
LOOP_COUNT=$4

#############
# FUNCTIONS #
#############

# Compare control $2 vs subject $4 and get difference in time in $cmp_time_diff. $1 and $3 are labels for $2 and $4.
compare_control_vs_subject(){
	# Execute control performance test
	execute_performance_test "$2" "$1"
	ctrl_elapsed=$test_time_elapsed
	ctrl_elapsed_ms=$(($ctrl_elapsed / 1000))
	echo -en "\rFinished $1 performance test... ~$ctrl_elapsed_ms ms"
	
	# Execute subject performance test
	echo ""
	execute_performance_test "$4" "$3"
	subj_elapsed=$test_time_elapsed
	subj_elapsed_ms=$(($subj_elapsed / 1000))
	echo -en "\rFinished $3 performance test... ~$subj_elapsed_ms ms"
	
	# Show time difference between both.
	cmp_time_diff=$((($subj_elapsed-$ctrl_elapsed) / 1000))
	echo ""
	echo -e "\033[1;33mPerformance cost of '$3' for $LOOP_COUNT optimized compilation is $cmp_time_diff ms.\033[0m"
}

# Execute performance tests and put result in $test_time_elapsed
execute_performance_test() {
	test_time_elapsed=0
	# Copy $1 over main.rs
	cp -r "../tests/rs/$1" "src/main.rs" 
	
	for (( c=1; c<=$LOOP_COUNT; c++ ))
	do
		if [[ "$VERBOSE" == "Y" ]]; then
			echo -en "\rExecute $2 performance test... $c of $LOOP_COUNT"
		fi
		result="$(cargo clean 2>&1)"	# Clean project to get fresh compilation
		start_time="$(date -u +%s%6N)"
		result="$(cargo build --release 2>&1)"
		end_time="$(date -u +%s%6N)"
		status=$?
		if (( $status != 0 )) ; then	# If build fail, show message and return 1.
			echo $result
			echo "-------------------------------"
			cat 'src/main.rs'	# Dump main.rs that caused error
			exit 1
		fi
		test_time_elapsed="$(($test_time_elapsed+$end_time-$start_time))"	# Add time used only for compilation
	done
}

#########
# TESTS #
#########
echo -en "\033[1;34m"
echo "################################"
echo "# target_cfg! performance test #"
echo "################################"
echo -en "\033[0m"
compare_control_vs_subject "target_cfg! control" "target_cfg_ctrl.rs" "target_cfg! macro" "target_cfg_perf.rs"

echo -e "\033[1;34m"
echo "###############################"
echo "# match_cfg! performance test #"
echo "###############################"
echo -en "\033[0m"
compare_control_vs_subject "match_cfg! control" "match_cfg_ctrl.rs" "match_cfg! macro" "match_cfg_perf.rs"

echo -e "\033[1;34m"
echo "#############################"
echo "# meta_cfg performance test #"
echo "#############################"
echo -en "\033[0m"
compare_control_vs_subject "meta_cfg control" "meta_cfg_ctrl.rs" "meta_cfg attribute" "meta_cfg_perf.rs"

echo ""

# Exit with code 0
exit 0

