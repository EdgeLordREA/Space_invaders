#!/bin/bash

# Output file
OUTPUT="rust_bundle.txt"

# Clear or create the output file
> "$OUTPUT"

# Find all .rs files recursively and process them
find . -name "*.rs" -type f | while read -r file; do
    # Add a clear header with the file path
    echo "========================================" >> "$OUTPUT"
    echo "FILE: $file" >> "$OUTPUT"
    echo "========================================" >> "$OUTPUT"
    echo "" >> "$OUTPUT"
    
    # Append the file contents
    cat "$file" >> "$OUTPUT"
    
    # Add spacing between files
    echo "" >> "$OUTPUT"
    echo "" >> "$OUTPUT"
done

echo "Done! Bundled all .rs files into $OUTPUT"
echo "Total lines: $(wc -l < "$OUTPUT")"
