use std::cmp::{max, min};

#[derive(Debug)]
pub struct InvalidRangeError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Range {
    pub start: u64,
    pub end: u64,
}

impl Range {
    pub const fn new(start: u64, end: u64) -> Result<Self, InvalidRangeError> {
        if end > start {
            Ok(Self { start, end })
        } else {
            Err(InvalidRangeError {})
        }
    }

    pub const fn contains(self, value: u64) -> bool {
        (value >= self.start) && (value < self.end)
    }

    pub fn intersection(&self, other: Self) -> Option<Self> {
        if max(self.start, other.start) > min(self.end - 1, other.end - 1) {
            None
        } else {
            Some(Self {
                start: max(self.start, other.start),
                end: min(self.end, other.end),
            })
        }
    }

    /// self minus other
    pub fn difference(self, other: Self) -> Vec<Self> {
        // special case 1: complete overlap. Return empty list
        if other.contains(self.start) && other.contains(self.end - 1) {
            return vec![];
        }

        // special case 2: no overlap. Return self
        if self.intersection(other).is_none() {
            return vec![self];
        }

        // special case 3: other is contained by self
        //     return 2 ranges
        if other.start > self.start && other.end < self.end {
            return vec![
                Self::new(self.start, other.start).unwrap(),
                Self::new(other.end, self.end).unwrap(),
            ];
        }

        // special case 4: left side gone
        if other.contains(self.start) {
            return vec![Self::new(other.end, self.end).unwrap()];
        }

        // special case 5: right side gone
        vec![Self::new(self.start, other.start).unwrap()]
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn contains() {
        // the range 1, 2
        let uut = Range::new(1, 3).unwrap();

        assert_eq!(uut.contains(0), false);
        assert_eq!(uut.contains(1), true);
        assert_eq!(uut.contains(2), true);
        assert_eq!(uut.contains(3), false);
    }

    #[test]
    fn intersection() {
        // Given: the range 10, 11, 12, 13, 14, 15
        let ten_to_15 = Range::new(10, 16).unwrap();

        // the range 1, 2, 3
        let one_to_3 = Range::new(1, 4).unwrap();
        assert_eq!(ten_to_15.intersection(one_to_3), None);

        // the range 8, 9
        let eight_nine = Range::new(8, 10).unwrap();
        assert_eq!(ten_to_15.intersection(eight_nine), None);

        // the range 9
        let nine = Range::new(9, 10).unwrap();
        assert_eq!(ten_to_15.intersection(nine), None);

        // the range 9, 10
        let nine_ten = Range::new(9, 11).unwrap();
        assert_eq!(
            ten_to_15.intersection(nine_ten),
            Some(Range::new(10, 11).unwrap())
        );

        // the range 9, 10, 11
        let nine_to_11 = Range::new(9, 12).unwrap();
        assert_eq!(
            ten_to_15.intersection(nine_to_11),
            Some(Range::new(10, 12).unwrap())
        );

        // the range 10
        let ten = Range::new(10, 11).unwrap();
        assert_eq!(
            ten_to_15.intersection(ten),
            Some(Range::new(10, 11).unwrap())
        );

        // the range 10, 11
        let ten_to_11 = Range::new(10, 12).unwrap();
        assert_eq!(
            ten_to_15.intersection(ten_to_11),
            Some(Range::new(10, 12).unwrap())
        );

        // the range 10, 11, 12, 13, 14
        let ten_to_14 = Range::new(10, 15).unwrap();
        assert_eq!(
            ten_to_15.intersection(ten_to_14),
            Some(Range::new(10, 15).unwrap())
        );

        // the range 11, 12, 13, 14
        let eleven_to_14 = Range::new(11, 15).unwrap();
        assert_eq!(
            ten_to_15.intersection(eleven_to_14),
            Some(Range::new(11, 15).unwrap())
        );

        // the range 10, 11, 12, 13, 14, 15
        assert_eq!(
            ten_to_15.intersection(ten_to_15),
            Some(Range::new(10, 16).unwrap())
        );

        // the range 10, 11, 12, 13, 14, 15, 16
        let ten_to_16 = Range::new(10, 17).unwrap();
        assert_eq!(
            ten_to_15.intersection(ten_to_16),
            Some(Range::new(10, 16).unwrap())
        );

        // the range 9, 10, 11, 12, 13, 14, 15, 16
        let nine_to_16 = Range::new(9, 17).unwrap();
        assert_eq!(
            ten_to_15.intersection(nine_to_16),
            Some(Range::new(10, 16).unwrap())
        );

        // the range 14, 15, 16
        let fourteen_to_16 = Range::new(14, 17).unwrap();
        assert_eq!(
            ten_to_15.intersection(fourteen_to_16),
            Some(Range::new(14, 16).unwrap())
        );

        // the range 15, 16
        let fifteen_to_16 = Range::new(15, 17).unwrap();
        assert_eq!(
            ten_to_15.intersection(fifteen_to_16),
            Some(Range::new(15, 16).unwrap())
        );

        // the range 15
        let fifteen = Range::new(15, 16).unwrap();
        assert_eq!(
            ten_to_15.intersection(fifteen),
            Some(Range::new(15, 16).unwrap())
        );

        // the range 16
        let sixteen = Range::new(16, 17).unwrap();
        assert_eq!(ten_to_15.intersection(sixteen), None);
    }

    #[test]
    fn difference() {
        let five_to_nine = Range::new(5, 10).unwrap();

        // the range 1, 2, 3
        let one_to_3 = Range::new(1, 4).unwrap();
        assert_eq!(
            five_to_nine.difference(one_to_3),
            vec![Range::new(5, 10).unwrap()]
        );

        // the range 1, 2, 3, 4
        let one_to_4 = Range::new(1, 5).unwrap();
        assert_eq!(
            five_to_nine.difference(one_to_4),
            vec![Range::new(5, 10).unwrap()]
        );

        // the range 1, 2, 3, 4, 5
        let one_to_5 = Range::new(1, 6).unwrap();
        assert_eq!(
            five_to_nine.difference(one_to_5),
            vec![Range::new(6, 10).unwrap()]
        );

        // the range 1, 2, 3, 4, 5, 6
        let one_to_6 = Range::new(1, 7).unwrap();
        assert_eq!(
            five_to_nine.difference(one_to_6),
            vec![Range::new(7, 10).unwrap()]
        );

        // the range 1, 2, 3, 4, 5, 6, 7, 8
        let one_to_8 = Range::new(1, 9).unwrap();
        assert_eq!(
            five_to_nine.difference(one_to_8),
            vec![Range::new(9, 10).unwrap()]
        );

        // the range 1, 2, 3, 4, 5, 6, 7, 8, 9
        let one_to_9 = Range::new(1, 10).unwrap();
        assert_eq!(five_to_nine.difference(one_to_9), vec![]);

        // the range 4, 5, 6, 7, 8, 9
        let four_to_9 = Range::new(4, 10).unwrap();
        assert_eq!(five_to_nine.difference(four_to_9), vec![]);

        // the range 5, 6, 7, 8, 9
        let five_to_9 = Range::new(5, 10).unwrap();
        assert_eq!(five_to_nine.difference(five_to_9), vec![]);

        // the range 6, 7, 8, 9
        let six_to_9 = Range::new(6, 10).unwrap();
        assert_eq!(
            five_to_nine.difference(six_to_9),
            vec![Range::new(5, 6).unwrap()]
        );

        // the range 7, 8, 9
        let seven_to_9 = Range::new(7, 10).unwrap();
        assert_eq!(
            five_to_nine.difference(seven_to_9),
            vec![Range::new(5, 7).unwrap()]
        );

        // the range 7, 8, 9, 10
        let seven_to_10 = Range::new(7, 11).unwrap();
        assert_eq!(
            five_to_nine.difference(seven_to_10),
            vec![Range::new(5, 7).unwrap()]
        );

        // the range 9, 10
        let nine_ten = Range::new(9, 11).unwrap();
        assert_eq!(
            five_to_nine.difference(nine_ten),
            vec![Range::new(5, 9).unwrap()]
        );

        // the range 9
        let nine = Range::new(9, 10).unwrap();
        assert_eq!(
            five_to_nine.difference(nine),
            vec![Range::new(5, 9).unwrap()]
        );

        // the range 10
        let ten = Range::new(10, 11).unwrap();
        assert_eq!(
            five_to_nine.difference(ten),
            vec![Range::new(5, 10).unwrap()]
        );

        // the range 6, 7, 8
        let six_to_8 = Range::new(6, 9).unwrap();
        assert_eq!(
            five_to_nine.difference(six_to_8),
            vec![Range::new(5, 6).unwrap(), Range::new(9, 10).unwrap()]
        );

        // the range 7
        let seven = Range::new(7, 8).unwrap();
        assert_eq!(
            five_to_nine.difference(seven),
            vec![Range::new(5, 7).unwrap(), Range::new(8, 10).unwrap()]
        );
    }
}
