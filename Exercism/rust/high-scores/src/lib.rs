#[derive(Debug)]
pub struct HighScores<'a>{
    scores: &'a [u32],
}

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> Self {
        HighScores {scores}
    }

    pub fn scores(&self) -> &[u32] {
        self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        self.scores().last().copied()
    }

    pub fn personal_best(&self) -> Option<u32> {

        if self.scores().is_empty(){
            return None
        }

        let mut sorted = self.scores().to_vec();
        &sorted.sort_by(|a, b| b.cmp(a));
        Some(sorted[0])
        
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut sorted = self.scores().to_vec();
        &sorted.sort();
        sorted.iter().rev().take(3).cloned().collect()
    }
}
