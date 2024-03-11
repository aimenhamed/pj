#!/bin/bash

cargo build --release
sudo cp target/release/pj /usr/local/bin/pj

# Define the pj function
pj_function='pj() {
  CONFIG_PATH="$HOME/.pj.json"
  /usr/local/bin/pj "$@"
  if [ $# -eq 0 ]; then
    PATH_TO_CD=$(cat $CONFIG_PATH | jq -r ".last_opened")
    cd "$PATH_TO_CD"
  fi
}'

# Check if .zshrc exists
zshrc_file=~/.zshrc
if [ -f "$zshrc_file" ]; then
    # Check if the pj function already exists in .zshrc
    if grep -q "pj()" "$zshrc_file"; then
        echo "pj function already exists in $zshrc_file."
    else
        # Append the pj function definition to .zshrc
        echo "$pj_function" >> "$zshrc_file"
        echo "Added pj function to $zshrc_file."
    fi
else
    echo "$zshrc_file not found."
fi

