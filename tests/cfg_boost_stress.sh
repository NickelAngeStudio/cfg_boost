####################################################
# FILE
# cfg_boost_stress.sh
#
# DESCRIPTION
# Stress test target_cfg!, match_cfg! and meta_cfg! by generating complex and random 'main.rs'
#
# PARAMETERS
# n/a
#
# USAGE
# $ bash cfg_boost_tests.sh via -s argument
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

# Loop count of stress test
LOOP_COUNT=$4

# List of predefined and custom aliases
ALIASES=("linux" "unix" "windows" "macos" "android" "ios" "wasm" "doc" "test" "desktop" "mobile" "pig" "dog" "cow" "parastratiosphecomyia_stratiosphecomyioides" "mosquito" "frog" "lion" "fish" "b")

# List of predefined and custom predicates
PREDICATES=("ar" "tf" "os" "fm" "ev" "ed" "pw" "vn" "at" "pn" "ft" "_" "c1" "c2" "c3" "c4" "c5" "c6" "c7" "c8" "c9" "really_long_predicate_and_i_mean_really_longgggggggggg" "x")

# List of predefined legacy cfg
LEGACY=("#[cfg(target_os=\"foo\")]"
		"#[cfg(all(target_os=\"foo\", target_architecture=\"foo\"))]"
		"#[cfg(all(unix, target_pointer_width = \"32\"))]"
		"#[cfg(target_os = \"foo2\")]"
		"#[cfg(feature = \"foo\")]")

# Limit of block generated per section
BLOCK_LIMIT=10

# Limit of arms for target and match macros
ARM_LIMIT=10

# Limit of function item in target_cfg!
ITEM_LIMIT=10

# Count total of block written
BLOCK_TOTAL=0

# Limit of predicate for an attribute
PREDICATE_LIMIT=20

#############
# FUNCTIONS #
#############
generate_main_header() {
	# Overwrite main.rs
	echo "#![cfg_attr(docsrs, feature(doc_cfg))]" > 'src/main.rs'
	echo "use cfg_boost::{target_cfg, match_cfg, meta_cfg};" >> 'src/main.rs'
}

# Generate arm attributes from predicates or legacy
generate_arm_attr() {

	local pred_type=$(( $RANDOM % 5 ))
	if [[ "$pred_type" == "0" ]]; then		# Legacy
		local index=$(( $RANDOM % ${#LEGACY[@]} ))	# Generate legacy index
		echo ${LEGACY[$index]}
	else
		echo $(generate_predicates)
	fi

}

# Generate a string of predicates for arm or meta.
generate_predicates() {
	local predicate_total=$(( $RANDOM % $PREDICATE_LIMIT + 1 ))
	local predicate_string=""
	local cpt=0;
	
	while [ $cpt -le $predicate_total ]
	do
		# is_not(!) modifier 
		local pred_type=$(( $RANDOM % 5 ))
		local is_not=""
		
		if [[ "$pred_type" == "0" ]]; then		# Add is_not
			is_not="!"
		fi
			
		# Chance for a group or not
		local pred_type=$(( $RANDOM % $PREDICATE_LIMIT ))
		if [[ "$pred_type" == "0" ]]; then		# Create group
			local predicate_group=$(generate_predicates)
			predicate_string="$predicate_string ($predicate_group)"
		else
			
			local pred_type=$(( $RANDOM % 3 ))
			if [[ "$pred_type" == "0" ]]; then		# Add alias
				local index=$(( $RANDOM % ${#ALIASES[@]} ))	# Generate alias index
				predicate_string="$predicate_string $(echo $is_not)${ALIASES[$index]}"
			else	# Add predicate
				local index=$(( $RANDOM % ${#ALIASES[@]} ))	# Generate predicate index
				predicate_string="$predicate_string $(echo $is_not)foo$(echo $cpt)_$(echo $BLOCK_TOTAL):${PREDICATES[$index]}"
			fi
		fi
		
		# And / Or
		if [ $cpt -lt $predicate_total ]; then
			local pred_type=$(( $RANDOM % 2 ))
	
			if [[ "$pred_type" == "0" ]]; then		# And
				predicate_string="$predicate_string &"
			else	# Or
				predicate_string="$predicate_string |"
			fi
		fi
		
		cpt=$(( $cpt+1 ))	# Increment loop counter
	done

	
	echo $predicate_string
}

# Generate 0..BLOCK_LIMIT target_cfg! block
generate_target_cfg() {
	local target_total=$(( $RANDOM % $BLOCK_LIMIT ))
	
	echo "" >> 'src/main.rs'
	for (( b=0; b<=$target_total; b++ ))
	do
		# Open target_cfg macro
		echo "target_cfg!{"	>> 'src/main.rs'
		
		
		local arm_total=$(( $RANDOM % $ARM_LIMIT ))
		for (( a=0; a<=$arm_total; a++ ))
		do
			local pred=$(generate_arm_attr)
			
			# Open arm
			echo "$pred => {"	>> 'src/main.rs'
			
			local fn_total=$(( $RANDOM % $ITEM_LIMIT ))
			
			for (( f=1; f<=$fn_total; f++ ))	# Write arm functions.
			do
				local gen_doc=$(( $RANDOM % 3 ))
				if [[ "$gen_doc" == "0" ]]; then		# Add documentation
					echo "/// Documentation for foo$(echo $BLOCK_TOTAL)_$(echo $f)_$(echo $a)" >> 'src/main.rs'
				fi
				echo "fn foo$(echo $BLOCK_TOTAL)_$(echo $f)_$(echo $a)() {}" >> 'src/main.rs'
			done
			
			# close arm
			echo "}," >> 'src/main.rs' 
		done
		
		# Close target_cfg macro
		echo "}" >> 'src/main.rs'
		
		BLOCK_TOTAL=$(( $BLOCK_TOTAL+1 ))
	done
}

# Generate 0..BLOCK_LIMIT match_cfg! block
generate_match_cfg() {
	
	local match_total=$(( $RANDOM % $BLOCK_LIMIT ))
	
	for (( m=0; m<=$match_total; m++ ))
	do
		echo "" >> 'src/main.rs'
		
		local gen_doc=$(( $RANDOM % 3 ))
		if [[ "$gen_doc" == "0" ]]; then		# Add documentation
			echo "/// Documentation for foo$(echo $BLOCK_TOTAL)_$(echo $m)" >> 'src/main.rs'
		fi
		
		# open function
		echo "fn foo$(echo $BLOCK_TOTAL)_$(echo $m)() {" >> 'src/main.rs'
		
		# open match cfg
		echo "let foo=match_cfg!{"	>> 'src/main.rs'
	
		local arm_total=$(( $RANDOM % $ARM_LIMIT ))
		for (( a=0; a<=$arm_total; a++ ))
		do
			local pred=$(generate_arm_attr)
			local value=$(( $m*($RANDOM % $BLOCK_TOTAL) ))
			
			# Write arm
			echo "$pred => $value,"	>> 'src/main.rs' 
		done
		# insert wildcard
		echo "_ => 0" >> 'src/main.rs'
	
		# close match cfg
		echo "};" >> 'src/main.rs'
	
		# close function
		echo "}" >> 'src/main.rs'
	
		BLOCK_TOTAL=$(( $BLOCK_TOTAL+1 ))
	done

}

# Generate 0..BLOCK_LIMIT meta_cfg! block
generate_meta_cfg() {
	local meta_total=$(( $RANDOM % $BLOCK_LIMIT ))
	
	for (( m=0; m<=$meta_total; m++ ))
	do
		local pred=$(generate_arm_attr)
		echo "" >> 'src/main.rs'
		
		local gen_doc=$(( $RANDOM % 3 ))
		if [[ "$gen_doc" == "0" ]]; then		# Add documentation
			echo "/// Documentation for foo$(echo $BLOCK_TOTAL)_$(echo $m)" >> 'src/main.rs'
		fi
				
				
		echo "#[meta_cfg($pred)]" >> 'src/main.rs'
		echo "fn foo$(echo $BLOCK_TOTAL)_$(echo $m)() {}" >> 'src/main.rs'
		
		BLOCK_TOTAL=$(( $BLOCK_TOTAL+1 ))
	done
	
	
	
}

generate_main_footer() {
	echo "fn main() {}"	>> 'src/main.rs'
}


########
# BODY #
########
# Copy aliases
mkdir ".cargo"
cp -r "../tests/rs/alias.toml" ".cargo/config.toml" 

echo -en "\033[1;34m"
echo "####################"
echo "# cfg_boost STRESS #"
echo "####################"
echo -en "\033[0m"

start_time="$(date -u +%s)"
for (( c=1; c<=$LOOP_COUNT; c++ ))
do
	if [[ "$VERBOSE" == "Y" ]]; then
		echo -en "\rExecuting stress test $c of $LOOP_COUNT..."
	fi

	# 1. Clean project
	result="$(cargo clean 2>&1)"
	
	# 2. Overwrite file and generate header
	generate_main_header
	
	# 3. Generate target_cfg! blocks
	generate_target_cfg
	
	# 4. Generate match_cfg! block
	generate_match_cfg
	
	# 5. Generate meta_cfg block
	generate_meta_cfg
	
	# 6. Finish file by writing footer
	generate_main_footer
	
	# 7. Compile project as release
	result="$(cargo build --release 2>&1)"
	status=$?
	if (( $status != 0 )) ; then	# If build fail, show message and return 1.
		echo $result
		echo "-------------------------------"
		cat 'src/main.rs'	# Dump main.rs that caused error
		exit 1
	fi
	
	# 8. Generate documentation
	result="$(RUSTDOCFLAGS="--cfg docsrs" cargo +nightly doc --all-features 2>&1)"
	if [[ "$result" != *"Finished"* ]]; then	# If doc fail, show message and return 1.
		echo $result
		echo "-------------------------------"
		cat 'src/main.rs'	# Dump main.rs that caused error
		exit 1
	fi
	
done
end_time="$(date -u +%s)"

diff=$(($end_time-$start_time))
echo ""
echo -e "\033[1;33mStress tests finished in $diff seconds.\033[0m"

# Exit with code 0
exit 0
