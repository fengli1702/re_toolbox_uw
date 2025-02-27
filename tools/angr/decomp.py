#!/usr/bin/env python3

import angr
import sys

binary_path = sys.argv[1]
function_name = sys.argv[2]

try:
    proj = angr.Project(binary_path, load_options={'auto_load_libs': False})
    cfg = proj.analyses.CFG(normalize=True)
    for addr, func in proj.kb.functions.items():
        if func.name == function_name:
            print(func)
            dec = proj.analyses.Decompiler(proj.kb.functions[addr])
            print(dec.errors)
            print(dec.codegen.text)
except Exception as e:
    print(f"Failed to analyze with angr: {e}")