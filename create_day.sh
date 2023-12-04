#!/bin/bash

# Check if exactly one argument was provided
if [ "$#" -ne 1 ]; then
    echo "Usage: $0 day_number"
    exit 1
fi

# Check if the directory already exists
if [ -d "day$1" ]; then
    echo "Error: Directory day$1 already exists."
    exit 1
fi

# Create the directory
mkdir "day$1"

# Change to the new directory
cd "day$1"

# Initialize a new Rust project
cargo init

# Copy the .gitignore file
cp ../day1/.gitignore .
