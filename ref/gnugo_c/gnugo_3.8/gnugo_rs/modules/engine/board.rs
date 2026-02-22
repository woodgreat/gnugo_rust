   //! 对应原engine/board.c
   #[derive(Debug)]
   pub struct Board {
       pub size: u32,
       // 其他字段与C结构体一致
   }

   impl Board {
       /// 等效于C的`board_init()`
       pub fn new(size: u32) -> Self {
           Self { size }
       }
   }
   