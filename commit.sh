#! /bin/bash

mkdir -p ../rust-xcb/build

for f in ../rust-xcb/build/*.rs; do
    rm $f
done

for f in src/*.rs; do
    cp $f ../rust-xcb/build
done

sed -i 's/\"src\"/\"build\"/g' ../rust-xcb/build/main.rs

MSG="$1"

if [ -n "$MSG" ]; then
    git add src/*
    git commit -m "$MSG"

    pushd ../rust-xcb
        git add build/*
        git commit -m "$MSG"
    popd
fi
