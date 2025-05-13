# Code Parser part of the ComCat pipeline. Calling get_blocks gets all the code blocks in a file to be commented.

import clang.cindex
import re
import subprocess
import tempfile
import os

blocks = []

def run_linter(cpp_code):
    # Write input code to a temporary file
    with tempfile.NamedTemporaryFile(mode='w', delete=False) as temp_file:
        temp_file.write(cpp_code)
        temp_file.flush()

        # Run cpplint on the temporary file
        process = subprocess.Popen(['clang-format', temp_file.name], stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
        stdout, stderr = process.communicate()

    # Delete the temporary file
    os.unlink(temp_file.name)

    # Check if there are any errors or warnings
    if process.returncode == 0:
        return stdout.strip()
    else:
        return stderr.strip()

def block2str(cursor, source_file):
    '''
    tokens = [x.spelling for x in cursor.get_tokens()]
    contents = ' '.join(tokens)
    contents = run_linter(contents)
    return contents
    '''
    extent = cursor.extent

    with open(source_file) as file:
        source_lines = file.readlines()

    # Extract the source code covered by the extent using line numbers
    start_line = extent.start.line - 1  # Zero-based line number
    end_line = extent.end.line - 1      # Zero-based line number

    source_code = ''.join(source_lines[start_line:end_line + 1])

    return source_code


def parse(cursor, file_name):
    if cursor.location.file is not None and cursor.location.file.name != file_name:
        return  # Skip cursors from included files

    # Associate every kind of cursor we care about with a comment type (template)
    # and add the cursor with its type to the blocks list as a tuple.
    # Based on our observations, we can avoid commenting children of certain block types,
    # Like code within conditionals
    if cursor.kind == clang.cindex.CursorKind.FUNCTION_DECL:
        blocks.append((0,block2str(cursor, file_name)))
    elif cursor.kind == clang.cindex.CursorKind.CXX_METHOD:
        blocks.append((0,block2str(cursor, file_name)))
    elif cursor.kind == clang.cindex.CursorKind.CONSTRUCTOR:
        blocks.append((0,block2str(cursor, file_name)))
    elif cursor.kind == clang.cindex.CursorKind.DESTRUCTOR:
        blocks.append((0,block2str(cursor, file_name)))
    elif cursor.kind == clang.cindex.CursorKind.FUNCTION_TEMPLATE:
        blocks.append((0,block2str(cursor, file_name)))
    elif cursor.kind == clang.cindex.CursorKind.IF_STMT:
        blocks.append((3,block2str(cursor, file_name)))
        return
    elif cursor.kind == clang.cindex.CursorKind.SWITCH_STMT:
        blocks.append((3,block2str(cursor, file_name)))
        return
    elif cursor.kind == clang.cindex.CursorKind.WHILE_STMT:
        blocks.append((2,block2str(cursor, file_name)))
    elif cursor.kind == clang.cindex.CursorKind.DO_STMT:
        blocks.append((2,block2str(cursor, file_name)))
    elif cursor.kind == clang.cindex.CursorKind.FOR_STMT:
        blocks.append((2,block2str(cursor, file_name)))
    #elif cursor.kind == clang.cindex.CursorKind.RETURN_STMT: # returns are rarely complicated enough to warrant a comment
    #    blocks.append((2,block2str(cursor, file_name)))
    elif cursor.kind == clang.cindex.CursorKind.CXX_TRY_STMT:
        blocks.append((2,block2str(cursor, file_name)))
    elif cursor.kind == clang.cindex.CursorKind.CXX_FOR_RANGE_STMT:
        blocks.append((2,block2str(cursor, file_name)))
    elif cursor.kind == clang.cindex.CursorKind.VAR_DECL:
        blocks.append((1,block2str(cursor, file_name)))
        return
    elif cursor.kind == clang.cindex.CursorKind.STRUCT_DECL:
        blocks.append((1,block2str(cursor, file_name)))
    elif cursor.kind == clang.cindex.CursorKind.UNION_DECL:
        blocks.append((1,block2str(cursor, file_name)))
    elif cursor.kind == clang.cindex.CursorKind.CLASS_DECL:
        blocks.append((1,block2str(cursor, file_name)))
    elif cursor.kind == clang.cindex.CursorKind.CLASS_TEMPLATE:
        blocks.append((1,block2str(cursor, file_name)))
    elif cursor.kind == clang.cindex.CursorKind.ENUM_DECL:
        blocks.append((1,block2str(cursor, file_name)))
    elif cursor.kind == clang.cindex.CursorKind.TYPEDEF_DECL:
        blocks.append((1,block2str(cursor, file_name)))

    for child in cursor.get_children():
        parse(child, file_name)
    
    #return blocks

def get_blocks(filename):
    # Just in case if this is a subsequent call to get_blocks
    blocks.clear()

    # Create the index
    index = clang.cindex.Index.create()

    # Parse the translation unit (source file)
    translation_unit = index.parse(filename)

    # Traverse the AST and add appropriate code blocks to the list
    parse(translation_unit.cursor, translation_unit.spelling)

    return list(dict.fromkeys(blocks)) #dedup blocks

def get_blocks_str(code):
    with tempfile.NamedTemporaryFile(mode='w', suffix='.cpp') as temp_file:
        temp_file.write(code)
        temp_file.flush()
        
        return get_blocks(temp_file.name)

def main():
    print(get_blocks("./test.cpp"))

if __name__ == '__main__':
    main()
