#!/home/taylor/.local/conda/bin/python

import angr
import sys

binary_path = sys.argv[1]
try:
    proj = angr.Project(binary_path, load_options={'auto_load_libs': False})
    cfg = proj.analyses.CFG(normalize=True)

    for func in cfg.kb.functions.values():
        print("!!> " + func.name)
except Exception as e:
    print(f"Failed to analyze with angr: {e}")