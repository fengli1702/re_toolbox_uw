#!/usr/bin/env python3

import sys
from pathlib import Path
import tempfile
import cxxfilt


binary_path = sys.argv[1]
txt = Path(binary_path).read_text()
demangled = cxxfilt.demangle(txt)

with tempfile.NamedTemporaryFile(mode='w', delete=False) as fp:
    fp.write(demangled)
    print(fp.name)
