# Frontend/Backend 責務分担ガイド

このプロジェクトでは「フロントエンドは UI に集中し、判断・処理はバックエンドに寄せる」を基本方針にします。

## 1. 役割の定義

| レイヤー | 担当すること | 担当しないこと |
| --- | --- | --- |
| Frontend (`src/`) | 画面描画、入力受付、画面遷移、ローディング/エラー表示 | 正誤判定、入力正規化、問題抽選、進行ルール判定 |
| Backend (`src-tauri/`) | クイズルール、入力検証、正規化、問題抽選、ゲーム状態更新 | 画面表示、入力コンポーネント制御 |

## 2. クイズ機能の責務分担

| 機能 | 置き場所 |
| --- | --- |
| プレイヤー名入力 UI | Frontend |
| 問題一覧表示/選択 UI | Frontend |
| 回答入力 UI（伏せ入力） | Frontend |
| 連続成功の計算 | Backend |
| 回答正規化（NFKC, 空白除去, かな変換） | Backend |
| 正誤判定 | Backend |
| 問題抽選（100 問のサンプリング） | Backend |
| クリア判定（5 連続成功） | Backend |
| 不正入力チェック（人数、回答配列サイズなど） | Backend |

## 3. 実装ルール

1. 画面操作で必要なデータは `invoke` で Backend コマンドを呼び出して取得/更新する。
2. Frontend は Backend から返ってきた結果をそのまま表示する。
3. ルール変更が発生したら、まず Backend のコマンドとテストを更新してから UI を調整する。
4. UI 側で正誤判定ロジックを再実装しない（表示専用の補助処理のみ可）。

## 4. コマンド設計の最小単位（推奨）

- `start_game(player_count: usize) -> GameState`
  - 問題抽選と初期状態を返す
- `submit_answers(state: GameState, answers: Vec<String>) -> RoundResult`
  - 正規化/正誤判定/連続成功更新をまとめて実行
- `select_question(state: GameState, question_id: u32) -> GameState`
  - 選択可能かを検証して状態を更新

`GameState` や `RoundResult` は `serde` でシリアライズ可能な構造体として定義し、Frontend/Backend 間で共通の契約にします。

## 5. このリポジトリの現状と次アクション

現状は `src/app.rs` に UI とゲームロジックが同居しています。  
以下の順番で分離すると安全です。

1. `src-tauri/src/lib.rs` にクイズ用コマンドと DTO（`GameState`, `RoundResult`）を追加
2. `normalize_answer`, `is_correct_answer`, `sample_round_questions`, `evaluate_confirm_round` を Backend 側へ移設
3. `src/app.rs` からロジック呼び出しを `invoke` ベースに置換
4. 既存テストを Backend 側へ移し、UI 側には画面遷移テストのみ残す
