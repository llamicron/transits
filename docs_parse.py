import re

def parse_docs():
    with open("commands.txt", 'r') as fi:
        text = fi.read()

    matches = re.findall(r'(-.*?(?=-))', text)
    matches = [m.replace("\\n", '') for m in matches]
    matches = [' '.join(m.split()) for m in matches]
    print(matches[0])
    print(matches[1])
    print(matches[2])


parse_docs()
