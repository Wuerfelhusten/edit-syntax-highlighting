#!/bin/bash
# Bash Script Syntax Test

# Variables
NAME="World"
COUNT=42
readonly CONST="constant"

# Arrays
FRUITS=("Apple" "Banana" "Orange")
declare -a numbers=(1 2 3 4 5)

# Functions
function greet() {
    local name=$1
    echo "Hello, ${name}!"
}

say_goodbye() {
    echo "Goodbye, $1!"
}

# Control structures
if [ $COUNT -gt 10 ]; then
    echo "Count is greater than 10"
elif [ $COUNT -eq 10 ]; then
    echo "Count equals 10"
else
    echo "Count is less than 10"
fi

# Loops
for i in {1..5}; do
    echo "Number: $i"
done

for fruit in "${FRUITS[@]}"; do
    echo "Fruit: $fruit"
done

while [ $COUNT -gt 0 ]; do
    echo "Countdown: $COUNT"
    ((COUNT--))
done

# Case statement
case "$1" in
    start)
        echo "Starting..."
        ;;
    stop)
        echo "Stopping..."
        ;;
    restart)
        echo "Restarting..."
        ;;
    *)
        echo "Usage: $0 {start|stop|restart}"
        exit 1
        ;;
esac

# Command substitution
current_date=$(date +%Y-%m-%d)
files_count=`ls -1 | wc -l`

# Pipes and redirections
cat file.txt | grep "pattern" | sort | uniq > output.txt
find . -name "*.txt" 2>/dev/null

# Conditionals
[ -f "file.txt" ] && echo "File exists"
[ -d "directory" ] || mkdir directory

# Arithmetic
result=$((5 + 3))
result=$((result * 2))

# String operations
string="Hello World"
echo "${string:0:5}"      # Hello
echo "${string/World/Bash}" # Hello Bash

# Exit codes
greet "Alice"
if [ $? -eq 0 ]; then
    echo "Success"
fi

exit 0
