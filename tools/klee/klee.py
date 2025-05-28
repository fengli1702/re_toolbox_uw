#!/usr/bin/env python3
import sys
import tempfile
import subprocess

if len(sys.argv) < 2:
    print("klee.py error: Missing binary_path argument.", file=sys.stderr)
    print("Usage: klee.py <path_to_llvm_bitcode_file.bc>", file=sys.stderr)
    sys.exit(2) # Exit with error code 2 for missing argument
binary_path = sys.argv[1]
print(f"klee.py: Received binary path: {binary_path}", file=sys.stderr)
try:
    print("Running KLEE on", binary_path, file=sys.stderr)
    # sub0 = subprocess.run(["klee", "--version"], capture_output=True)
    sub = subprocess.run(["klee", binary_path], capture_output=True)
    if sub.returncode == 0:
        contents = sub.stdout
        with tempfile.NamedTemporaryFile(mode='wb', delete=False) as fp:
            fp.write(contents)
            fp.close()
            print(fp.name)
    else:
        print(sub.stderr.decode(), file=sys.stderr)
        sys.exit(sub.returncode)
except FileNotFoundError:
    print("klee error", file=sys.stderr)
    sys.exit(127)
