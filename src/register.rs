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

/// Represents a register
pub struct Register {
    content: u8
}

impl Register {
    /// Set the content of the register
    pub fn set(&mut self, new_content: u8) {
        self.content = new_content;
    }


    /// Read the contetn of the register
    pub fn read(&self) -> u8 {
        self.content
    }

    pub fn new() -> Register {
        Register {content: 0}
    }
}