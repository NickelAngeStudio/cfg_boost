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

# 1. Copy target_cfg_test folder.
cp -r target_cfg_test ../target_cfg_test

# 2. Go to target_cfg root
cd ..

# 3. Generate target_cfg package
cargo package --allow-dirty

# 4. Copy package into target_cfg_test


# 5. Modify copied target_cfg_test cargo.toml



# #. Delete target_cfg_test folder.
rm -r target_cfg_test

# #. Delete package directory
rm -r target/package

