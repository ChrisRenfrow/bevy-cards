#!/bin/bash

target_workspace=1
window_name="App"

while true; do
    # Get a copy of the tree to perform queries on
    sway_tree=$(swaymsg -t get_tree)
    cont_id=$(echo "$sway_tree" | jq -r ".. | select(.name?==\"$window_name\") | .id")


    if [ "$cont_id" == "" ]; then
        echo "Didn't find container with name \"$window_name\"..."
        sleep 1
        continue
    fi

    echo "Found window named $window_name with id $cont_id. Moving to workspace $target_workspace"
    swaymsg "[con_id=$cont_id] floating enable"
    swaymsg "[con_id=$cont_id] move container to workspace $target_workspace"
    sleep 1

done
