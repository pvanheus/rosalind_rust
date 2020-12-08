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
    mod ba1b;
    mod ba1c;
    mod ba1d;
    mod ba1e;

    pub use ba1a::*;
    pub use ba1b::*;
    pub use ba1c::*;
    pub use ba1d::*;
    pub use ba1e::*;
}

#[cfg(test)]
mod rosalind_test {
    use crate::rosalind::*;
    use std::collections::HashMap;

    #[test]
    fn test_ba1a_1() {
        let dna = b"GCGCG";
        let pattern = b"GCG";
        assert_eq!(ba1a(dna, pattern), 2);
    }

    #[test]
    fn test_ba1b_1() {
        let dna = "ACGTTGCATGTCGCATGATGCATGAGAGCT";
        let k = 4;
        assert_eq!(ba1b(dna, k), vec!["CATG", "GCAT"]);
    }

    #[test]
    fn test_ba1c_1() {
        let dna = "AAAACCCGGT";
        let complementary_dna = "ACCGGGTTTT";
        assert_eq!(ba1c(dna), complementary_dna);
    }

    #[test]
    fn test_ba1d_1() {
        let dna = "GATATATGCATATACTT";
        let pattern = "ATAT";
        let result = vec![1, 3, 9];
        assert_eq!(ba1d(dna, pattern), result);
    }

    #[test]
    fn test_ba1e_1() {
        let dna = "CGGACTCGACAGATGTGAAGAAATGTGAAGACTGAGTGAAGAGAAGAGGAAACACGACACGACATTGCGACATAATGTACGAATGTAATGTGCCTATGGC";
        let k = 5;
        let l = 75;
        let t = 4;
        let mut result = vec!["CGACA", "GAAGA", "AATGT"];
        result.sort();
        assert_eq!(ba1e(dna, k, l, t), result);
    }
}
