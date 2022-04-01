# 🗺 Anti Facist Indoctrination

这是一个江苏省青年大学习自动完成器。往下翻之前可以看一则苏联笑话：

> 一所大学正在全力搜捕在校内张贴反法西斯标语的学生，请问这最有可能发生在何时何地？
>
> - A. 1936 年，德国，慕尼黑大学
> - B. 1937 年，日本，东京大学
> - C. 1935 年，中华民国，国立中央大学
> - D. 2022 年，中华人民共和国，清华大学

## 📦 单机使用方法

```bash
cargo run -r -- -c <YOUR_COOKIE_HERE>
```

## 🏭 全自动完成

首先，Fork 这个 Repo。然后为这个 Repo 设置一个名为 AFI_COOKIE 的 Secrets，里面填入你的 Cookie，像这样：`laravel_session=2333333333333333333333333333333333333333`

这以后，这个 Action 在每周一 18 时会运行，帮你完成任务。

## 🌏 如何获取 Cookie

见[此处](http://yuzai.xyz/%e9%9d%92%e5%b9%b4%e5%a4%a7%e5%ad%a6%e4%b9%a0%e6%8a%93%e5%8c%85%e6%95%99%e7%a8%8b/)，获得 `laravel_session=*` 后即可填入 Secrets。
