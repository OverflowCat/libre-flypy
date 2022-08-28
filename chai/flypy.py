from pychai import Schema

flypy = Schema('flypy')
flypy.run()

for nameChar in flypy.charList:
    if nameChar in flypy.component:
        scheme = flypy.component[nameChar]
    else:
        tree = flypy.tree[nameChar]
        componentList = tree.flatten_with_complex(flypy.complexRootList)
        scheme = sum((flypy.component[component] for component in componentList), tuple())
    if len(scheme) == 1:
        objectRoot = scheme[0]
        nameRoot = objectRoot.name
        # 单根字中的键名字，击四次该键，等效于取四次该字根
        if nameChar in '王土大木工目日口田山禾白月人金言立水火之已子女又幺':
            info = [nameRoot] * 4
        # 单根字中的单笔画字，取码为双击该键加上两个 L
        elif nameChar in '一丨丿丶乙':
            info = [nameRoot] * 2 + ['田'] * 2
        # 普通字根字，报户口 + 一二末笔
        else:
            firstStroke = objectRoot.strokeList[0].type
            secondStroke = objectRoot.strokeList[1].type
            if objectRoot.charlen == 2:
                info = [nameRoot, firstStroke, secondStroke]
            else:
                lastStroke = objectRoot.strokeList[-1].type
                info = [nameRoot, firstStroke, secondStroke, lastStroke]
    elif len(scheme) < 4:
        if nameChar in flypy.component or tree.structure not in 'hz':
            weima = '3'
        elif tree.structure == 'h': weima = '1'
        elif tree.structure == 'z': weima = '2'
        lastObjectRoot = scheme[-1]
        quma = flypy.category[lastObjectRoot.strokeList[-1].type]
        shibiema = quma + weima
        info = [objectRoot.name for objectRoot in scheme] + [shibiema]
    elif len(scheme) > 4:
        scheme = scheme[:3] + scheme[-1:]
        info = [objectRoot.name for objectRoot in scheme]
    else:
        info = [objectRoot.name for objectRoot in scheme]
    # code = ''.join(flypy.rootSet[nameRoot] for nameRoot in info)
    code = ' '.join(info)
    flypy.encoder[nameChar] = code

# print(flypy.gpdTime)
# print(flypy.decTime)
# print(flypy.selTime)

flypy.output()