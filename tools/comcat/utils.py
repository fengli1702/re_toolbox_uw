import re
#import time

pattern = re.compile(
    r'//.*?$|/\*.*?\*/|\'(?:\\.|[^\\\'])*\'|"(?:\\.|[^\\"])*"',
    re.DOTALL | re.MULTILINE
)

def comment_remover(code, remove_important=False):
    def replacer(match):
        s = match.group(0)
        if s.startswith('/'):  # It's a comment
            if not remove_important:
                # Preserve comments starting with '!'
                if s.startswith('/**!') or s.startswith('/*!') or s.startswith('//!'):
                    return s
            return " "  # Replace with a space
        else:
            return s  # Leave strings and character literals untouched
    
    return re.sub(pattern, replacer, code)

# Sometimes, ChatGPT will include a markdown code block (inside ```something like this```)
# when producing code. This function will remove that if it exists.
def remove_markdown_code_block(text):
    return re.sub(r"^```[\w+]*\n?|```$", "", text).strip()

def word_before_parenthesis(s):
    # Find the index of the first "("
    index = s.find("(")
    
    # Check if "(" is not found
    if index == -1:
        return None

    # Get the substring up to the index of "("
    substring = s[:index].strip()

    # Split the substring into words and get the last word
    words = substring.split()
    if words:
        return words[-1]
    else:
        return None

'''
def comment_code(code):
    while True:
        try:
            commented_file = comment_file_str(comment_remover(code))
            break
        except:
            print("Sleeping.")
            time.sleep(120) # wait 2 minutes

    return commented_file
'''