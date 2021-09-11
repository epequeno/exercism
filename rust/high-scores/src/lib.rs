#[derive(Debug)]
pub struct HighScores(Vec<u32>);

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        HighScores(scores.to_vec())
    }

    pub fn scores(&self) -> &[u32] {
        let HighScores(s) = self;
        s.as_slice()
    }

    pub fn latest(&self) -> Option<u32> {
        let HighScores(s) = self;
        s.iter().cloned().last()
    }

    pub fn personal_best(&self) -> Option<u32> {
        let HighScores(s) = self;
        s.iter().cloned().max()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let HighScores(s) = self;
        let mut res = s.iter().cloned().collect::<Vec<u32>>();
        res.sort();
        res.reverse();
        res.into_iter().take(3).collect()
    }
}
