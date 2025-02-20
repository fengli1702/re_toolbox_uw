#!/home/taylor/.local/conda/bin/python

import sys
import requests
from pathlib import Path

ENDPOINT= 'https://balamd.cumberland.isis.vanderbilt.edu/analyze'
USERNAME='balamd'
PASSWORD='!!balamd24'

file_path = Path(sys.argv[1])
file_contents = file_path.read_text()

session = requests.Session()
session.auth = (USERNAME, PASSWORD)
binthoven_result = response = session.post(ENDPOINT,
                    data={'decompiler': "ghidra",
                        'func_decomp': file_contents})
print(binthoven_result)