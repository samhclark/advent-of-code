use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct PuzzleAnswer {
    answer: String
}

impl From<&str> for PuzzleAnswer {
    fn from(s: &str) -> Self {
        PuzzleAnswer {
            answer: s.trim().to_string()
        }
    }
}

impl From<String> for PuzzleAnswer {
    fn from(s: String) -> Self {
        PuzzleAnswer {
            answer: s.trim().to_string()
        }
    }
}

impl From<i32> for PuzzleAnswer {
    fn from(value: i32) -> Self {
        PuzzleAnswer {
            answer: value.to_string()
        }
    }
}

impl From<i64> for PuzzleAnswer {
    fn from(value: i64) -> Self {
        PuzzleAnswer {
            answer: value.to_string()
        }
    }
}

impl From<u32> for PuzzleAnswer {
    fn from(value: u32) -> Self {
        PuzzleAnswer {
            answer: value.to_string()
        }
    }
}

impl From<usize> for PuzzleAnswer {
    fn from(value: usize) -> Self {
        PuzzleAnswer {
            answer: value.to_string()
        }
    }
}

impl Display for PuzzleAnswer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "puzzle_answer=[{}]", self.answer)
    }
}

// pub struct Day {
//     pub input: String
// }
