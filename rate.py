import json
import os

gen = {}

with open("output.txt", "r") as f:
    for line in f:
        zi, code = line.split('\t')
        gen[zi] = code.strip()

with open(os.path.join(os.path.dirname(__file__), 'flypy/小鹤音形单字全码码表.json'), 'r', encoding='utf-8') as f:
    flypy = json.load(f)
    count = 0
    for item in flypy:
        code = item['first_py'] + item['last_py']
        zi = item['character']

        if zi in gen:
            if gen[zi] != code:
                print(f"{zi}: expected {code} but got {gen[zi]}")
            else:
                count += 1
        else:
            print(f"{zi}: missing {code}")

    print(f"{count} correct / {len(flypy)} total")
