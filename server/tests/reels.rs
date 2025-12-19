use std::fs;
use std::path::Path;
use server::bng;

#[test]
fn reels() {
    let input_path = Path::new("../data/three_aztec_temples/reels/reels.json");
    let output_path = Path::new("../data/three_aztec_temples/reels/reels_out.json");
    let file_content = fs::read_to_string(input_path).expect("Не удалось прочитать файл reels.json");
    let data: bng::three_aztec_temples::reels::Reels = serde_json::from_str(&file_content).expect("Ошибка при разборе reels.json");
    let json_out = serde_json::to_string_pretty(&data).expect("Ошибка при сериализации структуры");
    fs::write(output_path, json_out).expect("Не удалось записать reels_out.json");
}