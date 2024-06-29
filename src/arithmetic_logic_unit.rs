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

pub struct ArithmeticLogicUnit {
    subtract: bool,
    zero_flag: bool,
    carry_flag: bool
}

impl ArithmeticLogicUnit {
    pub fn set_subtract(&mut self) {
        self.subtract = true;
    }

    pub fn reset_subtract(&mut self) {
        self.subtract = false;
    }

    pub fn get_result(&mut self, lhs: u8, mut rhs: u8) -> u8 {
        if self.subtract {
            let (negated_rhs, _) = (!rhs).overflowing_add(1);
            rhs = negated_rhs;
        }

        let (result, overflow) = lhs.overflowing_add(rhs);

        self.carry_flag = overflow;
        self.zero_flag = result == 0;

        return result;
    }

    pub fn new() -> ArithmeticLogicUnit {
        ArithmeticLogicUnit {
            subtract: false,
            zero_flag: false,
            carry_flag: false
        }
    }
}