use std::collections::HashMap;
use std::convert::TryInto;

#[derive(Debug)]
pub enum Instruction {
    Const(f64),
    Mul,
    Add,
    Load,
    Store,
    Function(usize, HashMap<usize, &f64>)
}

pub struct Function {
    nparams: usize,
    returns: bool,
    code: usize,
}

impl Function {
    pub fn new(nparams: usize, returns: bool, code: usize) -> Self {
        Function {
            nparams,
            returns,
            code,
        }
    }
}
pub struct Machine {
    stack: Vec<f64>,
    memory: Vec<u8>,
}

impl Machine {
    pub fn new(mem_size: usize) -> Self {
        Machine {
            stack: Vec::new(),
            memory: vec![0; mem_size],
        }
    }

    pub fn load(&mut self, addr: usize) -> f64 {
        f64::from_le_bytes(self.memory[addr..addr + 8].try_into().unwrap())
    }

    pub fn store(&mut self, addr: usize, val: f64) {
        self.memory[addr..addr + 8].copy_from_slice(&val.to_le_bytes());
    }

    pub fn push(&mut self, item: f64) {
        self.stack.push(item);
    }

    pub fn pop(&mut self) -> Option<f64> {
        self.stack.pop()
    }

    pub fn call(&mut self, func: Function, args: Vec<f64>) -> Option<f64> {
        let mut locals = HashMap::new();
        args.iter().enumerate().for_each(|(index, val)| {
            locals.insert(index, val);
        });

        self.execute(vec![Instruction::Function(func.code, locals)]);

        if func.returns {
            self.pop()
        } else {
            None
        }
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
                Instruction::Load => {
                    let addr = self.pop().unwrap();
                    let val = self.load(addr as usize);
                    self.push(val);
                }
                Instruction::Store => {
                    let val = self.pop().unwrap();
                    let addr = self.pop().unwrap();
                    self.store(addr as usize, val)
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

        let mut m = Machine::new(100);
        m.execute(code);
        println!("Result: {}", m.pop().unwrap());
    }
    #[test]
    fn example_variables() {
        let x_addr = 22.0;
        let v_addr = 42.0;

        let code = vec![
            Instruction::Const(x_addr),
            Instruction::Const(x_addr),
            Instruction::Load,
            Instruction::Const(v_addr),
            Instruction::Load,
            Instruction::Const(0.1),
            Instruction::Mul,
            Instruction::Add,
            Instruction::Store,
        ];

        let mut m = Machine::new(65536);
        m.store(x_addr as usize, 2.0);
        m.store(v_addr as usize, 3.0);
        m.execute(code);
        println!("Result: {}", m.load(x_addr as usize));
    }
    #[test]
    fn example_variables() {
        let x_addr = 22.0;
        let v_addr = 42.0;

        let code = vec![
            Instruction::Const(x_addr),
            Instruction::Const(x_addr),
            Instruction::Load,
            Instruction::Const(v_addr),
            Instruction::Load,
            Instruction::Const(0.1),
            Instruction::Mul,
            Instruction::Add,
            Instruction::Store,
        ];

        let mut m = Machine::new(65536);
        m.store(x_addr as usize, 2.0);
        m.store(v_addr as usize, 3.0);
        m.execute(code);
        println!("Result: {}", m.load(x_addr as usize));
    }
}
