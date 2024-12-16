struct MinStack {
    array: Vec<i32>,
    min: Vec<i32>
}

impl MinStack {
    fn new() -> Self {
        MinStack{array: vec![], min: vec![]}
    }
    
    fn push(&mut self, val: i32) {
        self.array.push(val);
        if self.min.is_empty() || val <= self.get_min() {
            self.min.push(val)
        }
    }
    
    fn pop(&mut self) {
        if self.array.is_empty() {return;}

        if *self.array.last().unwrap() == self.get_min() {
            self.min.pop();
        }
        self.array.pop();
    }
    
    fn top(&self) -> i32 {
        *self.array.last().unwrap()
    }
    
    fn get_min(&self) -> i32 {
        *self.min.last().unwrap_or(&i32::MIN)
    }
}

fn main() {
    let mut min_stack = MinStack::new();
    min_stack.push(3);
    min_stack.push(4);
    min_stack.push(1);
    min_stack.pop();
    min_stack.pop();

    println!("{:?}", min_stack.array);
    println!("{:?}", min_stack.min);
}