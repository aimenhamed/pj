#!/bin/bash

add_pj_to_shell() {
    local shell="$1"
    local rc_file="$2"
    local pj_function="$3"

    if [ -f "$rc_file" ]; then
        # Check if the pj function already exists in the rc file
        if grep -q "pj()" "$rc_file"; then
            echo "pj function already exists in $rc_file."
        else
            # Append the pj function definition to rc file
            echo "$pj_function" >> "$rc_file"
            echo "Added pj function to $rc_file."
        fi
    else
        echo "$rc_file not found."
    fi
}

cargo build --release
sudo cp target/release/pj /usr/local/bin/pj

pj_function='pj() {
  CONFIG_PATH="$HOME/.pj.json"
  /usr/local/bin/pj "$@"
  if [ $# -eq 0 ]; then
    PATH_TO_CD=$(cat $CONFIG_PATH | jq -r ".last_opened")
    cd "$PATH_TO_CD"
  fi
}'

shell_type="$(basename "$SHELL")"
case "$shell_type" in
    "bash")
        add_pj_to_shell "$shell_type" "$HOME/.bashrc" "$pj_function"
        ;;
    "zsh")
        add_pj_to_shell "$shell_type" "$HOME/.zshrc" "$pj_function"
        ;;
    *)  
        echo "Unsupported shell: $shell_type. Cannot add pj function to configuration file automatically."
        ;;
esac

