# ℹ️ Version and Build

### 👀 Where to see it
Version is displayed in **window title** and **status bar**, alongside build timestamp.

For full diagnostics: **❓ Help → ℹ️ About MMCelt**, or click the version in status bar. Displays version, build date, commit, branch, and — most useful — **the exact path of the running executable**.

### 💻 From terminal
```
mmcelt --version
```

Useful when multiple binaries exist on disk to verify active build without opening UI.

### 🤔 Why is version number alone insufficient?
Version number remains identical across iterative builds. It states intended target release, not **which concrete binary** was launched.

When managing local builds, portable USB copies, and working directories, file paths, commit hashes, and timestamps reliably differentiate binaries.

### ⚠️ "Uncommitted changes" notice
If shown, the executable was compiled with uncommitted repository modifications: that binary does not map to a clean git commit.

### ⬆ New version notice
At startup, MMCelt asks GitHub for the latest published version. If it is newer than yours, the top bar shows `⬆ New version available:` with the version number. It is a link: clicking it opens that version's page in your browser. **The program does not download or install anything**; updating is your decision.

If there is no connection or GitHub does not answer, nothing happens: there is simply no notice. To stop the check, untick `Check for a new version at startup` in the `🎨 View and Design` menu; when it is off, the program does not connect to the internet at startup.
