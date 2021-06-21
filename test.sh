#!/bin/sh

exit_code() {
    expr=$1
    expected=$2
    ../target/debug/ggvm $expr > func_amd64.s
    make && ./a.out
    code=$?
    if [ $code -eq $expected ] ; then
        echo "[\e[32mSUCCESS\e[37m] exit code test succeeded, got=${expr}"
    else
        echo "[\e[31mFAILED!!\e[37m]exit code test failed, args=${expr} expected=${expected}, got=${code}"
        exit 1
    fi
    make nuke
}

cd tests/
exit_code 42 42
exit_code 2 2
