#!/bin/bash

clean() {
    rm -f tmp*
}

try() {
    expected="$1"
    input="$2"

    ./target/debug/ikbk-9cc "$input" > tmp.s
    gcc -o tmp tmp.s
    ./tmp
    actual="$?"

    if [ "$actual" = "$expected" ]; then
        echo "$input => $actual"
    else
        echo "$expected expected, but got $actual"
        clean
        exit 1
    fi
}

try 0 0
try 42 42

echo OK
clean
