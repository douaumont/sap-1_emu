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

/// Represents the databus of SAP-1
pub struct Databus {
    /// Value written to the databus
    current_state: u8
}

impl Databus {
    /// Write new state to databus
    pub fn write(&mut self, new_state: u8) {
        self.current_state = new_state;
    }

    /// Read current state of databus
    pub fn read(&self) -> u8 {
        self.current_state
    }

    /// Read current state of databus and set its state to zero
    pub fn read_with_reset(&mut self) -> u8 {
        let old_state = self.current_state;
        self.current_state = 0;
        return old_state;
    }

    pub fn new() -> Databus {
        Databus {current_state: 0}
    }
}