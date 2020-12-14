pub mod parsers {
    use nom::IResult;

    #[derive(Debug, Eq, PartialEq, Clone)]
    pub enum RleSymbol {
        Dollar(u32),
        B(u32),
        O(u32),
    }

    impl RleSymbol {
        pub fn grow_pattern(&self, offset: &mut (u32, u32), alive_cells: &mut Vec<(u32, u32)>) {
            match self {
                RleSymbol::B(i) => {
                    offset.0 += i;
                }
                RleSymbol::O(i) => {
                    for j in (offset.0 .. offset.0 + i) {
                        alive_cells.push(offset.clone());
                        offset.0 = j + 1;
                    }
                }
                RleSymbol::Dollar(i) => {
                    offset.0 = 0;
                    offset.1 += i;
                }
            };
        }
    }

    pub fn parse_rle_symbol(input: &str) -> IResult<&str, RleSymbol> {
        let (i, num_str) = nom::character::complete::digit0(input)?;
        let count = match num_str {
            "" => 1,
            some_digit_string => some_digit_string.parse::<u32>().unwrap(),
        };
        nom::branch::alt((
            nom::combinator::value(
                RleSymbol::Dollar(count),
                nom::character::complete::char('$'),
            ),
            nom::combinator::value(RleSymbol::B(count), nom::character::complete::char('b')),
            nom::combinator::value(RleSymbol::O(count), nom::character::complete::char('o')),
        ))(i)
    }

    pub fn parse_rle_string(input: &str) -> IResult<&str, Vec<RleSymbol>> {
        nom::multi::many1(parse_rle_symbol)(input)
    }

    #[cfg(test)]
    mod tests {

        use super::*;

        #[test]
        fn parsing_single_rle_atom_works() {
            assert_eq!(parse_rle_symbol("$bo"), Ok(("bo", RleSymbol::Dollar(1))));
            assert_eq!(parse_rle_symbol("21$bo"), Ok(("bo", RleSymbol::Dollar(21))));
            assert_eq!(parse_rle_symbol("1bo"), Ok(("o", RleSymbol::B(1))));
        }

        #[test]
        fn parsing_multiple_rle_atoms_works() {
            assert_eq!(
                parse_rle_string("2bobo23$"),
                Ok((
                    "",
                    vec![
                        RleSymbol::B(2),
                        RleSymbol::O(1),
                        RleSymbol::B(1),
                        RleSymbol::O(1),
                        RleSymbol::Dollar(23),
                    ]
                ))
            );
        }

        #[test]
        fn rle_atom_works() {
            let mut offset = (0u32, 0u32);
            let mut pattern: Vec<(u32, u32)> = vec![];

            RleSymbol::B(2).grow_pattern(&mut offset, &mut pattern);
            assert_eq!(offset, (2, 0));
            assert_eq!(pattern, vec![]);

            RleSymbol::Dollar(1).grow_pattern(&mut offset, &mut pattern);
            assert_eq!(offset, (0, 1));
            assert_eq!(pattern, vec![]);

            RleSymbol::O(2).grow_pattern(&mut offset, &mut pattern);
            assert_eq!(offset, (2, 1));
            assert_eq!(pattern, vec![(0, 1), (1, 1)]);
        }
    }
}
