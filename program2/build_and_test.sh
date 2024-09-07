#! /bin/bash

while test $# -gt 0; do
    case "$1" in
        -h | --help)
            echo "Program to generate data and run it through program 2, Merge Sort"
            echo " "
            echo "arguments:"
            echo "-l | --length [len]       length of data to be generated, as an int"
            echo " "
            echo "options:"
            echo "-c | --cleanup            remove all .txt files once done"
            echo "-h | --help               display brief overview"
            exit 0
            ;;
        -l | --length)
            shift
            LEN=$1
            ;;
        -i | --indices)
            INDEXED=true;
            shift
            START=$1
            shift
            END=$1
            ;;
        -c | --cleanup)
            rm *.txt
            exit 0
            ;;
        *)
            break
            ;;
    esac
    shift
done

export FILENAME="unsorted$LEN.txt"

# First, cargo build
cargo build
# make generate program
make all
# generate data into output file
./generate_unsorted_data $LEN > $FILENAME
# pipe into rust program
if [[ $INDEXED == "true" ]]; then 
    export SORTED_FILENAME="sorted$LEN-$START-$END.txt"
    ./target/debug/program2 $START $END < $FILENAME > $SORTED_FILENAME
else 
    export SORTED_FILENAME="sorted$LEN.txt"
    ./target/debug/program2 < $FILENAME > $SORTED_FILENAME
fi

# cleanup
make clean
cargo clean