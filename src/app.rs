use leptos::prelude::*;
use unicode_normalization::UnicodeNormalization;

#[derive(Clone, Copy)]
struct Question {
    id: u32,
    question: &'static str,
    answer: &'static [&'static str],
}

const QUESTIONS: [Question; 100] = [
    Question {
        id: 1,
        question: "太陽から一番近い惑星は？",
        answer: &["水星", "すいせい"],
    },
    Question {
        id: 2,
        question: "1円玉は何で出来ている？",
        answer: &["アルミニウム"],
    },
    Question {
        id: 3,
        question: "ギリシャの首都は？",
        answer: &["アテネ"],
    },
    Question {
        id: 4,
        question: "日本で一番高い山は？",
        answer: &["富士山", "ふじさん"],
    },
    Question {
        id: 5,
        question: "地球の衛星の名前は？",
        answer: &["月", "つき"],
    },
    Question {
        id: 6,
        question: "日本の首都は？",
        answer: &["東京", "とうきょう"],
    },
    Question {
        id: 7,
        question: "一年は何日？",
        answer: &[
            "365日",
            "さんびゃくろくじゅうごにち",
            "サンビャクロクジュウゴニチ",
        ],
    },
    Question {
        id: 8,
        question: "水の化学式は？",
        answer: &["H2O", "えいちつーおー"],
    },
    Question {
        id: 9,
        question: "日本の国旗の色は？（赤とあと一つは？）",
        answer: &["白", "しろ"],
    },
    Question {
        id: 10,
        question: "1+1は？",
        answer: &["2", "に"],
    },
    Question {
        id: 11,
        question: "フランスの首都は？",
        answer: &["パリ"],
    },
    Question {
        id: 12,
        question: "イタリアの首都は？",
        answer: &["ローマ"],
    },
    Question {
        id: 13,
        question: "イギリスの首都は？",
        answer: &["ロンドン"],
    },
    Question {
        id: 14,
        question: "アメリカの首都は？",
        answer: &[
            "ワシントンD.C.",
            "わしんとんでぃーしー",
            "ワシントンディーシー",
            "わしんとんD.C.",
        ],
    },
    Question {
        id: 15,
        question: "カナダの首都は？",
        answer: &["オタワ"],
    },
    Question {
        id: 16,
        question: "オーストラリアの首都は？",
        answer: &["キャンベラ"],
    },
    Question {
        id: 17,
        question: "スペインの首都は？",
        answer: &["マドリード"],
    },
    Question {
        id: 18,
        question: "ドイツの首都は？",
        answer: &["ベルリン"],
    },
    Question {
        id: 19,
        question: "ブラジルの首都は？",
        answer: &["ブラジリア"],
    },
    Question {
        id: 20,
        question: "インドの首都は？",
        answer: &["ニューデリー"],
    },
    Question {
        id: 21,
        question: "1週間は何日？",
        answer: &["7日", "しちにち", "なのか"],
    },
    Question {
        id: 22,
        question: "1時間は何分？",
        answer: &[
            "60分",
            "ろくじゅっぷん",
            "ろくじっぷん",
            "ロクジュップン",
            "ロクジップン",
        ],
    },
    Question {
        id: 23,
        question: "1分は何秒？",
        answer: &["60秒", "ろくじゅうびょう"],
    },
    Question {
        id: 24,
        question: "1メートルは何センチメートル？",
        answer: &[
            "100センチメートル",
            "ひゃくせんちめーとる",
            "ヒャクセンチメートル",
            "100せんちめーとる",
        ],
    },
    Question {
        id: 25,
        question: "1キロメートルは何メートル？",
        answer: &[
            "1000メートル",
            "せんめーとる",
            "センメートル",
            "1000めーとる",
        ],
    },
    Question {
        id: 26,
        question: "円周率の最初の2桁は？",
        answer: &["3.14", "さんてんいちよん"],
    },
    Question {
        id: 27,
        question: "日本で一番長い川は？",
        answer: &["信濃川", "しなのがわ"],
    },
    Question {
        id: 28,
        question: "日本で一番大きい湖は？",
        answer: &["琵琶湖", "びわこ"],
    },
    Question {
        id: 29,
        question: "人間の体温は平均何度？",
        answer: &["36度", "さんじゅうろくど"],
    },
    Question {
        id: 30,
        question: "オリンピックは何年に一度？",
        answer: &["4年", "よねん"],
    },
    Question {
        id: 31,
        question: "サッカーは一チーム何人？",
        answer: &["11人", "じゅういちにん"],
    },
    Question {
        id: 32,
        question: "野球は一チーム何人？",
        answer: &["9人", "きゅうにん"],
    },
    Question {
        id: 33,
        question: "バスケットボールは一チーム何人？",
        answer: &["5人", "ごにん"],
    },
    Question {
        id: 34,
        question: "バレーボールは一チーム何人？",
        answer: &["6人", "ろくにん"],
    },
    Question {
        id: 35,
        question: "将棋の駒は何種類？",
        answer: &["8種類", "はっしゅるい"],
    },
    Question {
        id: 36,
        question: "日本の都道府県の数は？",
        answer: &["47", "よんじゅうなな"],
    },
    Question {
        id: 37,
        question: "北海道の県庁所在地は？",
        answer: &["札幌", "さっぽろ"],
    },
    Question {
        id: 38,
        question: "沖縄の県庁所在地は？",
        answer: &["那覇", "なは"],
    },
    Question {
        id: 39,
        question: "富士山は何県にある？（2県のうち1つ）",
        answer: &[
            "静岡県",
            "しずおかけん",
            "シズオカケン",
            "山梨県",
            "やまなしけん",
            "ヤマナシケン",
        ],
    },
    Question {
        id: 40,
        question: "日本で一番人口が多い都道府県は？",
        answer: &["東京都", "とうきょうと"],
    },
    Question {
        id: 41,
        question: "東京スカイツリーの高さは何メートル？",
        answer: &[
            "634メートル",
            "ろっぴゃくさんじゅうよんめーとる",
            "ロッピャクサンジュウヨンメートル",
            "634めーとる",
        ],
    },
    Question {
        id: 42,
        question: "日本の国歌は？",
        answer: &["君が代", "きみがよ"],
    },
    Question {
        id: 43,
        question: "日本の国花は？",
        answer: &["桜", "さくら"],
    },
    Question {
        id: 44,
        question: "12の干支の最初は？",
        answer: &["子（ねずみ）", "ねずみ"],
    },
    Question {
        id: 45,
        question: "12の干支の最後は？",
        answer: &["亥（いのしし）", "いのしし"],
    },
    Question {
        id: 46,
        question: "春夏秋冬で一番最初の季節は？",
        answer: &["春", "はる"],
    },
    Question {
        id: 47,
        question: "一月は何日まである？",
        answer: &["31日", "さんじゅういちにち"],
    },
    Question {
        id: 48,
        question: "二月は何日まである？（平年）",
        answer: &["28日", "にじゅうはちにち"],
    },
    Question {
        id: 49,
        question: "虹は何色？",
        answer: &["7色", "なないろ"],
    },
    Question {
        id: 50,
        question: "太陽系の惑星の数は？",
        answer: &["8個", "はっこ"],
    },
    Question {
        id: 51,
        question: "地球は太陽の周りを何日で一周する？",
        answer: &[
            "365日",
            "さんびゃくろくじゅうごにち",
            "サンビャクロクジュウゴニチ",
        ],
    },
    Question {
        id: 52,
        question: "北極にいる白いクマは？",
        answer: &["ホッキョクグマ"],
    },
    Question {
        id: 53,
        question: "世界で一番高い山は？",
        answer: &["エベレスト"],
    },
    Question {
        id: 54,
        question: "世界で一番大きい海は？",
        answer: &["太平洋", "たいへいよう"],
    },
    Question {
        id: 55,
        question: "世界で一番長い川は？",
        answer: &["ナイル川", "ないるがわ"],
    },
    Question {
        id: 56,
        question: "日本の三大祭りの一つ、京都の祭りは？",
        answer: &["祇園祭", "ぎおんまつり"],
    },
    Question {
        id: 57,
        question: "五輪のマークはいくつの輪？",
        answer: &["5つ", "いつつ"],
    },
    Question {
        id: 58,
        question: "日本で一番面積が大きい都道府県は？",
        answer: &["北海道", "ほっかいどう"],
    },
    Question {
        id: 59,
        question: "日本で一番面積が小さい都道府県は？",
        answer: &["香川県", "かがわけん"],
    },
    Question {
        id: 60,
        question: "日本三景の一つ、宮城県にあるのは？",
        answer: &["松島", "まつしま"],
    },
    Question {
        id: 61,
        question: "富士五湖の中で一番大きい湖は？",
        answer: &["山中湖", "やまなかこ"],
    },
    Question {
        id: 62,
        question: "光の三原色の一つは？",
        answer: &["赤", "あか"],
    },
    Question {
        id: 63,
        question: "日本の硬貨で一番大きい金額は？",
        answer: &["500円", "ごひゃくえん"],
    },
    Question {
        id: 64,
        question: "日本の紙幣で一番大きい金額は？",
        answer: &["1万円", "いちまんえん"],
    },
    Question {
        id: 65,
        question: "トランプは全部で何枚？",
        answer: &["52枚", "ごじゅうにまい"],
    },
    Question {
        id: 66,
        question: "ピアノの鍵盤は白黒合わせて何個？",
        answer: &["88個", "はちじゅうはっこ"],
    },
    Question {
        id: 67,
        question: "人間の歯は全部で何本？",
        answer: &["32本", "さんじゅうにほん"],
    },
    Question {
        id: 68,
        question: "人間の指は片手で何本？",
        answer: &["5本", "ごほん"],
    },
    Question {
        id: 69,
        question: "三角形の角の数は？",
        answer: &["3つ", "みっつ"],
    },
    Question {
        id: 70,
        question: "四角形の角の数は？",
        answer: &["4つ", "よっつ"],
    },
    Question {
        id: 71,
        question: "正六角形の角の数は？",
        answer: &["6つ", "むっつ"],
    },
    Question {
        id: 72,
        question: "音楽の三大要素の一つは？",
        answer: &["リズム"],
    },
    Question {
        id: 73,
        question: "日本の三権分立の一つは？",
        answer: &["立法", "りっぽう"],
    },
    Question {
        id: 74,
        question: "衆議院の任期は何年？",
        answer: &["4年", "よねん"],
    },
    Question {
        id: 75,
        question: "参議院の任期は何年？",
        answer: &["6年", "ろくねん"],
    },
    Question {
        id: 76,
        question: "日本の義務教育は何年間？",
        answer: &["9年", "きゅうねん"],
    },
    Question {
        id: 77,
        question: "小学校は何年生まで？",
        answer: &["6年生", "ろくねんせい"],
    },
    Question {
        id: 78,
        question: "中学校は何年生まで？",
        answer: &["3年生", "さんねんせい"],
    },
    Question {
        id: 79,
        question: "高校は何年生まで？",
        answer: &["3年生", "さんねんせい"],
    },
    Question {
        id: 80,
        question: "成人年齢は何歳？",
        answer: &["18歳", "じゅうはっさい"],
    },
    Question {
        id: 81,
        question: "選挙権は何歳から？",
        answer: &["18歳", "じゅうはっさい"],
    },
    Question {
        id: 82,
        question: "お酒を飲めるのは何歳から？",
        answer: &["20歳", "はたち", "にじゅっさい"],
    },
    Question {
        id: 83,
        question: "タバコを吸えるのは何歳から？",
        answer: &["20歳", "はたち", "にじゅっさい"],
    },
    Question {
        id: 84,
        question: "車の運転免許は何歳から？",
        answer: &["18歳", "じゅうはっさい"],
    },
    Question {
        id: 85,
        question: "日本の国技は？",
        answer: &["相撲", "すもう"],
    },
    Question {
        id: 86,
        question: "相撲の最高位は？",
        answer: &["横綱", "よこづな"],
    },
    Question {
        id: 87,
        question: "囲碁の盤は何路盤が標準？",
        answer: &["19路盤", "じゅうきゅうろばん"],
    },
    Question {
        id: 88,
        question: "将棋盤のマス目は縦横何マス？",
        answer: &["9マス", "きゅうます"],
    },
    Question {
        id: 89,
        question: "金メダルは何位？",
        answer: &["1位", "いちい"],
    },
    Question {
        id: 90,
        question: "銀メダルは何位？",
        answer: &["2位", "にい"],
    },
    Question {
        id: 91,
        question: "銅メダルは何位？",
        answer: &["3位", "さんい"],
    },
    Question {
        id: 92,
        question: "1ダースは何個？",
        answer: &["12個", "じゅうにこ"],
    },
    Question {
        id: 93,
        question: "1グロスは何ダース？",
        answer: &["12ダース", "じゅうにだーす"],
    },
    Question {
        id: 94,
        question: "水は何度で凍る？",
        answer: &["0度", "れいど"],
    },
    Question {
        id: 95,
        question: "水は何度で沸騰する？",
        answer: &["100度", "ひゃくど"],
    },
    Question {
        id: 96,
        question: "日本のお正月は何月何日？",
        answer: &["1月1日", "いちがつついたち"],
    },
    Question {
        id: 97,
        question: "クリスマスは何月何日？",
        answer: &[
            "12月25日",
            "じゅうにがつにじゅうごにち",
            "ジュウニガツニジュウゴニチ",
        ],
    },
    Question {
        id: 98,
        question: "バレンタインデーは何月何日？",
        answer: &["2月14日", "にがつじゅうよっか"],
    },
    Question {
        id: 99,
        question: "ひな祭りは何月何日？",
        answer: &["3月3日", "さんがつみっか"],
    },
    Question {
        id: 100,
        question: "こどもの日は何月何日？",
        answer: &["5月5日", "ごがついつか"],
    },
];

#[derive(Clone, Copy, PartialEq, Eq)]
enum Screen {
    Home,
    NameInput,
    GameSelect,
    GameAnswerList,
    GamePrivateInput,
    GameReveal,
    Clear,
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
                // Katakana -> Hiragana
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

#[component]
pub fn App() -> impl IntoView {
    let (screen, set_screen) = signal(Screen::Home);
    let (player_count, set_player_count) = signal(5usize);
    let (player_names, set_player_names) = signal(
        (1..=5)
            .map(|i| format!("プレイヤー{i}"))
            .collect::<Vec<String>>(),
    );

    let (success_streak, set_success_streak) = signal(0usize);
    let (current_player, set_current_player) = signal(0usize);
    let (answered_questions, set_answered_questions) = signal(Vec::<u32>::new());
    let (selected_question_idx, set_selected_question_idx) = signal(None::<usize>);
    let (answers, set_answers) = signal(Vec::<String>::new());
    let (input_player_idx, set_input_player_idx) = signal(None::<usize>);
    let (temp_answer, set_temp_answer) = signal(String::new());
    let (round_result, set_round_result) = signal(None::<bool>);

    let update_player_count = move |new_count: usize| {
        set_player_count.set(new_count);
        set_player_names.update(|names| {
            let mut updated = (1..=new_count)
                .map(|i| format!("プレイヤー{i}"))
                .collect::<Vec<String>>();

            for (idx, name) in names.iter().enumerate().take(new_count) {
                updated[idx] = name.clone();
            }
            *names = updated;
        });
    };

    let start_game = move || {
        set_success_streak.set(0);
        set_current_player.set(0);
        set_answered_questions.set(Vec::new());
        set_selected_question_idx.set(None);
        set_answers.set(Vec::new());
        set_input_player_idx.set(None);
        set_temp_answer.set(String::new());
        set_round_result.set(None);
        set_screen.set(Screen::GameSelect);
    };

    let selected_question = move || selected_question_idx.get().map(|idx| QUESTIONS[idx]);

    let all_answered = move || {
        let player_total = player_count.get();
        let current_answers = answers.get();
        current_answers.len() == player_total
            && current_answers.iter().all(|a| !a.trim().is_empty())
    };

    let confirm_round = move || {
        let question = match selected_question() {
            Some(q) => q,
            None => return,
        };

        let all_correct = answers
            .get()
            .iter()
            .all(|answer| is_correct_answer(answer, question.answer));

        set_round_result.set(Some(all_correct));

        if all_correct {
            let new_streak = success_streak.get_untracked() + 1;
            set_success_streak.set(new_streak);
            set_answered_questions.update(|answered| answered.push(question.id));

            if new_streak >= 5 {
                set_screen.set(Screen::Clear);
                return;
            }

            let next_player = (current_player.get_untracked() + 1) % player_count.get_untracked();
            set_current_player.set(next_player);
        } else {
            set_success_streak.set(0);
            set_current_player.set(0);
            set_answered_questions.set(Vec::new());
        }

        set_selected_question_idx.set(None);
        set_answers.set(Vec::new());
        set_input_player_idx.set(None);
        set_temp_answer.set(String::new());
        set_screen.set(Screen::GameSelect);
    };

    view! {
        <main class="app-root">
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

                                <button class="primary-btn" on:click=move |_| start_game()>
                                    "🏆 ゲームスタート"
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
                                        set_screen.set(Screen::Home);
                                        set_selected_question_idx.set(None);
                                        set_answers.set(Vec::new());
                                        set_round_result.set(None);
                                    }
                                >
                                    "✖ ゲームを終了"
                                </button>

                                <section class="card progress-card">
                                    <h3 class="progress-title">"連続成功"</h3>
                                    <div class="progress-track">
                                        {(1..=5)
                                            .map(|step| {
                                                let active = success_streak.get() >= step;
                                                view! {
                                                    <div class="progress-step">
                                                        <div class=move || {
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
                                            names
                                                .get(current_player.get())
                                                .cloned()
                                                .unwrap_or_else(|| "プレイヤー1".to_string())
                                        }}
                                    </p>
                                </section>

                                {move || {
                                    if let Some(result) = round_result.get() {
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

                                {move || {
                                    match screen.get() {
                                        Screen::GameSelect => {
                                            view! {
                                                <section class="card">
                                                    <h2 class="section-title">"問題を選択してください"</h2>
                                                    <p class="hint">"みんなが答えられそうな問題を選ぼう"</p>
                                                    <div class="question-list">
                                                        {QUESTIONS
                                                            .iter()
                                                            .map(|q| {
                                                                let is_answered = answered_questions
                                                                    .get()
                                                                    .contains(&q.id);
                                                                view! {
                                                                    <button
                                                                        class=move || {
                                                                            if is_answered {
                                                                                "question-btn done"
                                                                            } else {
                                                                                "question-btn"
                                                                            }
                                                                        }
                                                                        disabled=is_answered
                                                                        on:click=move |_| {
                                                                            set_selected_question_idx
                                                                                .set(Some((q.id - 1) as usize));
                                                                            set_answers
                                                                                .set(
                                                                                    vec![
                                                                                        String::new();
                                                                                        player_count.get_untracked()
                                                                                    ],
                                                                                );
                                                                            set_round_result.set(None);
                                                                            set_screen.set(Screen::GameAnswerList);
                                                                        }
                                                                    >
                                                                        <span class="qid">{q.id}</span>
                                                                        <span class="qtext">{q.question}</span>
                                                                        {if is_answered {
                                                                            view! { <span class="qdone">"✓"</span> }
                                                                                .into_any()
                                                                        } else {
                                                                            view! { <></> }.into_any()
                                                                        }}
                                                                    </button>
                                                                }
                                                            })
                                                            .collect_view()}
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
                                                            set_selected_question_idx.set(None);
                                                            set_answers.set(Vec::new());
                                                            set_screen.set(Screen::GameSelect);
                                                        }
                                                    >
                                                        "← 問題選択に戻る"
                                                    </button>

                                                    {move || {
                                                        if let Some(question) = selected_question() {
                                                            view! {
                                                                <div class="question-box">
                                                                    <p>{format!("問題 {}", question.id)}</p>
                                                                    <h3>{question.question}</h3>
                                                                </div>
                                                            }
                                                                .into_any()
                                                        } else {
                                                            view! { <></> }.into_any()
                                                        }
                                                    }}

                                                    <p class="hint center">"順番に回答してください（他の人に見られないように）"</p>

                                                    <div class="answer-list">
                                                        {(0..player_count.get())
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
                                                                                player_names
                                                                                    .get()
                                                                                    .get(idx)
                                                                                    .cloned()
                                                                                    .unwrap_or_else(|| {
                                                                                        format!("プレイヤー{}", idx + 1)
                                                                                    })
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
                                                            .collect_view()}
                                                    </div>

                                                    {move || {
                                                        if all_answered() {
                                                            view! {
                                                                <button
                                                                    class="accent-btn"
                                                                    on:click=move |_| set_screen.set(Screen::GameReveal)
                                                                >
                                                                    "答えを見る"
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
                                                                    .and_then(|idx| player_names
                                                                        .get()
                                                                        .get(idx)
                                                                        .cloned())
                                                                    .unwrap_or_else(|| "プレイヤー".to_string())
                                                            }}
                                                        </h3>
                                                        <p>"他の人は見ないでください"</p>
                                                    </div>

                                                    {move || {
                                                        if let Some(question) = selected_question() {
                                                            view! {
                                                                <div class="question-box">
                                                                    <p>{format!("問題 {}", question.id)}</p>
                                                                    <h3>{question.question}</h3>
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
                                                                    <p>{format!("問題 {}", question.id)}</p>
                                                                    <h3>{question.question}</h3>
                                                                    <div class="correct-box">
                                                                        <small>"正解"</small>
                                                                        <strong>{question.answer.first().copied().unwrap_or("")}</strong>
                                                                    </div>
                                                                </div>
                                                            }
                                                                .into_any()
                                                        } else {
                                                            view! { <></> }.into_any()
                                                        }
                                                    }}

                                                    <div class="result-list">
                                                        {(0..player_count.get())
                                                            .map(|idx| {
                                                                let answer = answers
                                                                    .get()
                                                                    .get(idx)
                                                                    .cloned()
                                                                    .unwrap_or_default();
                                                                let correct = selected_question()
                                                                    .map(|q| is_correct_answer(&answer, q.answer))
                                                                    .unwrap_or(false);
                                                                let row_class = if correct {
                                                                    "result-row ok"
                                                                } else {
                                                                    "result-row ng"
                                                                };
                                                                view! {
                                                                    <div class=row_class>
                                                                        <div>
                                                                            <p class="player-label">
                                                                                {move || {
                                                                                    player_names
                                                                                        .get()
                                                                                        .get(idx)
                                                                                        .cloned()
                                                                                        .unwrap_or_else(|| {
                                                                                            format!("プレイヤー{}", idx + 1)
                                                                                        })
                                                                                }}
                                                                            </p>
                                                                            <p class="player-answer">{answer}</p>
                                                                        </div>
                                                                        <span class="judge-mark">
                                                                            {if correct { "✓" } else { "✗" }}
                                                                        </span>
                                                                    </div>
                                                                }
                                                            })
                                                            .collect_view()}
                                                    </div>

                                                    <button class="primary-btn" on:click=move |_| confirm_round()>
                                                        "結果を確定"
                                                    </button>
                                                </section>
                                            }
                                                .into_any()
                                        }
                                        _ => view! { <></> }.into_any(),
                                    }
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
                                <p class="clear-text">"5人全員で5問連続正解を達成しました"</p>

                                <section class="card clear-card">
                                    <p>
                                        "チームワークばっちり！"
                                    </p>
                                    <p class="party">"🎉"</p>
                                </section>

                                <button class="warn-btn" on:click=move |_| set_screen.set(Screen::Home)>
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

#[cfg(test)]
mod tests {
    mod normalize_answer_tests {
        use super::super::normalize_answer;

        #[test]
        fn normalize_answer_removes_spaces() {
            let input = "  東 京　";
            assert_eq!(normalize_answer(input), "東京");
        }

        #[test]
        fn normalize_answer_lowercases_ascii() {
            let input = "  H2O ";
            assert_eq!(normalize_answer(input), "h2o");
        }

        #[test]
        fn normalize_answer_keeps_normalized_ascii_input() {
            let input = "  h2o ";
            assert_eq!(normalize_answer(input), "h2o");
        }

        #[test]
        fn normalize_answer_normalizes_fullwidth_digits() {
            let input = "３６５日";
            assert_eq!(normalize_answer(input), "365日");
        }

        #[test]
        fn normalize_answer_keeps_normalized_digit_input() {
            let input = "365ニチ";
            assert_eq!(normalize_answer(input), "365にち");
        }

        #[test]
        fn normalize_answer_normalizes_halfwidth_katakana() {
            let input = "ﾎｯｶｲﾄﾞｳ";
            assert_eq!(normalize_answer(input), "ほっかいどう");
        }

        #[test]
        fn normalize_answer_keeps_normalized_katakana_input() {
            let input = "ホッカイ ドウ";
            assert_eq!(normalize_answer(input), "ほっかいどう");
        }

        #[test]
        fn normalize_answer_keeps_normalized_hiragana_input() {
            let input = "ほっ かいどう";
            assert_eq!(normalize_answer(input), "ほっかいどう");
        }

        #[test]
        fn normalize_answer_keeps_hiragana_prolonged_sound_mark() {
            let input = "すいせー";
            assert_eq!(normalize_answer(input), "すいせー");
        }

        #[test]
        fn normalize_answer_keeps_hiragana_hyphen() {
            let input = "すいせ-";
            assert_eq!(normalize_answer(input), "すいせ-");
        }
    }

    mod is_correct_answer_tests {
        use super::super::is_correct_answer;

        #[test]
        fn matches_exact_kanji_answer() {
            assert!(is_correct_answer("水星", &["水星", "すいせい"]));
        }

        #[test]
        fn matches_katakana_variant_via_normalization() {
            assert!(is_correct_answer("スイセイ", &["水星", "すいせい"]));
        }

        #[test]
        fn matches_partial_kanji_prefix() {
            assert!(!is_correct_answer("水", &["水星", "すいせい"]));
        }

        #[test]
        fn matches_partial_hiragana_prefix() {
            assert!(!is_correct_answer("すいせ", &["水星", "すいせい"]));
        }

        #[test]
        fn returns_false_for_unmatched_answer() {
            assert!(!is_correct_answer("金星", &["水星", "すいせい"]));
        }
    }
}
