#[derive(Debug)]
pub enum Instruction {
    Const(f64),
    Mul,
    Add,
}

pub struct Machine {
    stack: Vec<f64>,
    memory: Vec<u8>,
}

impl Machine {
    pub fn new(mem_size: usize) -> Self {
        Machine {
            stack: Vec::new(),
            memory: Vec::with_capacity(mem_size),
        }
    }

    pub fn push(&mut self, item: f64) {
        self.stack.push(item);
    }

    pub fn pop(&mut self) -> Option<f64> {
        self.stack.pop()
    }

    pub fn execute(&mut self, instructions: Vec<Instruction>) {
        for instruction in instructions {
            println!("Op: {:?}, Stack: {:?}", instruction, self.stack);
            match instruction {
                Instruction::Const(item) => self.push(item),
                Instruction::Add => {
                    let right = self.pop().unwrap();
                    let left = self.pop().unwrap();
                    self.push(left + right);
                }
                Instruction::Mul => {
                    let right = self.pop().unwrap();
                    let left = self.pop().unwrap();
                    self.push(left * right);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        let code = vec![
            Instruction::Const(2.0),
            Instruction::Const(3.0),
            Instruction::Const(0.1),
            Instruction::Mul,
            Instruction::Add,
        ];

        let mut m = Machine::new();
        m.execute(code);
        println!("Result: {}", m.pop().unwrap());
    }
}
