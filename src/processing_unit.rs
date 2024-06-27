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


use crate::{databus::*, program_counter::*, register::*};

/// Memory is array of MEMORY_SIZE bytes
type Memory = [u8; MEMORY_SIZE];

/// Represents the whole SAP-1 computer
pub struct ProcessingUnit {
    databus: Databus,
    memory: Memory,
    program_counter: ProgramCounter,
    memory_address_register: Register,
    instruction_register: Register,
}

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

    /// Advance program counter by one
    fn increment_program_counter(&mut self) {
        self.program_counter.advance();
    }


    /// Perform fetch cycle
    fn fetch(&mut self) {
        self.program_counter_out();
        self.memory_address_register_in();
        self.memory_out();
        self.instruction_register_in();
        self.increment_program_counter();
    }

    pub fn new() -> ProcessingUnit {
        ProcessingUnit {
            databus: Databus::new(),
            memory: [0; MEMORY_SIZE],
            program_counter: ProgramCounter::new(),
            memory_address_register: Register::new(),
            instruction_register: Register::new()
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
}