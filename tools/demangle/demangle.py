#!/home/taylor/.local/conda/bin/python

import sys
import cxxfilt

binary_path = sys.argv[1]
with open(binary_path) as f:
    
    print(cxxfilt.demangle(binary_path))