// Solutions for Project Rosalind problems
//
// Copyright (C) 2020  Peter van Heusden <pvh@webbedfeet.co.za>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

pub mod rosalind {
    mod ba1a;

    pub use ba1a::*;
}

#[cfg(test)]
mod rosalind_test {
    use crate::rosalind::*;

    #[test]
    fn test_ba1a_1() {
        let dna = b"GCGCG";
        let pattern = b"GCG";
        assert_eq!(ba1a(dna, pattern), 2);
    }
}
