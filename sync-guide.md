# Zed 代码同步指南

本指南用于将 fork 的 Zed 项目与官方仓库同步。

## 项目背景与定制说明

* **基线版本**：本项目基于 Zed 官方 **`v1.9.0`** 版本 Fork，作为核心代码基线进行自主定制、功能增强与独立打包。
* **自主维护与定制的核心原因**：
  * **官方改动痛点**：
    官方在 **`v1.10.0`** 重构了 左右Diff 视图（支持切换“全文件（Full File）”与“仅改动片段（Hunks）”），将右侧对比编辑器改为了**片段摘录模式的多缓冲拼接（`MultiBuffer::without_headers` + `set_excerpts_for_buffer`）**。
  * **引发的架构与交互缺陷**：
    由于非单文件缓冲（`is_singleton() == false`），Zed 内部的所有核心导航与编辑器能力（如 LSP 变量跳转 `Go To Definition`、Minimap 缩略图、Outline 等）均将该视图判定为“临时对比摘录”。**在并排 Diff 中进行代码跳转时，会强行逃逸出当前 Diff 窗口，转而在常规的代码编辑器标签页中打开**，严重打断了代码审阅与排错时的连续心流。
  * **本项目的解法与定制优化**：
    1. **重构 `SoloDiffView` 为 Singleton 模式**：右侧改用原生的 `MultiBuffer::singleton(buffer, cx)` 并直接挂载 Diff 信息（`add_diff` + `set_all_diff_hunks_expanded`），确保右侧编辑器始终具备标准单文件缓冲区的全套语言服务与交互能力。
    2. **实现 Diff 上下文无缝跳转**：代码跳转与符号查找直接在当前并排 Diff 视图内原地定位与跳转，不再被踢回常规编辑器。
    3. **性能与逻辑轻量化**：免去了频繁计算、裁剪及动态增删片段（`excerpt_ranges`）的额外开销，大幅提升 Diff 渲染与交互的流畅度。
  * **权衡说明**：
    * 官方方案：支持 Full File / Hunks 切换，但牺牲了单文件缓冲的完整语言服务与交互能力。
    * 本项目方案：保留完整单文件缓冲体验（LSP 跳转、Minimap、Outline 等全部可用），但放弃了 Hunks 切换功能。

## 同步流程

### 1. 备份当前修改（可选）

如果你当前的修改想要保留，先创建一个备份分支：

```bash
# 创建备份分支
git checkout -b backup-my-changes

# 提交当前修改
git add .
git commit -m "backup: my local changes before sync"
```

### 2. 重置并同步官方最新代码

```bash
# 切换回 main 分支
git checkout main

# 重置所有本地修改
git reset --hard origin/main

# 添加/更新 upstream 远程仓库
git remote add upstream https://github.com/zed-industries/zed.git || git remote set-url upstream https://github.com/zed-industries/zed.git

# 获取官方最新代码
git fetch upstream main

# 合并官方 main 分支的最新代码
git merge upstream/main

# 推送到你的 fork 仓库
git push origin main
```

### 3. 创建新的开发分支（可选）

```bash
# 基于同步后的 main 创建新分支
git checkout -b sync-with-upstream
```

## 快速执行脚本

将以下命令复制到终端执行：

```bash
# 备份当前分支
git checkout -b backup-my-changes
git add .
git commit -m "backup: my local changes before sync"

# 切换回 main 并重置
git checkout main
git reset --hard origin/main

# 添加/更新 upstream 并同步
git remote add upstream https://github.com/zed-industries/zed.git || git remote set-url upstream https://github.com/zed-industries/zed.git
git fetch upstream main
git merge upstream/main
git push origin main
```

## 注意事项

- ⚠️ 执行 `git reset --hard` 会丢失所有未提交的修改，请先备份
- ⚠️ 合并时可能会有冲突，需要手动解决
- ✅ 同步后可以通过 `git log` 查看最新的提交历史
- ✅ 可以通过 `git diff upstream/main` 查看与官方的差异

## 常用命令

```bash
# 查看远程仓库
git remote -v

# 查看分支
git branch -a

# 查看当前状态
git status

# 查看提交历史
git log --oneline -10
```
