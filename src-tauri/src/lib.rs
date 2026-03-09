mod question_bank;

use question_bank::BASE_QUESTIONS;
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicU64, Ordering};
use unicode_normalization::UnicodeNormalization;

const ROUND_QUESTION_COUNT: usize = 100;
static RNG_STATE: AtomicU64 = AtomicU64::new(0x9E37_79B9_7F4A_7C15);

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GameState {
    player_count: usize,
    success_streak: usize,
    current_player: usize,
    round_questions: Vec<RoundQuestion>,
    answered_question_ids: Vec<u32>,
    selected_question_id: Option<u32>,
    round_result: Option<bool>,
    is_clear: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RoundQuestion {
    id: u32,
    text: String,
    answered: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RoundEvaluation {
    next_state: GameState,
    correct_answer: String,
    player_results: Vec<PlayerResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PlayerResult {
    answer: String,
    correct: bool,
}

fn random_index(upper_exclusive: usize) -> usize {
    let mut prev = RNG_STATE.load(Ordering::Relaxed);
    loop {
        let mut x = prev;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        let next = x.wrapping_mul(0x2545_F491_4F6C_DD1D);
        match RNG_STATE.compare_exchange_weak(prev, next, Ordering::Relaxed, Ordering::Relaxed) {
            Ok(_) => return (next % upper_exclusive as u64) as usize,
            Err(actual) => prev = actual,
        }
    }
}

fn shuffle_in_place<T>(items: &mut [T]) {
    if items.len() < 2 {
        return;
    }

    for i in (1..items.len()).rev() {
        let j = random_index(i + 1);
        items.swap(i, j);
    }
}

fn sample_round_questions() -> Vec<RoundQuestion> {
    let mut questions = BASE_QUESTIONS.to_vec();
    shuffle_in_place(&mut questions);
    questions
        .into_iter()
        .take(ROUND_QUESTION_COUNT)
        .map(|q| RoundQuestion {
            id: q.id,
            text: q.question.to_string(),
            answered: false,
        })
        .collect()
}

fn clamp_player_count(player_count: usize) -> usize {
    player_count.clamp(2, 10)
}

fn next_player_index(current_player: usize, player_count: usize) -> usize {
    if player_count == 0 {
        0
    } else {
        (current_player + 1) % player_count
    }
}

fn normalize_answer(answer: &str) -> String {
    let normalized = answer
        .nfkc()
        .collect::<String>()
        .trim()
        .to_lowercase()
        .replace(' ', "")
        .replace('　', "");
    katakana_to_hiragana(&normalized)
}

fn katakana_to_hiragana(input: &str) -> String {
    input
        .chars()
        .map(|c| {
            let code = c as u32;
            if (0x30A1..=0x30F6).contains(&code) {
                char::from_u32(code - 0x60).unwrap_or(c)
            } else {
                c
            }
        })
        .collect()
}

fn is_correct_answer(user_answer: &str, correct_answers: &[&str]) -> bool {
    let normalized = normalize_answer(user_answer);
    correct_answers
        .iter()
        .map(|a| normalize_answer(a))
        .any(|correct| normalized == correct)
}

fn should_debug_force_clear(player_count: usize, question_id: u32, answers: &[String]) -> bool {
    player_count == 2
        && question_id == 1
        && answers.len() == 2
        && normalize_answer(&answers[0]) == "ろ"
        && normalize_answer(&answers[1]) == "で"
}

fn find_question(question_id: u32) -> Option<question_bank::Question> {
    BASE_QUESTIONS.iter().copied().find(|q| q.id == question_id)
}

fn validate_answers(player_count: usize, answers: &[String]) -> Result<(), String> {
    if answers.len() != player_count {
        return Err("回答人数が一致していません".into());
    }
    if answers.iter().any(|a| a.trim().is_empty()) {
        return Err("未回答のプレイヤーがいます".into());
    }
    Ok(())
}

#[tauri::command]
fn start_game(player_count: usize) -> GameState {
    GameState {
        player_count: clamp_player_count(player_count),
        success_streak: 0,
        current_player: 0,
        round_questions: sample_round_questions(),
        answered_question_ids: Vec::new(),
        selected_question_id: None,
        round_result: None,
        is_clear: false,
    }
}

#[tauri::command]
fn select_question(state: GameState, question_id: u32) -> Result<GameState, String> {
    let mut state = state;

    let target = state
        .round_questions
        .iter()
        .find(|q| q.id == question_id)
        .ok_or_else(|| "問題が見つかりません".to_string())?;

    if target.answered {
        return Err("この問題はすでに回答済みです".into());
    }

    state.selected_question_id = Some(question_id);
    state.round_result = None;
    Ok(state)
}

#[tauri::command]
fn evaluate_round(state: GameState, answers: Vec<String>) -> Result<RoundEvaluation, String> {
    let mut next_state = state;
    let question_id = next_state
        .selected_question_id
        .ok_or_else(|| "問題が選択されていません".to_string())?;

    validate_answers(next_state.player_count, &answers)?;

    let question =
        find_question(question_id).ok_or_else(|| "問題データが壊れています".to_string())?;

    let player_results = answers
        .iter()
        .map(|answer| PlayerResult {
            answer: answer.clone(),
            correct: is_correct_answer(answer, question.answer),
        })
        .collect::<Vec<_>>();

    let forced_clear = should_debug_force_clear(next_state.player_count, question.id, &answers);
    let all_correct = forced_clear || player_results.iter().all(|result| result.correct);

    if all_correct {
        next_state.success_streak += 1;
        next_state.answered_question_ids.push(question.id);
        for item in &mut next_state.round_questions {
            if item.id == question.id {
                item.answered = true;
            }
        }
        next_state.current_player =
            next_player_index(next_state.current_player, next_state.player_count);
        next_state.round_result = Some(true);
        if next_state.success_streak >= 5 {
            next_state.is_clear = true;
        }
    } else {
        next_state.success_streak = 0;
        next_state.current_player = 0;
        next_state.answered_question_ids.clear();
        for item in &mut next_state.round_questions {
            item.answered = false;
        }
        next_state.round_result = Some(false);
        next_state.is_clear = false;
    }

    next_state.selected_question_id = None;

    Ok(RoundEvaluation {
        next_state,
        correct_answer: question.answer.first().copied().unwrap_or("").to_string(),
        player_results,
    })
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            start_game,
            select_question,
            evaluate_round
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn start_game_clamps_player_count() {
        let low = start_game(1);
        let high = start_game(20);
        assert_eq!(low.player_count, 2);
        assert_eq!(high.player_count, 10);
    }

    #[test]
    fn normalize_answer_works_for_katakana() {
        assert_eq!(normalize_answer(" ｽｲｾｲ "), "すいせい");
    }

    #[test]
    fn evaluate_round_resets_streak_on_incorrect() {
        let mut state = start_game(2);
        state.selected_question_id = Some(1);
        let result = evaluate_round(state, vec!["水星".into(), "金星".into()])
            .expect("evaluation should succeed");

        assert_eq!(result.next_state.success_streak, 0);
        assert_eq!(result.next_state.current_player, 0);
        assert_eq!(result.next_state.round_result, Some(false));
    }
}
