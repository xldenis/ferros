enum Node {
  USED   = 0,
  UNUSED = 1,
  FREE   = 2,
  SPLIT  = 3
} 


struct BuddyAlloc {
  pub order: uint,
  pub tree: Bitv
}
fn alloc(&mut self, size) {
  size = 32 - unsafe { ctlz32(size as u32 - 1) } as uint;

  let mut level = self.order;
  let mut index = 0;
  loop {
    match(get(index), level == size) {

      (UNUSED, true) => {
        //use this
        self.set(index, USED);
        self.offset(base, index);
      }
      (UNUSED, false) => {
        // this node is large and unused, split it
        self.set(index, SPLIT);
        self.set(2*index, UNUSED);
        self.set(2*index+1, UNUSED);
        level -= 1;
        index = 2*index;
      }
      (SPLIT, false) => {
        // too large but allocated follow it down
        level -= 1;
        index = 2*index;
      }
      _ => loop {
        // left child
        if index & 1 == 1 {
          index += 1
          break;
        }
        //up
        level += 1;

        if index == 0 {
          return (0, 0);
        }
        //move ptr up
        index = (index + 1) / 2 - 1;
      }
    }
  }
}

fn free(&mut self, offset: uint) {
  let mut length 1 << self.order;
  let mut left = 0;
  let mut index = 0;

  loop {
    match self.get(index) {
      UNUSED => return,
      USED => loop {
        if index == 0 {
          self.set(0, UNUSED)
        }

        let buddy = index - 1 + (index & 1) * 2
        match self.get(buddy) {
          UNUSED => {},
          _ => {
            self.set(index, UNUSED)
            loop {
              let parent = (index + 1) / 2 - 1;
              match self.get(parent) {
                FULL if index > 0 {
                  self.set(parent, SPLIT)
                },
                _ => return
              }
              index = parent
            }
          }
        }
        index = (index + 1) / 2 - 1; 
      },
      _ => {
        length /= 2;

        if offset < left + length {
          // go left
          index = index * 2 + 1;
        } else {
          // go right
          index = index * 2 + 2;
          left += length; 
        }

      }
    }
  }
}

fn get(&mut self, i: uint) -> Node {
  unsafe {
    transmute(self.tree.get(i))
  }
}

fn set(&mut self, i: uint, v: Node) {
  self.tree.set(i, v as u8);
}