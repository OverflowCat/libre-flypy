# libre-flypy

自由小鹤是一个自由的小鹤音形码表实现，旨在通过代码基于开源的数据上生成一套小鹤音形码表，并补全 Unicode 中存在但小鹤音形未编码的汉字。

注意，本项目目前处于早期开发阶段，不保证生成的码表的正确性。

目前表内形码的正确性为 **7884 / 8106 (97.26%)** 字，全码的正确性为 **7872 / 8106 (97.11%)** 字，输出共计 **52721** 个编码。

目前表内全码错误或缺失的字：

```
厂匕几九刁力又于才寸大丈与万口山巾及亡尸卫女飞丰井夫丐木五不车巨屯戈中内水见午牛手毛壬升长凶月氏丹方火斗丑书玉未末世本戊平东北业目且甲申电由史央皿凹生矢失乍丘乎甩印乐册永民弗出发母耳吏西夹夷尧曲朱廷乔臼舟兆争亥充米农声严求甫更束两来肖串我身龟免弃武丧画事雨妻齿果制垂秉所夜单肃承陋函柬面骨幽钙卸养逆乘旅兼继曹雀兜象毫涵寇惠翘凿鼎鹉毁颖毅赢疆乜廿卅尹夬爿毋耒曳凼聿艮丞卣沔臾亟眄禺鸩釜蚩堇菡啬啮颍彀嗯嵊滏榖臧廛畿縠墼羲豳篾卬𠙶毌㧑尨伭卺叚郪隺烝崡庳婳堾堼棤黹颋䐃𬭚漦酂彟
```

## 使用

可以在 Release 页面下载构建好的码表。

## 开发

阁下需要安装有 stable 版本的 Rust 工具链。

码表需要手动构建生成，本仓库中不包含构建产物。

本项目形码基于 [表意文字描述序列](https://zh.wikipedia.org/wiki/%E8%A1%A8%E6%84%8F%E6%96%87%E5%AD%97%E6%8F%8F%E8%BF%B0%E5%AD%97%E7%AC%A6) 拆字。克隆本仓库后，阁下需要 git clone [yi-bai/ids](https://github.com/yi-bai/ids) 到仓库中的 ids 文件夹。或者，将该仓库中的 [`ids_lv2.txt`](https://github.com/yi-bai/ids/blob/main/ids_lv2.txt) 复制到 `ids` 文件夹中。

同时，拼音数据来源于 [mozillazg/pinyin-data](https://github.com/mozillazg/pinyin-data)，克隆本仓库后，阁下需要 git clone 该仓库至 `pinyin-data` 文件夹中。或者，将该仓库中的 [`pinyin.txt`](https://github.com/mozillazg/pinyin-data/blob/master/pinyin.txt) 复制到 `pinyin-data` 文件夹中。

运行 `cargo run --release` 即可构建码表。构建完成后，码表将位于 `output.txt` 中。

## 测试

阁下需要安装有 Python 3.10+。

为验证表内字的编码正确性，本项目提供了一个测试工具。运行 `python rate.py` 即可运行测试。测试结果将输出到终端中。

## 贡献

欢迎提交 PR。如果想修复错误的形码，最好的方式是在 [my_ids.txt](./my_ids.txt) 中添加符合笔顺和拆字方式的表意文字描述序列。

## 协议

本项目代码使用 [AGPL](LICENSE) 协议，构建的码表释出到[公有领域](https://creativecommons.org/publicdomain/zero/1.0/deed.zh)。
