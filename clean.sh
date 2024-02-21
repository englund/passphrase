#!/usr/bin/env bash

illegal_chars=-.
start_length=4
end_length=5

while IFS=$'\n' read -r line;
do
    # Get the length of the line
    length=${#line}

    if (($length >= $start_length && $length <= $end_length)) \
        && [[ $line != *[$illegal_chars]* ]] \
        && [[ $line != [A-ZÅÄÖ]* ]]; then
        echo "$line"
    fi
done