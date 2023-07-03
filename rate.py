import json
import os

gen = {}

with open("output.txt", "r") as f:
    for line in f:
        zi, code = line.split('\t')
        gen[zi] = code.strip()

flypy = {}

with open(os.path.join(os.path.dirname(__file__), 'flypy/小鹤音形单字全码码表.json'), 'r', encoding='utf-8') as f:
    flypy = json.load(f)

count = 0
output = []
for item in flypy:
    code = item['first_py'] + item['last_py']
    zi = item['character']

    if zi not in gen:
        print(f"{zi}: missing {code}")
        output.append(f"{zi}\t{code}\n")
    elif gen[zi] != code:
        print(f"{zi}: expected {code} but got {gen[zi]}")
        output.append(f"{zi}\t{code}\n")
    else:
        count += 1

print(f"{count} correct / {len(flypy)} total")

with open("查漏补缺.txt", "w") as f:
    f.writelines(output)
