use parser::{endline, fixed, many1, optional, signed_number, take_while1};

#[derive(Debug, PartialEq, Clone, Copy)]
enum Op {
    Nop(i32),
    Jmp(i32),
    Acc(i32),
}

struct BootCode {
    accumulator: i32,
    ip: usize,
    instructions: Vec<Op>,
    visited: Vec<bool>,
}

impl BootCode {
    fn load(instructions: Vec<Op>) -> Self {
        let instructions_len = instructions.len();
        BootCode {
            accumulator: 0,
            ip: 0,
            instructions: instructions,
            visited: vec![false; instructions_len],
        }
    }

    fn step(&mut self) {
        let current_ip = self.ip;
        match self.instructions[current_ip] {
            Op::Nop(_) => self.ip += 1,
            Op::Jmp(amt) => self.ip = ((self.ip as i32) + amt) as usize,
            Op::Acc(amt) => {
                self.accumulator += amt;
                self.ip += 1
            }
        };
        self.visited[current_ip] = true;
    }

    fn run(&mut self) -> bool {
        loop {
            if self.ip == self.instructions.len() {
                return true;
            } else if self.visited[self.ip] {
                return false;
            }
            self.step();
        }
    }

    fn repair(&mut self) {
        loop {
            let current_ip = self.ip;
            let current_acc = self.accumulator;
            let current_op = self.instructions[current_ip];

            match current_op {
                Op::Nop(amt) if amt != 0 => self.instructions[current_ip] = Op::Jmp(amt),
                Op::Jmp(amt) => self.instructions[current_ip] = Op::Nop(amt),
                _ => {}
            };

            let res = self.run();
            if res {
                return;
            } else {
                self.ip = current_ip;
                self.accumulator = current_acc;
                for e in self.visited.iter_mut() {
                    *e = false;
                }
                self.instructions[current_ip] = current_op;
                self.step();
            }
        }
    }

    fn reset(&mut self) {
        self.ip = 0;
        self.accumulator = 0;
        for e in self.visited.iter_mut() {
            *e = false;
        }
    }

    fn accumulator(&self) -> i32 {
        self.accumulator
    }
}

fn parse_instruction(input: &str) -> Option<(Op, &str)> {
    let (op_str, input) = take_while1(|c| c.is_ascii_lowercase(), input)?;
    let (_, input) = fixed(" ", input)?;
    let (amt, input) = signed_number(input)?;
    let op = match op_str {
        "nop" => Op::Nop(amt),
        "jmp" => Op::Jmp(amt),
        "acc" => Op::Acc(amt),
        _ => return None,
    };
    let (_, input) = optional(endline, input);
    Some((op, input))
}

fn main() {
    let input = include_str!("input");
    let (instructions, input) = many1(parse_instruction, input).expect("Failed to parse input");
    assert_eq!(input, "");
    let mut machine = BootCode::load(instructions);
    let res_1 = machine.run();
    assert!(!res_1);
    println!(
        "Part 1: Accumulator value at cycle: {}",
        machine.accumulator()
    );
    machine.reset();
    machine.repair();
    println!(
        "Part 2: Accumulator value after repair: {}",
        machine.accumulator()
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_op_variants() {
        for (input, expected) in [
            ("nop +0", Op::Nop(0)),
            ("acc +9", Op::Acc(9)),
            ("jmp -10", Op::Jmp(-10)),
        ]
        .iter()
        {
            let res = parse_instruction(input);
            assert_eq!(res, Some((*expected, "")));
        }
    }

    #[test]
    fn parse_op_multiple() {
        let input = "nop +0\njmp +10";
        let res = parse_instruction(input);
        assert_eq!(res, Some((Op::Nop(0), "jmp +10")));
        let (_, input) = res.unwrap();
        let res = parse_instruction(input);
        assert_eq!(res, Some((Op::Jmp(10), "")));
        let (_, input) = res.unwrap();
        let res = parse_instruction(input);
        assert!(res.is_none());
    }

    #[test]
    fn bootcode_step() {
        let mut machine = machine();
        assert_machine(&machine, 0, 0);
        machine.step();
        assert_machine(&machine, 1, 0);
        machine.step();
        assert_machine(&machine, 2, 1);
        machine.step();
        assert_machine(&machine, 6, 1);
    }

    #[test]
    fn bootcode_cycle() {
        let mut machine = machine();
        let res = machine.run();
        assert!(!res);
        assert_machine(&machine, 1, 5);
    }

    #[test]
    fn bootcode_repair() {
        let mut machine = machine();
        machine.repair();
        let res = machine.run();
        assert!(res);
        assert_machine(&machine, 9, 8);
    }

    fn machine() -> BootCode {
        use Op::*;
        let instructions = vec![
            Nop(0),
            Acc(1),
            Jmp(4),
            Acc(3),
            Jmp(-3),
            Acc(-99),
            Acc(1),
            Jmp(-4),
            Acc(6),
        ];
        BootCode::load(instructions)
    }

    fn assert_machine(machine: &BootCode, exp_ip: usize, exp_acc: i32) {
        assert_eq!(machine.ip, exp_ip, "Unexpected IP");
        assert_eq!(machine.accumulator(), exp_acc, "Unexpected accumulator");
    }
}
