# Zed 代码同步指南

本指南用于将 fork 的 Zed 项目与官方仓库同步。

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
