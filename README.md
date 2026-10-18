<div align="center">

<img src=".github/resources/banner.png" width="100%" alt="slot：为 Anbernic RG SP 量身打造、以 Game Boy 为核心的前端。">

<a href="LICENSE"><img src="https://img.shields.io/github/license/BrandonKowalski/slot?style=for-the-badge&labelColor=242429&color=4d4d57&label=%E8%AE%B8%E5%8F%AF%E8%AF%81" height="36" alt="许可证"></a>
<a href="https://github.com/BrandonKowalski/slot/stargazers"><img src="https://img.shields.io/github/stars/BrandonKowalski/slot?style=for-the-badge&labelColor=242429&color=4d4d57&label=%E6%98%9F%E6%A0%87" height="36" alt="星标"></a>
<a href="https://github.com/BrandonKowalski/slot/releases"><img src="https://img.shields.io/github/downloads/BrandonKowalski/slot/total?style=for-the-badge&labelColor=242429&color=4d4d57&label=%E4%B8%8B%E8%BD%BD%E9%87%8F" height="36" alt="下载量"></a>
<a href="https://slot-cfw.fyi"><img src="https://img.shields.io/badge/%E6%8C%87%E5%8D%97-slot--cfw.fyi-4d4d57?style=for-the-badge&labelColor=242429" height="36" alt="指南"></a>
<a href="CHANGELOG.md"><img src="https://img.shields.io/badge/%E6%9B%B4%E6%96%B0%E6%97%A5%E5%BF%97-4d4d57?style=for-the-badge&labelColor=242429" height="36" alt="更新日志"></a>

</div>

---

# slot 是什么？

slot 是一款为 Anbernic RG SP 量身打造、以 Game Boy 为核心的前端。

仅支持 GBA、GBC 和 GB 游戏。

> [!IMPORTANT]
> ROM 必须解压后使用。slot 无法读取 `.zip` 或 `.7z` 文件。

完整的用户指南请参阅 [slot-cfw.fyi](https://slot-cfw.fyi)。

版本说明请参阅[更新日志](CHANGELOG.md)。

---

# 它是什么样的？

<p align="center">
<img src=".github/resources/screenshots/carousel.gif" width="48%" alt="在 Game Boy Advance、Game Boy 和 Game Boy Color 游戏架之间滚动卡带轮播列表">
<img src=".github/resources/screenshots/core-picker.gif" width="48%" alt="翻转卡带显示电路板，以选择 mGBA 或 gpSP">
</p>

<p align="center">
<img src=".github/resources/screenshots/link.gif" width="96.5%" alt="两台 SP 通过模拟联机功能进行《Mario Kart》竞速">
</p>

---

# 支持的设备

| 设备                | 是否支持                         | 起始版本 |
|---------------------|----------------------------------|----------|
| Anbernic RG SP      | 是                               | 始终支持 |
| Anbernic RG34XX     | 是                               | 1.4.0    |
| Anbernic RG34XXSP   | 是（需验证左摇杆映射十字键功能） | 1.4.0    |
| Anbernic RG35XXSP   | 是                               | 1.5.0    |
| Anbernic 35XX Plus  | 是                               | 1.5.0    |
| Anbernic RG28XX     | 是                               | 1.6.0    |
| Anbernic RG35XX-H   | 是                               | 1.7.0    |

如果你拥有尚未测试的设备，或曾在列表之外的设备上尝试运行 slot，请[在 GitHub 上提交 Issue](https://github.com/BrandonKowalski/slot/issues)并告知测试结果。

---

# 简体中文构建

通过 `task build:device` 生成的设备构建使用简体中文界面。如需在主机上运行简体中文构建，请执行 `cargo run --release -p slot --features lang-zh-cn`。

中文游戏名会根据首个非空白汉字的拼音首字母归入 A–Z 分组，上、下键的字母跳转也会使用这些分组；数字、标点及无法识别的文字统一归入 `#` 分组。

存储卡随附虎卅泓制作的 **Plix Bold**（普力士）字体（[cathree3/Plix](https://github.com/cathree3/Plix)）。该字体以 IBM Plex Sans 为基础，采用 SIL Open Font License 授权。设备包中同时包含 `System/fonts/Plix-Bold.otf` 及其许可证文本。二进制文件内还嵌入了一小部分 Plix 字形，以确保缺少存储卡字体时中文界面仍可正常阅读。

`System/fonts/` 支持自定义 TTF、OTF 和 TTC 文件，并使用按文件名排序后的第一个字体。作为单文件替代方案，`System/font.ttf` 仍受支持。

---

# AI 使用说明

Rust 前端以及 [slot-cfw.fyi](https://slot-cfw.fyi) 上的 SVG 均由 Claude 编写，我审核了它生成的全部内容。所有文档则 100% 由自由放养的碳基人类执笔。

我开发这个项目，是想重现儿时坐在汽车后座玩 GBA SP 的感觉，同时加入一些现代化的便利功能。

从技术角度看，这个项目没有什么令人兴奋或开创性的内容，因此我把胶水代码交给了 AI。mGBA 和 gpSP 已经存在多年，才是这个项目真正的主角。出色的拟物化美术由 [Markus](https://github.com/MarkusMaribu) 创作；他看到最初那个简陋的版本后，主动提出要为它精心装扮。

如果你无法接受我使用 AI，我完全理解。只需知道，AI 是这里最无趣的部分。

想来还是应该分享一下这些被浪费的水最终换来了什么。✌🏻
