use anchor_lang::error_code;

#[error_code]
pub enum ScoringError {
    #[msg("Check the code for the correct scores")]
    IncorrectScores,
    #[msg("there is no summation. Check the code!")]
    NoTotalScore,
//     #[msg("Check the code for the correct scores")]
//     IncorrectScores,
}
