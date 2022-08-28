from codecs import open
from collections import defaultdict
from yaml import load, dump
try:
    from yaml import CLoader as Loader, CDumper as Dumper
except ImportError:
    from yaml import Loader, Dumper

data = load(open('flypy_src.schema.yaml', 'r'), Loader=Loader)
mapper = defaultdict(list)

for k, v in data['部件'].items():
    mapper[k] += v
data.pop('部件')

for k, v in data['小字'].items():
    mapper[k] += v
data.pop('小字')
data['mapper'] = dict(mapper)

output = dump(data, Dumper=Dumper)
with open('flypy.schema.yaml', 'w') as f:
    f.write(output)