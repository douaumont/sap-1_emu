/*
    Copyright 2024 Artyom Makarov

    This file is part of sap-1_emu.

    sap-1_emu is free software: you can redistribute it and/or modify it under the terms of the 
    GNU General Public License as published by the Free Software Foundation, 
    either version 3 of the License, or (at your option) any later version.

    sap-1_emu is distributed in the hope that it will be useful, 
    but WITHOUT ANY WARRANTY; 
    without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. 
    See the GNU General Public License for more details.

    You should have received a copy of the GNU General Public License along with sap-1_emu. 
    If not, see <https://www.gnu.org/licenses/>. 
*/


use std::time::Duration;

use crate::{databus::*, program_counter::*, register::*};

/// Memory is array of MEMORY_SIZE bytes
type Memory = [u8; MEMORY_SIZE];
/// Simple operations are the most basic operations a processing unit can perform
type SimpleOperation = fn(&mut ProcessingUnit);
/// Control word consists of several simple operations that are executed sequentially in this implementation
type ControlWord = Vec<SimpleOperation>;
/// Instruction is a sequence of control words
type Instruction = Vec<ControlWord>;

/// Represents the whole SAP-1 computer
pub struct ProcessingUnit {
    databus: Databus,
    memory: Memory,
    program_counter: ProgramCounter,
    memory_address_register: Register,
    instruction_register: Register,
    a_reg: Register,
    b_reg: Register,
    halted: bool,
}

const ARRAY_REPEAT_VALUE: Vec<Vec<for<'a> fn(&'a mut ProcessingUnit)>> = Instruction::new();

impl ProcessingUnit {
    /// Write program counter value to databus
    fn program_counter_out(&mut self) {
        self.databus.write(self.program_counter.read());
    }

    /// Read value from databus and write it to Memory Address Register (MAR)
    fn memory_address_register_in(&mut self) {
        self.memory_address_register.set(self.databus.read_with_reset());
    }

    /// Write the value of memory at address stored in MAR to databus
    fn memory_out(&mut self) {
        let address = self.memory_address_register.read();
        self.databus.write(self.memory[address as usize]);
    }

    /// Read value from databus into Instruction Register (IR)
    fn instruction_register_in(&mut self) {
        self.instruction_register.set(self.databus.read_with_reset());
    }

    /// Write the lower 4 bits of the IR (i.e. data of the instruction) to the databus
    fn instruction_data_out(&mut self) {
        let data = self.instruction_register.read() & 0x0F;
        self.databus.write(data);
    }


    /// Read value from the databus and store it in register A
    fn a_reg_in(&mut self) {
        let data = self.databus.read_with_reset();
        self.a_reg.set(data);
    }

    /// Advance program counter by one
    fn increment_program_counter(&mut self) {
        self.program_counter.advance();
    }

    fn halt_clock(&mut self) {
        self.halted = true;
    }

    /// Perform fetch cycle.
    /// After that IR will hold the opcode
    fn fetch(&mut self) {
        self.program_counter_out();
        self.memory_address_register_in();
        self.memory_out();
        self.instruction_register_in();
        self.increment_program_counter();
    }

    fn construct_instruction_table() -> [Instruction; 16] {
        let mut instruction_table: [Instruction; 16] = [ARRAY_REPEAT_VALUE; 16];
        
        //LDI
        instruction_table[0x5] = vec![vec![ProcessingUnit::instruction_data_out, ProcessingUnit::a_reg_in]];
        //HLT
        instruction_table[0xF] = vec![vec![ProcessingUnit::halt_clock]];

        return instruction_table;
    }

    pub fn run(&mut self) {
        let instruction_table = Self::construct_instruction_table();
        loop {
            self.fetch();
            if self.halted {
                break;
            }

            let opcode = (self.instruction_register.read() & 0xF0) >> 4;
            let instruction = &instruction_table[opcode as usize];
            for control_word in instruction {
                for operation in control_word {
                    operation(self);
                }
            }


            //imitate real clock which runs at ~500 Hz
            std::thread::sleep(Duration::from_millis(2));
        }
    }

    pub fn load_memory(&mut self, memory: Memory) {
        self.memory = memory;
    }

    pub fn new() -> ProcessingUnit {
        ProcessingUnit {
            databus: Databus::new(),
            memory: [0; MEMORY_SIZE],
            program_counter: ProgramCounter::new(),
            memory_address_register: Register::new(),
            instruction_register: Register::new(),
            a_reg: Register::new(),
            b_reg: Register::new(),
            halted: false,
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::ProcessingUnit;

    #[test]
    pub fn test_fetch_cycle() {
        let mut pu = ProcessingUnit::new();
        pu.memory[0] = 0x13;
        pu.fetch();
        assert_eq!(pu.instruction_register.read(), 0x13);
    }

    #[test]
    pub fn test_ldi_instruction() {
        let mut pu = ProcessingUnit::new();
        pu.memory[0] = 0x54;
        pu.memory[1] = 0xF0;
        pu.run();
        assert_eq!(pu.a_reg.read(), 0x4);
    }
}