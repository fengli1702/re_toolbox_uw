#!/usr/bin/env python3

import sys
import tempfile
import subprocess

binary_path = sys.argv[1]
sub = subprocess.run(["clang-format", "--style", "Microsoft", binary_path], capture_output=True)
if sub.returncode == 0:
    contents = sub.stdout
    with tempfile.NamedTemporaryFile(mode='wb', delete=False) as fp:
        fp.write(contents)
        fp.close()
        print(fp.name)
