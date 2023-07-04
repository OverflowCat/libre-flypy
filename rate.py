from collections import defaultdict
from rich import print
import json
import os

gen = defaultdict(list)

with open("output.txt", "r") as f:
    for line in f:
        zi, code = line.split('\t')
        gen[zi].append(code.strip())

flypy = {}

with open(os.path.join(os.path.dirname(__file__), 'flypy/小鹤音形单字全码码表.json'), 'r', encoding='utf-8') as f:
    flypy = json.load(f)

count = 0
output = []
zis = []
for item in flypy:
    code = item['fly_code'][0:4]
    zi = item['character']

    if zi not in gen:
        print(f"{zi}: [yellow]missing {code}[/yellow]")
        output.append(f"{zi}\t{code}\n")
        zis.append(zi)
    elif code not in gen[zi]:
        音 = code[:2]
        if not any(音 == c[:2] for c in gen[zi]):
            print(
                f"{zi}: 音码错误, expected [green]{code}[/green] but got [medium_orchid]{' / '.join(gen[zi])}[/medium_orchid]"
            )
        else:
            print(
                f"{zi}: 形码错误, expected [green]{code}[/green] but got [red]{' / '.join(gen[zi])}[/red]"
            )
            output.append(f"{zi}\t{code}\n")
        zis.append(zi)
    else:
        count += 1

total = len(flypy)

print(f"[green]{count}[/green] correct / [blue]{total}[/blue] total,",
      f"[pink]{count/total*100:.2f}%[/pink]")

with open("查漏补缺.txt", "w") as f:
    f.writelines(output)

print("错误或缺失的字：[red]" + "".join(zis) + "[/red]")
