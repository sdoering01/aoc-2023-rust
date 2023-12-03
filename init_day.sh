#!/bin/sh
# Creates a new crate for the next day and copies the template into it.
# Currently only works on mac with `gfind` installed.
last_day_num=$(gfind -maxdepth 1 -type d -name 'day-*' | sort | tail -n 1 | cut -d "-" -f 2)
new_day_num=$(printf %02d $(( $last_day_num + 1 )))
new_day_dir="day-$new_day_num"

cargo new $new_day_dir --vcs none
cp template.rs $new_day_dir/src/main.rs
