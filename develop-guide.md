# GNU Go Rust 开发指引

## 项目概述

本项目是基于 GNU Go 3.8 的 Rust 重写版本，目标是实现一个功能完整的围棋引擎，支持 GTP 协议和 SGF 文件格式。

---

## 当前状态

### 已实现功能

- [x] 基础棋盘表示与落子规则
- [x] 提子与气计算
- [x] 打劫规则（基础实现）
- [x] GTP 协议支持
- [x] SGF 文件读写
- [x] 终端 UI
- [x] 基础模式匹配框架

### 主要问题（按优先级排序）

#### 1. 高优先级 - 核心功能缺失

| 问题 | 位置 | 说明 |
|------|------|------|
| **模式匹配存根实现** | `patterns/pattern_matching.rs:103-110` | `pattern_matches()` 永远返回 `true`，未实现真正的模式匹配逻辑 |
| **AI 高级难度未实现** | `engine/ai.rs:35` | `Advanced` 难度直接调用 `greedy_move`，未实现 minimax/alpha-beta 搜索 |
| **模式数据库加载** | `patterns/pattern_database.rs:94-98` | 硬编码路径加载 `.db` 文件，实际文件可能不存在 |

#### 2. 中优先级 - 功能不完善

| 问题 | 位置 | 说明 |
|------|------|------|
| **影响力计算效率低** | `engine/evaluation.rs:124-140` | `calculate_influence()` 使用双重循环，时间复杂度 O(n⁴) |
| **终局计分简化** | `engine/game.rs:186-201` | `determine_winner()` 只计算棋子数+提子数，未实现真正的领地计算 |
| **死活判断不完整** | `engine/eye.rs` | 只有基础眼形检测，缺少完整的死活分析 |

#### 3. 低优先级 - 代码质量

| 问题 | 位置 | 说明 |
|------|------|------|
| **未使用的导入** | `sgf/mod.rs:8,11` | `self` 和 `Board` 导入未使用 |
| **GTP 参数未使用** | `gtp/mod.rs:154,178` | `play` 和 `genmove` 中颜色参数解析后未使用（`_stone` 前缀） |

---

## 开发建议

### 阶段一：核心引擎（优先级最高）

#### 1.1 实现真正的模式匹配

**目标**：替换 `pattern_matching.rs` 中的存根实现

```rust
// 当前存根实现
fn pattern_matches(&self, _board: &Board, _row: usize, _col: usize, _pattern_id: u32) -> bool {
    true  // 永远返回 true，无实际意义
}

// 需要实现：
// 1. 定义模式的数据结构（如 3x3、5x5 的模板）
// 2. 实现模板与棋盘的匹配算法
// 3. 支持旋转、翻转等变换
// 4. 从 .db 文件加载真实模式数据
```

**参考**：原版 GNU Go 的 `patterns/` 目录下的模式定义

#### 1.2 实现搜索算法

**目标**：完成 AI 的 `Advanced` 难度

```rust
// engine/ai.rs
AIDifficulty::Advanced => {
    // TODO: 实现 minimax + alpha-beta 剪枝
    // 或蒙特卡洛树搜索（MCTS）
    self.minimax_move(board, player, depth)
}
```

**建议步骤**：
1. 先实现基础的 minimax 搜索
2. 添加 alpha-beta 剪枝优化
3. 考虑实现 MCTS（更现代的做法）

### 阶段二：规则完善

#### 2.1 完善终局计分

**目标**：实现中国规则或日本规则的完整计分

```rust
// engine/game.rs
determine_winner() {
    // 当前：只计算棋子数
    // 需要：实现领地计算（围住的空点）
    // 需要：处理死子提掉后的领地
}
```

#### 2.2 完善死活判断

**目标**：实现完整的死活分析

```rust
// engine/eye.rs
// 当前：简单的眼形检测
// 需要：
// 1. 识别真正的眼（对方无法破坏）
// 2. 识别假眼
// 3. 判断棋块的死活状态
```

### 阶段三：性能优化

#### 3.1 优化影响力计算

```rust
// engine/evaluation.rs
// 当前：双重循环 O(n⁴)
// 优化：使用卷积或距离变换算法
```

#### 3.2 添加缓存机制

```rust
// 为模式匹配结果、眼形分析结果添加缓存
// 避免重复计算
```

---

## 代码规范

### 命名约定

- 模块名：`snake_case`（如 `pattern_matching`）
- 结构体/枚举名：`PascalCase`（如 `PatternMatcher`）
- 函数/变量名：`snake_case`（如 `find_patterns`）
- 常量名：`SCREAMING_SNAKE_CASE`（如 `MAX_BOARD_SIZE`）

### 注释规范

```rust
//! 模块级文档注释（放在文件开头）

/// 函数/结构体文档注释
/// 
/// # 参数
/// - `board`: 棋盘状态
/// - `color`: 执子方
/// 
/// # 返回值
/// 返回最佳落子位置，如果没有则返回 None
pub fn find_best_move(board: &Board, color: Stone) -> Option<(usize, usize)> {
    // 行内注释：解释复杂的逻辑
    let score = evaluate(board); // 评估当前局面
    
    // TODO: 标记待完成的任务
    // FIXME: 标记需要修复的问题
}
```

### 错误处理

```rust
// 使用 Result 返回错误
pub fn place_stone(&mut self, x: usize, y: usize, stone: Stone) -> Result<(), &'static str> {
    if x >= self.size || y >= self.size {
        return Err("Position out of bounds");
    }
    // ...
}

// 避免使用 unwrap()，使用 ? 或 match 处理
```

---

## 测试策略

### 单元测试

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capture() {
        let mut board = Board::new(9);
        // 设置提子局面
        board.set_stone(0, 1, Stone::Black);
        board.set_stone(1, 0, Stone::Black);
        board.set_stone(1, 2, Stone::Black);
        board.set_stone(2, 1, Stone::Black);
        board.set_stone(1, 1, Stone::White);
        
        // 验证提子
        assert!(board.place_stone(1, 1, Stone::Black).is_ok());
        assert_eq!(board.get_stone(1, 1), Stone::Black);
        assert_eq!(board.get_captured()[0], 1); // 提了1个白子
    }
}
```

### 集成测试

- 测试 GTP 命令序列
- 测试 SGF 文件读写
- 测试完整对局流程

### 测试数据

项目根目录已提供一些 SGF 测试文件：
- `simple_test.sgf`
- `test.sgf`
- `multi_move_test.sgf`
- `error_test.sgf`

---

## 参考资源

### 原版 GNU Go

- 源码位置：`ref/gnugo_c/`
- 重点关注：
  - `engine/` - 核心引擎
  - `patterns/` - 模式匹配
  - `interface/gtp.c` - GTP 协议实现

### 围棋算法资料

1. **模式匹配**：参考原版 `patterns.c` 和 `patterns.h`
2. **搜索算法**：
   - Minimax + Alpha-Beta
   - Monte Carlo Tree Search (MCTS)
3. **死活判断**：参考原版 `owl.c`（猫头鹰模块）

---

## 提交规范

### 提交信息格式

```

<日期>.>.<版本> <类型>

类型说明：
- p  - 进展（progress）
- pf - 进展+修复（progress+fix）
- f  - 修复（fix）
- r  - 重构（refactor）
- d  - 文档（documentation）

示例：
2026.02.23g9 p  - 第9个进展提交
2026.02.23g7 pf - 第7个提交，包含进展和修复
```

---

## 常见问题

### Q: 什么是"存根实现"？

A: 存根（Stub）是一种占位符实现，函数有定义但只返回默认值，用于：
- 让代码编译通过
- 定义接口供后续填充
- 并行开发时约定契约

与虚函数的区别：Rust 没有强制要求覆盖存根，需要开发者自觉完成。

### Q: 如何运行测试？

```bash
cd gnugo-rs
cargo test           # 运行所有测试
cargo test -- --nocapture  # 显示打印输出
cargo test test_name # 运行特定测试
```

### Q: 如何启用 ko 规则测试？

```bash
cargo run --features ko_test -- --test-ko
```

---

## 联系方式

项目维护者：wood & zulu_ai
许可证：GPL-3.0-or-later
