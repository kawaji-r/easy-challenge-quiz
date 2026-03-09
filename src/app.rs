use leptos::prelude::*;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;

#[wasm_bindgen(inline_js = r#"
export async function invokeTauri(cmd, payload) {
  const core = window.__TAURI__ && window.__TAURI__.core;
  if (!core || typeof core.invoke !== "function") {
    throw new Error("Tauri invoke API is not available");
  }
  return await core.invoke(cmd, payload ?? {});
}
"#)]
extern "C" {
    #[wasm_bindgen(catch, js_name = invokeTauri)]
    async fn invoke_tauri(cmd: &str, payload: JsValue) -> Result<JsValue, JsValue>;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Screen {
    Home,
    NameInput,
    GameSelect,
    GameAnswerList,
    GamePrivateInput,
    GameReveal,
    Clear,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
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

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct RoundQuestion {
    id: u32,
    text: String,
    answered: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct RoundEvaluation {
    next_state: GameState,
    correct_answer: String,
    player_results: Vec<PlayerResult>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct PlayerResult {
    answer: String,
    correct: bool,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct StartGameArgs {
    player_count: usize,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct SelectQuestionArgs {
    state: GameState,
    question_id: u32,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct EvaluateRoundArgs {
    state: GameState,
    answers: Vec<String>,
}

async fn tauri_invoke<Req, Res>(command: &str, payload: &Req) -> Result<Res, String>
where
    Req: Serialize,
    Res: DeserializeOwned,
{
    let js_payload = serde_wasm_bindgen::to_value(payload).map_err(|e| e.to_string())?;
    let response = invoke_tauri(command, js_payload)
        .await
        .map_err(js_error_to_string)?;
    serde_wasm_bindgen::from_value(response).map_err(|e| e.to_string())
}

fn js_error_to_string(err: JsValue) -> String {
    if let Some(text) = err.as_string() {
        return text;
    }
    js_sys::JSON::stringify(&err)
        .ok()
        .and_then(|s| s.as_string())
        .unwrap_or_else(|| "Unknown JS error".to_string())
}

fn default_player_name(index: usize) -> String {
    format!("プレイヤー{}", index + 1)
}

fn resize_player_names(current_names: &[String], new_count: usize) -> Vec<String> {
    let mut updated = (0..new_count)
        .map(default_player_name)
        .collect::<Vec<String>>();

    for (idx, name) in current_names.iter().enumerate().take(new_count) {
        updated[idx] = name.clone();
    }

    updated
}

fn player_name_or_default(player_names: &[String], index: usize) -> String {
    player_names
        .get(index)
        .cloned()
        .unwrap_or_else(|| default_player_name(index))
}

#[component]
pub fn App() -> impl IntoView {
    let (screen, set_screen) = signal(Screen::Home);
    let (player_count, set_player_count) = signal(5usize);
    let (player_names, set_player_names) = signal(
        (1..=5)
            .map(|i| format!("プレイヤー{i}"))
            .collect::<Vec<String>>(),
    );

    let (game_state, set_game_state) = signal(None::<GameState>);
    let (answers, set_answers) = signal(Vec::<String>::new());
    let (input_player_idx, set_input_player_idx) = signal(None::<usize>);
    let (temp_answer, set_temp_answer) = signal(String::new());
    let (round_evaluation, set_round_evaluation) = signal(None::<RoundEvaluation>);
    let (is_processing, set_is_processing) = signal(false);
    let (error_message, set_error_message) = signal(None::<String>);

    let update_player_count = move |new_count: usize| {
        set_player_count.set(new_count);
        set_player_names.update(|names| {
            *names = resize_player_names(names, new_count);
        });
    };

    let reset_in_round_input = move || {
        set_answers.set(Vec::new());
        set_input_player_idx.set(None);
        set_temp_answer.set(String::new());
        set_round_evaluation.set(None);
    };

    let start_game = move || {
        let count = player_count.get_untracked();
        set_is_processing.set(true);
        set_error_message.set(None);
        spawn_local(async move {
            let result = tauri_invoke::<_, GameState>(
                "start_game",
                &StartGameArgs {
                    player_count: count,
                },
            )
            .await;
            set_is_processing.set(false);
            match result {
                Ok(state) => {
                    set_game_state.set(Some(state));
                    reset_in_round_input();
                    set_screen.set(Screen::GameSelect);
                }
                Err(err) => set_error_message.set(Some(err)),
            }
        });
    };

    let selected_question = move || {
        game_state.get().and_then(|state| {
            let selected_id = state.selected_question_id?;
            state
                .round_questions
                .into_iter()
                .find(|q| q.id == selected_id)
        })
    };

    let all_answered = move || {
        let required = game_state
            .get()
            .map(|state| state.player_count)
            .unwrap_or_else(|| player_count.get());
        let current_answers = answers.get();
        current_answers.len() == required && current_answers.iter().all(|a| !a.trim().is_empty())
    };

    view! {
        <main class="app-root">
            {move || {
                if let Some(message) = error_message.get() {
                    view! { <div class="result-banner result-ng">{message}</div> }.into_any()
                } else {
                    view! { <></> }.into_any()
                }
            }}

            {move || match screen.get() {
                Screen::Home => {
                    view! {
                        <div class="screen screen-blue">
                            <div class="screen-inner">
                                <section class="hero">
                                    <h1 class="hero-title">
                                        "できて当たり前"<br/>"クイズ"
                                    </h1>
                                    <p class="hero-subtitle">"みんなで、答えられて当然の問題に挑戦！"</p>
                                </section>

                                <section class="card">
                                    <h2 class="section-title">"👥 参加人数"</h2>
                                    <div class="count-picker">
                                        <button
                                            class="round-btn"
                                            on:click=move |_| update_player_count(player_count.get().saturating_sub(1).max(2))
                                        >
                                            "-"
                                        </button>
                                        <div class="count-display">
                                            <p class="count-number">{move || player_count.get()}</p>
                                            <p class="count-unit">"人"</p>
                                        </div>
                                        <button
                                            class="round-btn"
                                            on:click=move |_| update_player_count((player_count.get() + 1).min(10))
                                        >
                                            "+"
                                        </button>
                                    </div>
                                    <p class="hint">"2〜10人で選択できます"</p>
                                </section>

                                <section class="card">
                                    <h2 class="section-title">"ルール説明"</h2>
                                    <div class="rules">
                                        <p><span class="rule-num">"1"</span>" 好きな問題を1つ選択"</p>
                                        <p><span class="rule-num">"2"</span>" 全員が正解できたら成功"</p>
                                        <p><span class="rule-num">"3"</span>" 成功したら次の人へ交代"</p>
                                        <p><span class="rule-num">"4"</span>" 5連続成功でゲームクリア"</p>
                                    </div>
                                </section>

                                <section class="warn-box">
                                    "一人でも間違えたら最初からやり直し。みんなが知っていそうな問題を選ぼう！"
                                </section>

                                <button class="primary-btn" on:click=move |_| set_screen.set(Screen::NameInput)>
                                    "🏆 次へ"
                                </button>
                            </div>
                        </div>
                    }
                        .into_any()
                }
                Screen::NameInput => {
                    view! {
                        <div class="screen screen-blue">
                            <div class="screen-inner">
                                <section class="hero">
                                    <h1 class="hero-title small">"プレイヤー名を入力"</h1>
                                    <p class="hero-subtitle">"空欄でもそのまま進められます"</p>
                                </section>

                                <section class="card">
                                    <div class="field-list">
                                        {(0..player_count.get())
                                            .map(|idx| {
                                                view! {
                                                    <label class="field-row">
                                                        <span>{format!("プレイヤー {}", idx + 1)}</span>
                                                        <input
                                                            class="text-input"
                                                            prop:value=move || {
                                                                player_names
                                                                    .get()
                                                                    .get(idx)
                                                                    .cloned()
                                                                    .unwrap_or_else(String::new)
                                                            }
                                                            on:input=move |ev| {
                                                                let value = event_target_value(&ev);
                                                                set_player_names.update(|names| {
                                                                    if let Some(name) = names.get_mut(idx) {
                                                                        *name = value;
                                                                    }
                                                                });
                                                            }
                                                        />
                                                    </label>
                                                }
                                            })
                                            .collect_view()}
                                    </div>
                                </section>

                                <button class="primary-btn" disabled=move || is_processing.get() on:click=move |_| start_game()>
                                    {move || if is_processing.get() { "開始中..." } else { "🏆 ゲームスタート" }}
                                </button>
                                <button class="secondary-btn" on:click=move |_| set_screen.set(Screen::Home)>
                                    "戻る"
                                </button>
                            </div>
                        </div>
                    }
                        .into_any()
                }
                Screen::GameSelect | Screen::GameAnswerList | Screen::GamePrivateInput | Screen::GameReveal => {
                    view! {
                        <div class="screen screen-blue">
                            <div class="screen-inner">
                                <button
                                    class="link-btn"
                                    on:click=move |_| {
                                        reset_in_round_input();
                                        set_game_state.set(None);
                                        set_error_message.set(None);
                                        set_screen.set(Screen::Home);
                                    }
                                >
                                    "✖ ゲームを終了"
                                </button>

                                <section class="card progress-card">
                                    <h3 class="progress-title">"連続成功"</h3>
                                    <div class="progress-track">
                                        {(1..=5)
                                            .map(|step| {
                                                view! {
                                                    <div class="progress-step">
                                                        <div class=move || {
                                                            let active = game_state
                                                                .get()
                                                                .map(|state| state.success_streak >= step)
                                                                .unwrap_or(false);
                                                            if active { "dot dot-active" } else { "dot" }
                                                        }>
                                                            {step}
                                                        </div>
                                                    </div>
                                                }
                                            })
                                            .collect_view()}
                                    </div>
                                </section>

                                <section class="card chooser-card">
                                    <p class="chooser-caption">"問題選択者"</p>
                                    <p class="chooser-name">
                                        {move || {
                                            let names = player_names.get();
                                            let current = game_state.get().map(|state| state.current_player).unwrap_or(0);
                                            player_name_or_default(&names, current)
                                        }}
                                    </p>
                                </section>

                                {move || {
                                    if let Some(result) = game_state.get().and_then(|state| state.round_result) {
                                        let text = if result {
                                            "前のラウンド: 全員正解！"
                                        } else {
                                            "前のラウンド: 誰かが不正解。連続成功はリセットされました"
                                        };
                                        let class = if result {
                                            "result-banner result-ok"
                                        } else {
                                            "result-banner result-ng"
                                        };
                                        view! { <div class=class>{text}</div> }.into_any()
                                    } else {
                                        view! { <></> }.into_any()
                                    }
                                }}

                                {move || match screen.get() {
                                    Screen::GameSelect => {
                                        view! {
                                            <section class="card">
                                                <h2 class="section-title">"問題を選択してください"</h2>
                                                <p class="hint">"みんなが答えられそうな問題を選ぼう"</p>
                                                <div class="question-list">
                                                    {move || {
                                                        if let Some(state) = game_state.get() {
                                                            state
                                                                .round_questions
                                                                .into_iter()
                                                                .enumerate()
                                                                .map(|(idx, q)| {
                                                                    let question_id = q.id;
                                                                    let is_answered = q.answered;
                                                                    let qtext = q.text.clone();
                                                                    view! {
                                                                        <button
                                                                            class=move || {
                                                                                if is_answered {
                                                                                    "question-btn done"
                                                                                } else {
                                                                                    "question-btn"
                                                                                }
                                                                            }
                                                                            disabled=move || is_answered || is_processing.get()
                                                                            on:click=move |_| {
                                                                                if is_answered {
                                                                                    return;
                                                                                }
                                                                                let current_state = match game_state.get_untracked() {
                                                                                    Some(state) => state,
                                                                                    None => return,
                                                                                };
                                                                                set_is_processing.set(true);
                                                                                set_error_message.set(None);
                                                                                spawn_local(async move {
                                                                                    let result = tauri_invoke::<_, GameState>(
                                                                                        "select_question",
                                                                                        &SelectQuestionArgs {
                                                                                            state: current_state,
                                                                                            question_id,
                                                                                        },
                                                                                    )
                                                                                    .await;
                                                                                    set_is_processing.set(false);
                                                                                    match result {
                                                                                        Ok(next_state) => {
                                                                                            let slots = vec![String::new(); next_state.player_count];
                                                                                            set_game_state.set(Some(next_state));
                                                                                            set_answers.set(slots);
                                                                                            set_input_player_idx.set(None);
                                                                                            set_temp_answer.set(String::new());
                                                                                            set_round_evaluation.set(None);
                                                                                            set_screen.set(Screen::GameAnswerList);
                                                                                        }
                                                                                        Err(err) => set_error_message.set(Some(err)),
                                                                                    }
                                                                                });
                                                                            }
                                                                        >
                                                                            <span class="qid">{idx + 1}</span>
                                                                            <span class="qtext">{qtext}</span>
                                                                            {if is_answered {
                                                                                view! { <span class="qdone">"✓"</span> }
                                                                                    .into_any()
                                                                            } else {
                                                                                view! { <></> }.into_any()
                                                                            }}
                                                                        </button>
                                                                    }
                                                                })
                                                                .collect_view()
                                                                .into_any()
                                                        } else {
                                                            view! { <></> }.into_any()
                                                        }
                                                    }}
                                                </div>
                                            </section>
                                        }
                                            .into_any()
                                    }
                                    Screen::GameAnswerList => {
                                        view! {
                                            <section class="card">
                                                <button
                                                    class="link-btn"
                                                    on:click=move |_| {
                                                        reset_in_round_input();
                                                        set_screen.set(Screen::GameSelect);
                                                    }
                                                >
                                                    "← 問題選択に戻る"
                                                </button>

                                                {move || {
                                                    if let Some(question) = selected_question() {
                                                        view! {
                                                            <div class="question-box">
                                                                <p>{"問題"}</p>
                                                                <h3>{question.text}</h3>
                                                            </div>
                                                        }
                                                            .into_any()
                                                    } else {
                                                        view! { <></> }.into_any()
                                                    }
                                                }}

                                                <p class="hint center">"順番に回答してください（他の人に見られないように）"</p>

                                                <div class="answer-list">
                                                    {move || {
                                                        if let Some(state) = game_state.get() {
                                                            (0..state.player_count)
                                                                .map(|idx| {
                                                                    let has_answered = answers
                                                                        .get()
                                                                        .get(idx)
                                                                        .map(|a| !a.trim().is_empty())
                                                                        .unwrap_or(false);
                                                                    view! {
                                                                        <button
                                                                            class=move || {
                                                                                if has_answered {
                                                                                    "answer-btn done"
                                                                                } else {
                                                                                    "answer-btn"
                                                                                }
                                                                            }
                                                                            on:click=move |_| {
                                                                                set_input_player_idx.set(Some(idx));
                                                                                set_temp_answer.set(String::new());
                                                                                set_screen.set(Screen::GamePrivateInput);
                                                                            }
                                                                        >
                                                                            <span>
                                                                                {move || {
                                                                                    player_name_or_default(
                                                                                        &player_names.get(),
                                                                                        idx,
                                                                                    )
                                                                                }}
                                                                                " の回答"
                                                                            </span>
                                                                            {if has_answered {
                                                                                view! { <span>"✓"</span> }.into_any()
                                                                            } else {
                                                                                view! { <></> }.into_any()
                                                                            }}
                                                                        </button>
                                                                    }
                                                                })
                                                                .collect_view()
                                                                .into_any()
                                                        } else {
                                                            view! { <></> }.into_any()
                                                        }
                                                    }}
                                                </div>

                                                {move || {
                                                    if all_answered() {
                                                        view! {
                                                            <button
                                                                class="accent-btn"
                                                                disabled=move || is_processing.get()
                                                                on:click=move |_| {
                                                                    let current_state = match game_state.get_untracked() {
                                                                        Some(state) => state,
                                                                        None => return,
                                                                    };
                                                                    let current_answers = answers.get_untracked();
                                                                    set_is_processing.set(true);
                                                                    set_error_message.set(None);
                                                                    spawn_local(async move {
                                                                        let result = tauri_invoke::<_, RoundEvaluation>(
                                                                            "evaluate_round",
                                                                            &EvaluateRoundArgs {
                                                                                state: current_state,
                                                                                answers: current_answers,
                                                                            },
                                                                        )
                                                                        .await;
                                                                        set_is_processing.set(false);
                                                                        match result {
                                                                            Ok(eval) => {
                                                                                set_round_evaluation.set(Some(eval));
                                                                                set_screen.set(Screen::GameReveal);
                                                                            }
                                                                            Err(err) => set_error_message.set(Some(err)),
                                                                        }
                                                                    });
                                                                }
                                                            >
                                                                {move || if is_processing.get() { "判定中..." } else { "答えを見る" }}
                                                            </button>
                                                        }
                                                            .into_any()
                                                    } else {
                                                        view! { <></> }.into_any()
                                                    }
                                                }}
                                            </section>
                                        }
                                            .into_any()
                                    }
                                    Screen::GamePrivateInput => {
                                        view! {
                                            <section class="card">
                                                <div class="private-box">
                                                    <h3>
                                                        {move || {
                                                            input_player_idx
                                                                .get()
                                                                .map(|idx| player_name_or_default(&player_names.get(), idx))
                                                                .unwrap_or_else(|| default_player_name(0))
                                                        }}
                                                    </h3>
                                                    <p>"他の人は見ないでください"</p>
                                                </div>

                                                {move || {
                                                    if let Some(question) = selected_question() {
                                                        view! {
                                                            <div class="question-box">
                                                                <p>{"問題"}</p>
                                                                <h3>{question.text}</h3>
                                                            </div>
                                                        }
                                                            .into_any()
                                                    } else {
                                                        view! { <></> }.into_any()
                                                    }
                                                }}

                                                <label class="field-row solo">
                                                    <span>"あなたの回答"</span>
                                                    <input
                                                        class="text-input"
                                                        prop:value=move || temp_answer.get()
                                                        on:input=move |ev| set_temp_answer.set(event_target_value(&ev))
                                                        placeholder="答えを入力"
                                                        autofocus=true
                                                    />
                                                </label>

                                                <button
                                                    class="success-btn"
                                                    on:click=move |_| {
                                                        if let Some(idx) = input_player_idx.get() {
                                                            set_answers.update(|all_answers| {
                                                                if let Some(slot) = all_answers.get_mut(idx) {
                                                                    *slot = temp_answer.get_untracked();
                                                                }
                                                            });
                                                            set_input_player_idx.set(None);
                                                            set_temp_answer.set(String::new());
                                                            set_screen.set(Screen::GameAnswerList);
                                                        }
                                                    }
                                                >
                                                    "回答を確定"
                                                </button>
                                                <button
                                                    class="secondary-btn"
                                                    on:click=move |_| {
                                                        set_input_player_idx.set(None);
                                                        set_temp_answer.set(String::new());
                                                        set_screen.set(Screen::GameAnswerList);
                                                    }
                                                >
                                                    "キャンセル"
                                                </button>
                                            </section>
                                        }
                                            .into_any()
                                    }
                                    Screen::GameReveal => {
                                        view! {
                                            <section class="card">
                                                {move || {
                                                    if let Some(question) = selected_question() {
                                                        view! {
                                                            <div class="question-box">
                                                                <p>{"問題"}</p>
                                                                <h3>{question.text}</h3>
                                                                <div class="correct-box">
                                                                    <small>"正解"</small>
                                                                    <strong>
                                                                        {move || {
                                                                            round_evaluation
                                                                                .get()
                                                                                .map(|eval| eval.correct_answer)
                                                                                .unwrap_or_default()
                                                                        }}
                                                                    </strong>
                                                                </div>
                                                            </div>
                                                        }
                                                            .into_any()
                                                    } else {
                                                        view! { <></> }.into_any()
                                                    }
                                                }}

                                                <div class="result-list">
                                                    {move || {
                                                        if let Some(eval) = round_evaluation.get() {
                                                            eval
                                                                .player_results
                                                                .into_iter()
                                                                .enumerate()
                                                                .map(|(idx, result)| {
                                                                    let row_class = if result.correct {
                                                                        "result-row ok"
                                                                    } else {
                                                                        "result-row ng"
                                                                    };
                                                                    view! {
                                                                        <div class=row_class>
                                                                            <div>
                                                                                <p class="player-label">
                                                                                    {player_name_or_default(&player_names.get(), idx)}
                                                                                </p>
                                                                                <p class="player-answer">{result.answer}</p>
                                                                            </div>
                                                                            <span class="judge-mark">
                                                                                {if result.correct { "✓" } else { "✗" }}
                                                                            </span>
                                                                        </div>
                                                                    }
                                                                })
                                                                .collect_view()
                                                                .into_any()
                                                        } else {
                                                            view! { <></> }.into_any()
                                                        }
                                                    }}
                                                </div>

                                                <button
                                                    class="primary-btn"
                                                    on:click=move |_| {
                                                        let eval = match round_evaluation.get_untracked() {
                                                            Some(value) => value,
                                                            None => return,
                                                        };
                                                        let is_clear = eval.next_state.is_clear;
                                                        set_game_state.set(Some(eval.next_state));
                                                        reset_in_round_input();
                                                        set_screen.set(if is_clear {
                                                            Screen::Clear
                                                        } else {
                                                            Screen::GameSelect
                                                        });
                                                    }
                                                >
                                                    "結果を確定"
                                                </button>
                                            </section>
                                        }
                                            .into_any()
                                    }
                                    _ => view! { <></> }.into_any(),
                                }}
                            </div>
                        </div>
                    }
                        .into_any()
                }
                Screen::Clear => {
                    view! {
                        <div class="screen screen-yellow">
                            <div class="screen-inner center-layout">
                                <div class="clear-icon">"🏆"</div>
                                <h1 class="hero-title clear">"ゲームクリア！"</h1>
                                <p class="clear-text">
                                    {move || {
                                        let count = game_state
                                            .get()
                                            .map(|state| state.player_count)
                                            .unwrap_or_else(|| player_count.get());
                                        format!("{}人全員で5問連続正解を達成しました", count)
                                    }}
                                </p>

                                <section class="card clear-card">
                                    <p>
                                        "チームワークばっちり！"
                                    </p>
                                    <p class="party">"🎉"</p>
                                </section>

                                <button
                                    class="warn-btn"
                                    on:click=move |_| {
                                        reset_in_round_input();
                                        set_game_state.set(None);
                                        set_screen.set(Screen::Home);
                                    }
                                >
                                    "もう一度プレイ"
                                </button>
                            </div>
                        </div>
                    }
                        .into_any()
                }
            }}
        </main>
    }
}
