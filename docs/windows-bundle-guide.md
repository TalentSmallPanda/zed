# Zed Windows 打包指南

本文档记录在 Windows 上打包 Zed 编辑器的完整流程，包括环境配置、脚本修改和执行步骤。

## 目录

- [环境要求](#环境要求)
- [环境检查](#环境检查)
- [脚本修改](#脚本修改)
- [执行打包](#执行打包)
- [常见问题](#常见问题)

---

## 环境要求

### 必需软件

1. **Rust 工具链** (1.95.0+)
   ```powershell
   rustc --version  # 应显示 rustc 1.95.0+
   cargo --version
   ```

2. **Visual Studio 2022/2026 BuildTools**
   - 包含 C++ 工具链
   - Windows SDK（包含 `rc.exe` 资源编译器）
   - Universal CRT SDK

3. **Inno Setup 6**
   - 用于生成 Windows 安装程序 (.exe)
   - 下载地址：https://jrsoftware.org/isdl.php

4. **PowerShell 5.1+**
   - Windows 自带

### 可选软件

- **PowerShell 7** - 如果使用 PowerShell 7，某些脚本语法更现代
- **Git Bash** - 用于 Unix 风格的命令行操作

---

## 环境检查

### 1. 检查 Rust 工具链

```powershell
rustc --version
cargo --version
```

### 2. 检查 Visual Studio

```powershell
# 检查 VS BuildTools 路径
ls "D:\Program Files (x86)\Microsoft Visual Studio\18\BuildTools\Common7\Tools\Launch-VsDevShell.ps1"

# 检查 Windows SDK (rc.exe)
ls "C:\Program Files (x86)\Windows Kits\10\bin\10.0.26100.0\x64\rc.exe"

# 检查 Universal CRT
ls "C:\Program Files (x86)\Windows Kits\10\Lib\10.0.26100.0\ucrt\x64\ucrt.lib"
```

### 3. 检查 Inno Setup

```powershell
# 默认安装路径
ls "C:\Program Files (x86)\Inno Setup 6\ISCC.exe"

# 或用户目录安装
ls "C:\Users\<你的用户名>\AppData\Local\Programs\Inno Setup 6\ISCC.exe"
```

---

## 脚本修改

### 修改 `script/bundle-windows.ps1`

#### 1. 修改 VS BuildTools 路径（第 43-51 行）

**原代码：**
```powershell
Push-Location
& "C:\Program Files\Microsoft Visual Studio\2022\Community\Common7\Tools\Launch-VsDevShell.ps1" -Arch (Get-VSArch -Arch $Architecture) -HostArch (Get-VSArch -Arch $OSArchitecture)
Pop-Location
```

**修改后（适配你的路径）：**
```powershell
Push-Location
# Modified for VS 2026 BuildTools on D: drive
$vsDevShellPath = "D:\Program Files (x86)\Microsoft Visual Studio\18\BuildTools\Common7\Tools\Launch-VsDevShell.ps1"
if (Test-Path $vsDevShellPath) {
    & $vsDevShellPath -Arch (Get-VSArch -Arch $Architecture) -HostArch (Get-VSArch -Arch $OSArchitecture)
} else {
    Write-Error "VS Dev Shell not found at: $vsDevShellPath"
    exit 1
}
Pop-Location
```

#### 2. 修改 Inno Setup 路径（第 335-343 行）

**原代码：**
```powershell
$innoSetupPath = "C:\Program Files (x86)\Inno Setup 6\ISCC.exe"
```

**修改后（适配你的路径）：**
```powershell
# Modified for local Inno Setup installation
# Default path: C:\Program Files (x86)\Inno Setup 6\ISCC.exe
# Your path: C:\Users\05637\AppData\Local\Programs\Inno Setup 6\ISCC.exe
$innoSetupPath = "C:\Users\05637\AppData\Local\Programs\Inno Setup 6\ISCC.exe"
if (-not (Test-Path $innoSetupPath)) {
    Write-Error "Inno Setup not found at: $innoSetupPath"
    Write-Output "Please install Inno Setup 6 or update the path in bundle-windows.ps1"
    exit 1
}
```

#### 3. 修复 PowerShell 5.1 兼容性（`script/generate-licenses.ps1`）

**第 5 行，原代码：**
```powershell
$outputFile=$args[0] ? $args[0] : "$(Get-Location)/assets/licenses.md"
```

**修改后：**
```powershell
if ($args[0]) {
    $outputFile = $args[0]
} else {
    $outputFile = "$(Get-Location)/assets/licenses.md"
}
```

**第 37 行，原代码：**
```powershell
$failFlag = $env:ALLOW_MISSING_LICENSES ? "--fail" : ""
```

**修改后：**
```powershell
if ($env:ALLOW_MISSING_LICENSES) {
    $failFlag = "--fail"
} else {
    $failFlag = ""
}
```

#### 4. 修复 .nupkg 解压问题（第 242-247 行）

**原代码：**
```powershell
function DownloadConpty {
    $url = "https://github.com/microsoft/terminal/releases/download/v1.23.13503.0/Microsoft.Windows.Console.ConPTY.1.23.251216003.nupkg"
    $zipPath = ".\Microsoft.Windows.Console.ConPTY.1.23.251216003.nupkg"
    Invoke-WebRequest -Uri $url -OutFile $zipPath
    Expand-Archive -Path $zipPath -DestinationPath ".\conpty" -Force
}
```

**修改后：**
```powershell
function DownloadConpty {
    $url = "https://github.com/microsoft/terminal/releases/download/v1.23.13503.0/Microsoft.Windows.Console.ConPTY.1.23.251216003.nupkg"
    $zipPath = ".\Microsoft.Windows.Console.ConPTY.1.23.251216003.nupkg"
    $tempZipPath = ".\Microsoft.Windows.Console.ConPTY.1.23.251216003.zip"
    Invoke-WebRequest -Uri $url -OutFile $zipPath
    # .nupkg is actually a zip file, rename it to .zip for Expand-Archive
    Rename-Item -Path $zipPath -NewName $tempZipPath -Force
    Expand-Archive -Path $tempZipPath -DestinationPath ".\conpty" -Force
    Remove-Item -Path $tempZipPath -Force
}
```

---

## 执行打包

### 方式 1：使用修改后的脚本（推荐）

```powershell
# 进入项目目录
cd e:\Rust\zed

# 执行打包（管理员权限 PowerShell）
.\script\bundle-windows.ps1 -Architecture x86_64

# 或带自动安装选项
.\script\bundle-windows.ps1 -i -Architecture x86_64
```

### 方式 2：使用 Git Bash

```bash
cd /e/Rust/zed
powershell.exe -ExecutionPolicy Bypass -File "./script/bundle-windows.ps1" -Architecture x86_64
```

### 输出位置

打包成功后，安装程序位于：
```
e:\Rust\zed\target\Zed-x86_64.exe
```

---

## 完整执行流程示例

### 1. 环境准备检查

```powershell
# 检查 Rust
rustc --version  # rustc 1.95.0
cargo --version  # cargo 1.95.0

# 检查 VS BuildTools
ls "D:\Program Files (x86)\Microsoft Visual Studio\18\BuildTools\Common7\Tools\Launch-VsDevShell.ps1"

# 检查 Windows SDK
ls "C:\Program Files (x86)\Windows Kits\10\bin\10.0.26100.0\x64\rc.exe"

# 检查 Inno Setup
ls "C:\Users\05637\AppData\Local\Programs\Inno Setup 6\ISCC.exe"
```

### 2. 执行打包

```powershell
cd e:\Rust\zed
.\script\bundle-windows.ps1 -Architecture x86_64
```

### 3. 查看打包结果

```powershell
# 查看安装包
ls target\Zed-x86_64.exe

# 查看安装包大小
(dir target\Zed-x86_64.exe).Length / 1MB  # 显示 MB 大小
```

### 4. 安装 Zed

```powershell
# 双击运行安装程序
.\target\Zed-x86_64.exe

# 或使用 PowerShell
Start-Process .\target\Zed-x86_64.exe
```

---

## 常见问题

### 1. PowerShell 版本过低

**错误信息：**
```
表达式或语句中包含意外的标记"?"
```

**解决方案：**
修改 `script/generate-licenses.ps1`，将三元运算符改为 `if` 语句（见上文脚本修改部分）。

### 2. 缺少 Windows SDK

**错误信息：**
```
无法找到 rc.exe
```

**解决方案：**
安装 Windows SDK：
1. 打开 Visual Studio Installer
2. 修改 VS BuildTools 安装
3. 勾选 "Windows 10/11 SDK"
4. 安装

### 3. .nupkg 解压失败

**错误信息：**
```
Expand-Archive : .nupkg 不是支持的存档文件格式
```

**解决方案：**
修改 `script/bundle-windows.ps1` 中的 `DownloadConpty` 函数，将 `.nupkg` 重命名为 `.zip` 后解压（见上文脚本修改部分）。

### 4. VS BuildTools 路径不正确

**错误信息：**
```
VS Dev Shell not found at: ...
```

**解决方案：**
修改 `script/bundle-windows.ps1` 第 45 行，使用正确的 VS 路径。

### 5. Inno Setup 未找到

**错误信息：**
```
Inno Setup not found at: ...
```

**解决方案：**
1. 安装 Inno Setup 6：https://jrsoftware.org/isdl.php
2. 或修改脚本第 338 行，使用实际的 Inno Setup 安装路径

---

## 环境变量说明

打包脚本会自动设置以下环境变量，**无需手动配置**：

- `ZED_WORKSPACE` - 自动从 Cargo metadata 读取
- `RELEASE_VERSION` - 自动从 Cargo.toml 读取
- `ZED_RELEASE_CHANNEL` - 从 `crates/zed/RELEASE_CHANNEL` 读取（当前是 `dev`）

---

## 打包产物

打包完成后，会生成以下文件：

| 文件 | 位置 | 说明 |
|------|------|------|
| `Zed-x86_64.exe` | `target/` | Windows 安装程序 |
| `zed.exe` | 安装目录 | Zed 编辑器主程序 |
| `cli.exe` | 安装目录 | 命令行工具 |
| `remote_server.exe` | `target/` | 远程服务器二进制（zip 压缩） |

---

## 参考链接

- [Zed 官方文档](https://zed.dev/docs)
- [Windows 开发指南](https://zed.dev/docs/development/windows)
- [Inno Setup 文档](https://jrsoftware.org/ishelp/)
- [Visual Studio BuildTools](https://visualstudio.microsoft.com/downloads/#build-tools-for-visual-studio-2022)

---

**最后更新时间：** 2026-08-10  
**适用版本：** Zed Dev Channel  
**测试环境：** Windows 11 + VS 2026 BuildTools + Rust 1.95.0
