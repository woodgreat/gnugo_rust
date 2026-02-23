# 📜 GNU Go Rust GTP 命令手册 (v0.2.0)

*最后更新: 2026-02-23*

```text
GNU Go Rust 是一个兼容GTP协议的围棋引擎，支持标准19x19棋盘及SGF文件交互。
```

## 🏷️ 基础信息命令

### `protocol_version`
```bash
# 返回支持的GTP协议版本
= 2
```

### `name` 
```bash
# 返回引擎名称
= gnugo_rs
```

### `version`
```bash
# 返回版本信息  
= 0.2.0
```

## 🎮 对弈控制命令

### `boardsize [9|13|19]`
```bash
# 设置棋盘大小 (默认19)
boardsize 9
= 
```

### `clear_board`
```bash
# 清空棋盘并重置游戏
= 
```

### `komi [6.5]`
```bash
# 设置贴目值 (默认6.5)
komi 7.5
=
```

### `get_komi`
```bash 
# 获取当前贴目值
= 7.5
```

## ⚔️ 对弈命令

### `play <color> <move>`
```bash
# 落子 (B/W + 坐标 或 pass)
play B D4
=
play W pass
=
```

### `genmove <color>` 
```bash
# 电脑生成一步棋
genmove B
= E3
```

### `undo`
```bash
# 撤销上一步
= 
```

### `final_score`
```bash
# 获取最终得分 (需游戏结束)
= B+3.5
```

## 🏗️ 棋盘状态命令

### `showboard`
```bash
# 显示当前棋盘状态 (文本格式)
= 
  A B C D E F G H J
9 . . . . . . . . . 9
8 . . . . . . . . . 8
...
```

### `list_stones <color>`
```bash 
# 列出所有指定颜色棋子
list_stones black
= D4 E3 F5
```

### `is_legal <color> <move>`
```bash
# 检查落子是否合法 (返回1/0)
is_legal white E5
= 1
```

## 🔍 分析命令

### `countlib <move>`
```bash
# 计算指定位置的气数
countlib D4
= 3
```

### `findlib <n>`
```bash
# 查找有n口气的所有位置
findlib 1
= E4 F3
```

### `eye_data <color> <move>`
```bash
# 获取眼位分析数据
eye_data black E4
= origin 4 4
  color black
  esize 2
  ...
```

## 📁 SGF文件命令

### `loadsgf <filename>`
```bash
# 加载SGF文件
loadsgf game.sgf
=
```

### `printsgf [filename]`
```bash
# 导出当前棋局为SGF
printsgf output.sgf
= 
# 或打印到标准输出
printsgf
= (;FF[4]GM[1]SZ[19]...)
```

## 🛠️ 系统命令

### `list_commands` / `list` / `help`
```bash
# 列出所有可用命令
= protocol_version
name
version
...
```

### `known_command <cmd>`
```bash
# 检查命令是否存在
known_command play
= true
```

### `quit` / `exit`
```bash
# 退出程序
=
```

## 🎯 特殊功能命令

### `ladder_attack <move>`
```bash
# 梯子攻击分析
ladder_attack E4
= 1 F5  # 可攻击
```

### `time_settings <main_time> <byo_time> <byo_stones>`
```bash
# 设置计时器 (暂未实现)
=
```

## 📌 使用示例

### 基础对局流程
```bash
boardsize 9
clear_board
komi 6.5
play black D4
genmove white
showboard
```

### SGF文件操作
```bash
loadsgf opening.sgf
printsgf current.sgf
```

### 分析模式
```bash
is_legal black E5
countlib E5
eye_data white E5
```

---

```text
注意: 
1. 所有命令返回空字符串表示成功
2. 错误响应以"?"开头
3. 坐标格式: 大写字母+数字 (如 "D4")
```

