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
