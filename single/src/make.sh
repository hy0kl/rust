#!/usr/bin/env bash
# @describe:
# @author:   Jerry Yang(hy0kle@gmail.com)

#set -x

target="read-file write iterator exec dining-philosophers args string"

argc=$#
if ((argc < 1))
then
    echo "Usage: $0 (build|clean)"
    exit 0
fi

action=$1

if [ "build" = "$action" ]
then
    for tag in $target
    do
        rustc $tag.rs
    done
elif [ "clean" = "$action" ]
then
    for tag in $target
    do
        rm "$tag"
    done
else
    echo "invalid action: $action"
fi
