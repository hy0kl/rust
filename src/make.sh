#!/bin/bash
# @describe:
# @author:   Jerry Yang(hy0kle@gmail.com)

#set -x

target="read-file write iterator exec dining-philosophers"

for tag in $target
do
    rustc $tag.rs
done
