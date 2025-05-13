from DeGPT.degpt.role import single_run
import sys
import tempfile

input = sys.argv[1]

def read_code(f: str) -> str:
    with open(f, 'r') as r:
        return r.read()

degpt_str = single_run(read_code(input), 'out.json', 'all')



# Write result to a temp file, print its path
with tempfile.NamedTemporaryFile(
        mode="w",     # write text
        delete=False,   # keep the file
        encoding="utf-8"
    ) as tmp:
    tmp.write(degpt_str)
    tmp_path = tmp.name

    #print output filename
    print(tmp_path)
