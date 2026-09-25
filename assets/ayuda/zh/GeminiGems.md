# 🔵 与 Gemini CLI 集成

Gemini 是 Google 开发的人工智能模型，而 **Gemini CLI** 是其官方终端命令行工具。MMCelt 支持直接连接 Gemini CLI，帮助您分析、拓展并丰富思维导图。

### 💻 在终端中使用 Gemini CLI

1. 进入 `🤖 人工智能` → `🔌 将MMCelt连接到我的AI...` 并连接 Gemini CLI。
2. 打开导图，选择 `🤖 人工智能` → `📤 发送到...` → `Gemini CLI`。
3. 审阅将要发送给 AI 的文档与任务要求预览。
4. 点击**在控制台中启动**。MMCelt 会在准备好的环境中自动打开终端，并在后台监视文件变更，以便将 Gemini 创建的节点同步回导图中。

### 💡 技巧与建议：免费 API 密钥与兼容性

如果在运行 Gemini CLI 时控制台提示身份验证或浏览器登录遇到阻碍，最快速、直接且可靠的方法是使用来自 Google AI Studio 的**免费 API 密钥**：

1. 使用您的 Google 账号登录 Google AI Studio (`https://aistudio.google.com/app/apikey`)，点击 **Create API Key**（创建 API 密钥）。该服务完全免费。
2. 复制生成的密钥。
3. 在启动终端前或在系统用户配置文件中设置环境变量：
   - **在 Windows (PowerShell) 中：**
     ```powershell
     $env:GEMINI_API_KEY="您的密钥"
     ```
   - **在 Linux 或 macOS (Bash/Zsh) 中：**
     ```bash
     export GEMINI_API_KEY="您的密钥"
     ```
4. 设置该变量后，Gemini CLI 将直接正常运行，无需再在浏览器中进行额外的登录验证。
5. 您可以在官方代码仓库中查看 Gemini CLI 的完整文档：`https://github.com/google-gemini/gemini-cli`。

**ℹ️ Gemini CLI 当前账号兼容性现状：**
已确认：自 2026 年 6 月 18 日起，Google 已在 Gemini CLI 中取消个人账号（包含免费账号、Google AI Pro 及 Ultra 账号）的交互式 "Sign in with Google" 登录方式。
- **2026年9月13日：** 在总监的电脑上首次测试，终端客户端输出了如下提示：*«This client is no longer supported for Gemini Code Assist for individuals. To continue using Gemini, please migrate to the Antigravity suite of products»*。
- **随后在多个较旧版本的 Gemini CLI 上重复测试**，以排除具体版本导致的问题：所有版本结果一致。
- **已通过 Google AI Studio 免费 API 密钥确认：** 连接全程成功运行，在磁盘上实际创建并验证了新节点与连线，且不受个人账号任何限制。

前文所述的 API 密钥方案**是目前个人账号唯一确认可行的途径**：交互式登录已不再对个人账号开放。

### 🌐 在网页浏览器中使用 Gemini

如果您不想使用终端：
1. 通过 `📁 文件` → `🤖 导出面向 AI 的 Markdown (.md)` 导出导图（或点击 `🤖 人工智能` → `👁️ 预览AI Markdown (.md)...`）。
2. 在浏览器中打开 Gemini 官网，粘贴文本并输入您的要求。
3. 复制 Gemini 生成的回复并返回 MMCelt：点击 `🤖 人工智能` → `📥 从AI导入（ChatGPT、Claude、Gemini）...`，即可将新的分支融入您的导图中。
