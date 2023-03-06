# 🗺 Anti Facist Indoctrination

![banner](https://socialify.git.ci/lixiang810/AntiFacistIndoctrination/image?description=1&font=KoHo&forks=1&issues=1&language=1&name=1&owner=1&pattern=Circuit%20Board&pulls=1&stargazers=1&theme=Dark)

## 2023-03-05 修正程序，适应新版大学习，请更新程序，否则可能无法使用

这是一个江苏省青年大学习自动完成器。往下翻之前可以看一则苏联笑话：

> 一所大学正在全力搜捕在校内张贴反法西斯标语的学生，请问这最有可能发生在何时何地？
>
> - A. 1936 年，德国，慕尼黑大学
> - B. 1937 年，日本，东京大学
> - C. 1935 年，中华民国，国立中央大学
> - D. 2022 年，中华人民共和国，清华大学
> - E. 2022 年，中华人民共和国，复旦大学（4 月 18 日新增）

## 📦 单机使用方法

```bash
cargo run -r -- -c <YOUR_COOKIE_HERE> [-s <YOUR_TOKEN_HERE>]
```

## 🏭 全自动完成

### Fork / Star 都快大于 2 了，各位用之前能不能 Star 一下？

首先，Fork 这个 Repo。然后为这个 Repo 设置一个名为 AFI_COOKIE 的 Secrets，里面填入你的 Cookie，像这样：`laravel_session=2333333333333333333333333333333333333333`

设置完之后，在 Repo 页面点击 Actions，按照里面的提示启用 Actions，否则 Action 不会运行。

这以后，这个 Action 在每周一 18 时会运行，帮你完成任务。值得注意的是，你需要每 60 天手动开启一下这个 Action，不然会被 GitHub 暂停。

## 🍪 如何获取 Cookie

见[此处](https://yuzaii.github.io/archives/c59a0c1a.html)（备份见[此处](https://web.archive.org/web/20230306140515/https://yuzaii.github.io/archives/c59a0c1a.html)），获得 `laravel_session=*` 后即可填入 Secrets。

## 📱 Server 酱消息推送

[注册 Server 酱](https://sct.ftqq.com/)并获取 `SendKey` ，然后为你 Fork 下的 Repo 设置一个名为 `AFI_TOKEN` 的 Secrets，里面填入 `SendKey`。这以后，本项目在运行后会使用 Server 酱向你推送结果。
