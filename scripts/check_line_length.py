#!/usr/bin/env python3

# `$0 <pattern> <max-len>` reads all files matching the wildcard `pattern` and
# outputs line numbers for all lines with length greater than `max-len`.

import glob
import sys

def main(pattern, max_len):
    found = False

    for file_path in glob.glob(pattern, recursive=True):
        with open(file_path, 'rb') as file:
            lines = file.read().split(b'\n')
            for (i, line) in enumerate(lines):
                if len(line) > max_len:
                    found = True
                    print("{}:{}: Length {}".format(file_path, i + 1, len(line)))
    if found:
        sys.exit(1)

if __name__ == '__main__':
    if len(sys.argv) != 3:
        sys.stderr.write("usage: {} <pattern> <max-len>\n".format(sys.argv[0]))
        sys.exit(1)

    pattern = sys.argv[1]
    max_len = int(sys.argv[2])

    main(pattern, max_len)
