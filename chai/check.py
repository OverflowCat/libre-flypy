from pychai import lookup

from codecs import open
from collections import defaultdict
from yaml import load, dump
try:
    from yaml import CLoader as Loader, CDumper as Dumper
except ImportError:
    from yaml import Loader, Dumper

""" def lookup(sourceChar: str, numList: list):
    WEN = loadFromPackage('wen.yaml')
    ZI = loadFromPackage('zi.yaml')

    msg = ''
    if sourceChar in WEN:
        msg = format('恭喜！您现在可以为该字根起一个名字，在 mapper 中添加这个字根的名字，并在 aliaser 中注册，语法如下：\n【名字】: [%s, %s]' % (sourceChar, str(numList).replace("'", '')))
        return (True, msg)
    elif sourceChar in ZI:
        msg = format('您提供的汉字「%s」不是基本部件，它的结构为：%s。请尝试将您需要的字根定位到这些基本部件中，然后重新查询。' % (sourceChar, str(ZI[sourceChar]).replace("'", '')))
    else:
        msg = format('您提供的汉字「%s」不在 GB 字集内。请您使用常用汉字查询字根。' % sourceChar)
    return (False, msg) """


data = load(open('flypy_src.schema.yaml', 'r'), Loader=Loader)
for k, v in data['小字'].items():
    for c in v:
        if not isinstance(c, str):
            continue
        (ok, msg) = lookup(c, [0])
        if not ok:
            print(msg)

# 部件：
""" 
您提供的汉字「骨」不是基本部件，它的结构为：[z, 骨上, 青下]。请尝试将您需要的字根定位到这些基本部件中，然后重新查询。
您提供的汉字「黑」不是基本部件，它的结构为：[z, 黑上, 灬]。请尝试将您需要的字根定位到这些基本部件中，然后重新查询。
您提供的汉字「足」不是基本部件，它的结构为：[z, 口, 足下]。请尝试将您需要的字根定位到这些基本部件中，然后重新查询。
"""
# 小字：
"""
您提供的汉字「卵」不是基本部件，它的结构为：[h, 卵左, 卵右]。请尝试将您需要的字根定位到这些基本部件中，然后重新查询。
您提供的汉字「丽」不是基本部件，它的结构为：[z, 一, [h, [n, 冂, 丶], [n, 冂, 丶]]]。请尝试将您需要的字根定位到这些基本部件中，然后重新查询。
您提供的汉字「灭」不是基本部件，它的结构为：[z, 一, 火]。请尝试将您需要的字根定位到这些基本部件中，然后重新查询。
您提供的汉字「面」不是基本部件，它的结构为：[z, 而上, 面下]。请尝试将您需要的字根定位到这些基本部件中，然后重新查询。
您提供的汉字「乞」不是基本部件，它的结构为：[z, 每上, 乙]。请尝试将您需要的字根定位到这些基本部件中，然后重新查询。
您提供的汉字「勺」不是基本部件，它的结构为：[q, 勹, 丶]。请尝试将您需要的字根定位到这些基本部件中，然后重新查询。
您提供的汉字「丫」不是基本部件，它的结构为：[z, 总上, 丨]。请尝试将您需要的字根定位到这些基本部件中，然后重新查询。
"""
