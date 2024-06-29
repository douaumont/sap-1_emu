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
pub const MEMORY_SIZE: usize = 16;

/// Represents the program counter
pub struct ProgramCounter {
    value: u8
}

impl ProgramCounter {
    /// Read current value of program counter
    pub fn read(&self) -> u8 {
        self.value
    }

    pub fn set(&mut self, new_value: u8) {
        self.value = new_value;
    }

    /// Increment value of program counter
    pub fn advance(&mut self) {
        self.value += 1;
        if self.value as usize >= MEMORY_SIZE {
            self.value = 0;
        }
    }

    pub fn new() -> ProgramCounter {
        ProgramCounter {value: 0}
    }
}