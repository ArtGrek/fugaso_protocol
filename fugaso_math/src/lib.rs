pub mod protocol;
pub mod math;
pub mod protocol_new_game_name;
pub mod protocol_zeus;
pub mod protocol_thor;
pub mod protocol_hold_and_win;
pub mod protocol_big_bass_bonanza_1000;
pub mod protocol_mega_thunder;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
