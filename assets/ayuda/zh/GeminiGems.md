# 🔵 与 Antigravity CLI (agy) 集成

Google Antigravity 是 Google 的现代高级开发平台，而 **Antigravity CLI (`agy`)** 是其官方终端命令行工具，正式取代了先前的 Gemini CLI。MMCelt 支持直接连接 Antigravity CLI，帮助您分析、拓展并丰富思维导图。

### 💻 在终端中使用 Antigravity CLI (`agy`)

1. 进入 `🤖 人工智能` → `🔌 将MMCelt连接到我的AI...` 并点击 Antigravity CLI (agy) 旁的**连接**按钮。
2. 打开导图，选择 `🤖 人工智能` → `📤 发送到...` → `Antigravity CLI (agy)`。
3. 审阅将要发送给 AI 的文档与任务要求预览。
4. 点击**在控制台中启动**。MMCelt 会在准备好的环境中自动打开终端运行 `agy -i`，并在后台监视文件变更，以便通过 MCP 将智能体创建的节点同步回导图中。

### 💡 使用标准 Google 账户认证

Antigravity CLI 原生支持您的**标准个人 Google 账户**：
- 无需配置 Google AI Studio API 密钥或额外环境变量。
- 若尚未登录 `agy`，该工具将自动调起浏览器，供您使用普通 Google 账户完成登录。
- MMCelt 的 MCP 服务器配置将写入 `~/.gemini/config/mcp_config.json`，使智能体能够在授权的项目目录范围内读写导图。

### 🌐 在网页浏览器中使用 Gemini

如果您不想使用终端：
1. 通过 `📁 文件` → `🤖 导出面向 AI 的 Markdown (.md)` 导出导图（或点击 `🤖 人工智能` → `👁️ 预览AI Markdown (.md)...`）。
2. 在浏览器中打开 Gemini 官网，粘贴文本并输入您的要求。
3. 复制 Gemini 生成的回复并返回 MMCelt：点击 `🤖 人工智能` → `📥 从AI导入（ChatGPT、Claude、Gemini）...`，即可将新的分支融入您的导图中。
