#!/bin/bash

NUM_FILES=100
DIR_NAME="testdir"

mkdir -p ${DIR_NAME}

# Create Python files
for ((i=1; i<=NUM_FILES; i++)); do
    # Generate a dummy name (e.g., 1.py, 2.py, ..., 100.py)
    file_name="${DIR_NAME}/${i}.py"

    # Create the Python file with a simple comment inside
    echo "# This is a dummy Python file." > ${file_name}
done

echo "Done creating ${NUM_FILES} Python files."
