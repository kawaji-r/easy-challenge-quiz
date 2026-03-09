use leptos::prelude::*;
#[cfg(not(target_arch = "wasm32"))]
use std::sync::atomic::{AtomicU64, Ordering};
use unicode_normalization::UnicodeNormalization;

#[derive(Clone, Copy)]
struct Question {
    id: u32,
    question: &'static str,
    answer: &'static [&'static str],
}

const BASE_QUESTIONS: [Question; 500] = [
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
    Question {
        id: 101,
        question: "西暦1601年は何世紀？",
        answer: &["17世紀"],
    },
    Question {
        id: 102,
        question: "西暦1602年は何世紀？",
        answer: &["17世紀"],
    },
    Question {
        id: 103,
        question: "西暦1603年は何世紀？",
        answer: &["17世紀"],
    },
    Question {
        id: 104,
        question: "西暦1604年は何世紀？",
        answer: &["17世紀"],
    },
    Question {
        id: 105,
        question: "西暦1605年は何世紀？",
        answer: &["17世紀"],
    },
    Question {
        id: 106,
        question: "西暦1606年は何世紀？",
        answer: &["17世紀"],
    },
    Question {
        id: 107,
        question: "西暦1607年は何世紀？",
        answer: &["17世紀"],
    },
    Question {
        id: 108,
        question: "西暦1608年は何世紀？",
        answer: &["17世紀"],
    },
    Question {
        id: 109,
        question: "西暦1609年は何世紀？",
        answer: &["17世紀"],
    },
    Question {
        id: 110,
        question: "西暦1610年は何世紀？",
        answer: &["17世紀"],
    },
    Question {
        id: 111,
        question: "西暦1611年は何世紀？",
        answer: &["17世紀"],
    },
    Question {
        id: 112,
        question: "西暦1612年は何世紀？",
        answer: &["17世紀"],
    },
    Question {
        id: 113,
        question: "西暦1613年は何世紀？",
        answer: &["17世紀"],
    },
    Question {
        id: 114,
        question: "西暦1614年は何世紀？",
        answer: &["17世紀"],
    },
    Question {
        id: 115,
        question: "西暦1615年は何世紀？",
        answer: &["17世紀"],
    },
    Question {
        id: 116,
        question: "西暦1616年は何世紀？",
        answer: &["17世紀"],
    },
    Question {
        id: 117,
        question: "西暦1617年は何世紀？",
        answer: &["17世紀"],
    },
    Question {
        id: 118,
        question: "西暦1618年は何世紀？",
        answer: &["17世紀"],
    },
    Question {
        id: 119,
        question: "西暦1619年は何世紀？",
        answer: &["17世紀"],
    },
    Question {
        id: 120,
        question: "西暦1620年は何世紀？",
        answer: &["17世紀"],
    },
    Question {
        id: 121,
        question: "西暦1621年は何世紀？",
        answer: &["17世紀"],
    },
    Question {
        id: 122,
        question: "西暦1622年は何世紀？",
        answer: &["17世紀"],
    },
    Question {
        id: 123,
        question: "西暦1623年は何世紀？",
        answer: &["17世紀"],
    },
    Question {
        id: 124,
        question: "西暦1624年は何世紀？",
        answer: &["17世紀"],
    },
    Question {
        id: 125,
        question: "西暦1625年は何世紀？",
        answer: &["17世紀"],
    },
    Question {
        id: 126,
        question: "西暦1626年は何世紀？",
        answer: &["17世紀"],
    },
    Question {
        id: 127,
        question: "西暦1627年は何世紀？",
        answer: &["17世紀"],
    },
    Question {
        id: 128,
        question: "西暦1628年は何世紀？",
        answer: &["17世紀"],
    },
    Question {
        id: 129,
        question: "西暦1629年は何世紀？",
        answer: &["17世紀"],
    },
    Question {
        id: 130,
        question: "西暦1630年は何世紀？",
        answer: &["17世紀"],
    },
    Question {
        id: 131,
        question: "西暦1631年は何世紀？",
        answer: &["17世紀"],
    },
    Question {
        id: 132,
        question: "西暦1632年は何世紀？",
        answer: &["17世紀"],
    },
    Question {
        id: 133,
        question: "西暦1633年は何世紀？",
        answer: &["17世紀"],
    },
    Question {
        id: 134,
        question: "西暦1634年は何世紀？",
        answer: &["17世紀"],
    },
    Question {
        id: 135,
        question: "西暦1635年は何世紀？",
        answer: &["17世紀"],
    },
    Question {
        id: 136,
        question: "西暦1636年は何世紀？",
        answer: &["17世紀"],
    },
    Question {
        id: 137,
        question: "西暦1637年は何世紀？",
        answer: &["17世紀"],
    },
    Question {
        id: 138,
        question: "西暦1638年は何世紀？",
        answer: &["17世紀"],
    },
    Question {
        id: 139,
        question: "西暦1639年は何世紀？",
        answer: &["17世紀"],
    },
    Question {
        id: 140,
        question: "西暦1640年は何世紀？",
        answer: &["17世紀"],
    },
    Question {
        id: 141,
        question: "西暦1641年は何世紀？",
        answer: &["17世紀"],
    },
    Question {
        id: 142,
        question: "西暦1642年は何世紀？",
        answer: &["17世紀"],
    },
    Question {
        id: 143,
        question: "西暦1643年は何世紀？",
        answer: &["17世紀"],
    },
    Question {
        id: 144,
        question: "西暦1644年は何世紀？",
        answer: &["17世紀"],
    },
    Question {
        id: 145,
        question: "西暦1645年は何世紀？",
        answer: &["17世紀"],
    },
    Question {
        id: 146,
        question: "西暦1646年は何世紀？",
        answer: &["17世紀"],
    },
    Question {
        id: 147,
        question: "西暦1647年は何世紀？",
        answer: &["17世紀"],
    },
    Question {
        id: 148,
        question: "西暦1648年は何世紀？",
        answer: &["17世紀"],
    },
    Question {
        id: 149,
        question: "西暦1649年は何世紀？",
        answer: &["17世紀"],
    },
    Question {
        id: 150,
        question: "西暦1650年は何世紀？",
        answer: &["17世紀"],
    },
    Question {
        id: 151,
        question: "西暦1651年は何世紀？",
        answer: &["17世紀"],
    },
    Question {
        id: 152,
        question: "西暦1652年は何世紀？",
        answer: &["17世紀"],
    },
    Question {
        id: 153,
        question: "西暦1653年は何世紀？",
        answer: &["17世紀"],
    },
    Question {
        id: 154,
        question: "西暦1654年は何世紀？",
        answer: &["17世紀"],
    },
    Question {
        id: 155,
        question: "西暦1655年は何世紀？",
        answer: &["17世紀"],
    },
    Question {
        id: 156,
        question: "西暦1656年は何世紀？",
        answer: &["17世紀"],
    },
    Question {
        id: 157,
        question: "西暦1657年は何世紀？",
        answer: &["17世紀"],
    },
    Question {
        id: 158,
        question: "西暦1658年は何世紀？",
        answer: &["17世紀"],
    },
    Question {
        id: 159,
        question: "西暦1659年は何世紀？",
        answer: &["17世紀"],
    },
    Question {
        id: 160,
        question: "西暦1660年は何世紀？",
        answer: &["17世紀"],
    },
    Question {
        id: 161,
        question: "西暦1661年は何世紀？",
        answer: &["17世紀"],
    },
    Question {
        id: 162,
        question: "西暦1662年は何世紀？",
        answer: &["17世紀"],
    },
    Question {
        id: 163,
        question: "西暦1663年は何世紀？",
        answer: &["17世紀"],
    },
    Question {
        id: 164,
        question: "西暦1664年は何世紀？",
        answer: &["17世紀"],
    },
    Question {
        id: 165,
        question: "西暦1665年は何世紀？",
        answer: &["17世紀"],
    },
    Question {
        id: 166,
        question: "西暦1666年は何世紀？",
        answer: &["17世紀"],
    },
    Question {
        id: 167,
        question: "西暦1667年は何世紀？",
        answer: &["17世紀"],
    },
    Question {
        id: 168,
        question: "西暦1668年は何世紀？",
        answer: &["17世紀"],
    },
    Question {
        id: 169,
        question: "西暦1669年は何世紀？",
        answer: &["17世紀"],
    },
    Question {
        id: 170,
        question: "西暦1670年は何世紀？",
        answer: &["17世紀"],
    },
    Question {
        id: 171,
        question: "西暦1671年は何世紀？",
        answer: &["17世紀"],
    },
    Question {
        id: 172,
        question: "西暦1672年は何世紀？",
        answer: &["17世紀"],
    },
    Question {
        id: 173,
        question: "西暦1673年は何世紀？",
        answer: &["17世紀"],
    },
    Question {
        id: 174,
        question: "西暦1674年は何世紀？",
        answer: &["17世紀"],
    },
    Question {
        id: 175,
        question: "西暦1675年は何世紀？",
        answer: &["17世紀"],
    },
    Question {
        id: 176,
        question: "西暦1676年は何世紀？",
        answer: &["17世紀"],
    },
    Question {
        id: 177,
        question: "西暦1677年は何世紀？",
        answer: &["17世紀"],
    },
    Question {
        id: 178,
        question: "西暦1678年は何世紀？",
        answer: &["17世紀"],
    },
    Question {
        id: 179,
        question: "西暦1679年は何世紀？",
        answer: &["17世紀"],
    },
    Question {
        id: 180,
        question: "西暦1680年は何世紀？",
        answer: &["17世紀"],
    },
    Question {
        id: 181,
        question: "西暦1681年は何世紀？",
        answer: &["17世紀"],
    },
    Question {
        id: 182,
        question: "西暦1682年は何世紀？",
        answer: &["17世紀"],
    },
    Question {
        id: 183,
        question: "西暦1683年は何世紀？",
        answer: &["17世紀"],
    },
    Question {
        id: 184,
        question: "西暦1684年は何世紀？",
        answer: &["17世紀"],
    },
    Question {
        id: 185,
        question: "西暦1685年は何世紀？",
        answer: &["17世紀"],
    },
    Question {
        id: 186,
        question: "西暦1686年は何世紀？",
        answer: &["17世紀"],
    },
    Question {
        id: 187,
        question: "西暦1687年は何世紀？",
        answer: &["17世紀"],
    },
    Question {
        id: 188,
        question: "西暦1688年は何世紀？",
        answer: &["17世紀"],
    },
    Question {
        id: 189,
        question: "西暦1689年は何世紀？",
        answer: &["17世紀"],
    },
    Question {
        id: 190,
        question: "西暦1690年は何世紀？",
        answer: &["17世紀"],
    },
    Question {
        id: 191,
        question: "西暦1691年は何世紀？",
        answer: &["17世紀"],
    },
    Question {
        id: 192,
        question: "西暦1692年は何世紀？",
        answer: &["17世紀"],
    },
    Question {
        id: 193,
        question: "西暦1693年は何世紀？",
        answer: &["17世紀"],
    },
    Question {
        id: 194,
        question: "西暦1694年は何世紀？",
        answer: &["17世紀"],
    },
    Question {
        id: 195,
        question: "西暦1695年は何世紀？",
        answer: &["17世紀"],
    },
    Question {
        id: 196,
        question: "西暦1696年は何世紀？",
        answer: &["17世紀"],
    },
    Question {
        id: 197,
        question: "西暦1697年は何世紀？",
        answer: &["17世紀"],
    },
    Question {
        id: 198,
        question: "西暦1698年は何世紀？",
        answer: &["17世紀"],
    },
    Question {
        id: 199,
        question: "西暦1699年は何世紀？",
        answer: &["17世紀"],
    },
    Question {
        id: 200,
        question: "西暦1700年は何世紀？",
        answer: &["17世紀"],
    },
    Question {
        id: 201,
        question: "西暦1701年は何世紀？",
        answer: &["18世紀"],
    },
    Question {
        id: 202,
        question: "西暦1702年は何世紀？",
        answer: &["18世紀"],
    },
    Question {
        id: 203,
        question: "西暦1703年は何世紀？",
        answer: &["18世紀"],
    },
    Question {
        id: 204,
        question: "西暦1704年は何世紀？",
        answer: &["18世紀"],
    },
    Question {
        id: 205,
        question: "西暦1705年は何世紀？",
        answer: &["18世紀"],
    },
    Question {
        id: 206,
        question: "西暦1706年は何世紀？",
        answer: &["18世紀"],
    },
    Question {
        id: 207,
        question: "西暦1707年は何世紀？",
        answer: &["18世紀"],
    },
    Question {
        id: 208,
        question: "西暦1708年は何世紀？",
        answer: &["18世紀"],
    },
    Question {
        id: 209,
        question: "西暦1709年は何世紀？",
        answer: &["18世紀"],
    },
    Question {
        id: 210,
        question: "西暦1710年は何世紀？",
        answer: &["18世紀"],
    },
    Question {
        id: 211,
        question: "西暦1711年は何世紀？",
        answer: &["18世紀"],
    },
    Question {
        id: 212,
        question: "西暦1712年は何世紀？",
        answer: &["18世紀"],
    },
    Question {
        id: 213,
        question: "西暦1713年は何世紀？",
        answer: &["18世紀"],
    },
    Question {
        id: 214,
        question: "西暦1714年は何世紀？",
        answer: &["18世紀"],
    },
    Question {
        id: 215,
        question: "西暦1715年は何世紀？",
        answer: &["18世紀"],
    },
    Question {
        id: 216,
        question: "西暦1716年は何世紀？",
        answer: &["18世紀"],
    },
    Question {
        id: 217,
        question: "西暦1717年は何世紀？",
        answer: &["18世紀"],
    },
    Question {
        id: 218,
        question: "西暦1718年は何世紀？",
        answer: &["18世紀"],
    },
    Question {
        id: 219,
        question: "西暦1719年は何世紀？",
        answer: &["18世紀"],
    },
    Question {
        id: 220,
        question: "西暦1720年は何世紀？",
        answer: &["18世紀"],
    },
    Question {
        id: 221,
        question: "西暦1721年は何世紀？",
        answer: &["18世紀"],
    },
    Question {
        id: 222,
        question: "西暦1722年は何世紀？",
        answer: &["18世紀"],
    },
    Question {
        id: 223,
        question: "西暦1723年は何世紀？",
        answer: &["18世紀"],
    },
    Question {
        id: 224,
        question: "西暦1724年は何世紀？",
        answer: &["18世紀"],
    },
    Question {
        id: 225,
        question: "西暦1725年は何世紀？",
        answer: &["18世紀"],
    },
    Question {
        id: 226,
        question: "西暦1726年は何世紀？",
        answer: &["18世紀"],
    },
    Question {
        id: 227,
        question: "西暦1727年は何世紀？",
        answer: &["18世紀"],
    },
    Question {
        id: 228,
        question: "西暦1728年は何世紀？",
        answer: &["18世紀"],
    },
    Question {
        id: 229,
        question: "西暦1729年は何世紀？",
        answer: &["18世紀"],
    },
    Question {
        id: 230,
        question: "西暦1730年は何世紀？",
        answer: &["18世紀"],
    },
    Question {
        id: 231,
        question: "西暦1731年は何世紀？",
        answer: &["18世紀"],
    },
    Question {
        id: 232,
        question: "西暦1732年は何世紀？",
        answer: &["18世紀"],
    },
    Question {
        id: 233,
        question: "西暦1733年は何世紀？",
        answer: &["18世紀"],
    },
    Question {
        id: 234,
        question: "西暦1734年は何世紀？",
        answer: &["18世紀"],
    },
    Question {
        id: 235,
        question: "西暦1735年は何世紀？",
        answer: &["18世紀"],
    },
    Question {
        id: 236,
        question: "西暦1736年は何世紀？",
        answer: &["18世紀"],
    },
    Question {
        id: 237,
        question: "西暦1737年は何世紀？",
        answer: &["18世紀"],
    },
    Question {
        id: 238,
        question: "西暦1738年は何世紀？",
        answer: &["18世紀"],
    },
    Question {
        id: 239,
        question: "西暦1739年は何世紀？",
        answer: &["18世紀"],
    },
    Question {
        id: 240,
        question: "西暦1740年は何世紀？",
        answer: &["18世紀"],
    },
    Question {
        id: 241,
        question: "西暦1741年は何世紀？",
        answer: &["18世紀"],
    },
    Question {
        id: 242,
        question: "西暦1742年は何世紀？",
        answer: &["18世紀"],
    },
    Question {
        id: 243,
        question: "西暦1743年は何世紀？",
        answer: &["18世紀"],
    },
    Question {
        id: 244,
        question: "西暦1744年は何世紀？",
        answer: &["18世紀"],
    },
    Question {
        id: 245,
        question: "西暦1745年は何世紀？",
        answer: &["18世紀"],
    },
    Question {
        id: 246,
        question: "西暦1746年は何世紀？",
        answer: &["18世紀"],
    },
    Question {
        id: 247,
        question: "西暦1747年は何世紀？",
        answer: &["18世紀"],
    },
    Question {
        id: 248,
        question: "西暦1748年は何世紀？",
        answer: &["18世紀"],
    },
    Question {
        id: 249,
        question: "西暦1749年は何世紀？",
        answer: &["18世紀"],
    },
    Question {
        id: 250,
        question: "西暦1750年は何世紀？",
        answer: &["18世紀"],
    },
    Question {
        id: 251,
        question: "西暦1751年は何世紀？",
        answer: &["18世紀"],
    },
    Question {
        id: 252,
        question: "西暦1752年は何世紀？",
        answer: &["18世紀"],
    },
    Question {
        id: 253,
        question: "西暦1753年は何世紀？",
        answer: &["18世紀"],
    },
    Question {
        id: 254,
        question: "西暦1754年は何世紀？",
        answer: &["18世紀"],
    },
    Question {
        id: 255,
        question: "西暦1755年は何世紀？",
        answer: &["18世紀"],
    },
    Question {
        id: 256,
        question: "西暦1756年は何世紀？",
        answer: &["18世紀"],
    },
    Question {
        id: 257,
        question: "西暦1757年は何世紀？",
        answer: &["18世紀"],
    },
    Question {
        id: 258,
        question: "西暦1758年は何世紀？",
        answer: &["18世紀"],
    },
    Question {
        id: 259,
        question: "西暦1759年は何世紀？",
        answer: &["18世紀"],
    },
    Question {
        id: 260,
        question: "西暦1760年は何世紀？",
        answer: &["18世紀"],
    },
    Question {
        id: 261,
        question: "西暦1761年は何世紀？",
        answer: &["18世紀"],
    },
    Question {
        id: 262,
        question: "西暦1762年は何世紀？",
        answer: &["18世紀"],
    },
    Question {
        id: 263,
        question: "西暦1763年は何世紀？",
        answer: &["18世紀"],
    },
    Question {
        id: 264,
        question: "西暦1764年は何世紀？",
        answer: &["18世紀"],
    },
    Question {
        id: 265,
        question: "西暦1765年は何世紀？",
        answer: &["18世紀"],
    },
    Question {
        id: 266,
        question: "西暦1766年は何世紀？",
        answer: &["18世紀"],
    },
    Question {
        id: 267,
        question: "西暦1767年は何世紀？",
        answer: &["18世紀"],
    },
    Question {
        id: 268,
        question: "西暦1768年は何世紀？",
        answer: &["18世紀"],
    },
    Question {
        id: 269,
        question: "西暦1769年は何世紀？",
        answer: &["18世紀"],
    },
    Question {
        id: 270,
        question: "西暦1770年は何世紀？",
        answer: &["18世紀"],
    },
    Question {
        id: 271,
        question: "西暦1771年は何世紀？",
        answer: &["18世紀"],
    },
    Question {
        id: 272,
        question: "西暦1772年は何世紀？",
        answer: &["18世紀"],
    },
    Question {
        id: 273,
        question: "西暦1773年は何世紀？",
        answer: &["18世紀"],
    },
    Question {
        id: 274,
        question: "西暦1774年は何世紀？",
        answer: &["18世紀"],
    },
    Question {
        id: 275,
        question: "西暦1775年は何世紀？",
        answer: &["18世紀"],
    },
    Question {
        id: 276,
        question: "西暦1776年は何世紀？",
        answer: &["18世紀"],
    },
    Question {
        id: 277,
        question: "西暦1777年は何世紀？",
        answer: &["18世紀"],
    },
    Question {
        id: 278,
        question: "西暦1778年は何世紀？",
        answer: &["18世紀"],
    },
    Question {
        id: 279,
        question: "西暦1779年は何世紀？",
        answer: &["18世紀"],
    },
    Question {
        id: 280,
        question: "西暦1780年は何世紀？",
        answer: &["18世紀"],
    },
    Question {
        id: 281,
        question: "西暦1781年は何世紀？",
        answer: &["18世紀"],
    },
    Question {
        id: 282,
        question: "西暦1782年は何世紀？",
        answer: &["18世紀"],
    },
    Question {
        id: 283,
        question: "西暦1783年は何世紀？",
        answer: &["18世紀"],
    },
    Question {
        id: 284,
        question: "西暦1784年は何世紀？",
        answer: &["18世紀"],
    },
    Question {
        id: 285,
        question: "西暦1785年は何世紀？",
        answer: &["18世紀"],
    },
    Question {
        id: 286,
        question: "西暦1786年は何世紀？",
        answer: &["18世紀"],
    },
    Question {
        id: 287,
        question: "西暦1787年は何世紀？",
        answer: &["18世紀"],
    },
    Question {
        id: 288,
        question: "西暦1788年は何世紀？",
        answer: &["18世紀"],
    },
    Question {
        id: 289,
        question: "西暦1789年は何世紀？",
        answer: &["18世紀"],
    },
    Question {
        id: 290,
        question: "西暦1790年は何世紀？",
        answer: &["18世紀"],
    },
    Question {
        id: 291,
        question: "西暦1791年は何世紀？",
        answer: &["18世紀"],
    },
    Question {
        id: 292,
        question: "西暦1792年は何世紀？",
        answer: &["18世紀"],
    },
    Question {
        id: 293,
        question: "西暦1793年は何世紀？",
        answer: &["18世紀"],
    },
    Question {
        id: 294,
        question: "西暦1794年は何世紀？",
        answer: &["18世紀"],
    },
    Question {
        id: 295,
        question: "西暦1795年は何世紀？",
        answer: &["18世紀"],
    },
    Question {
        id: 296,
        question: "西暦1796年は何世紀？",
        answer: &["18世紀"],
    },
    Question {
        id: 297,
        question: "西暦1797年は何世紀？",
        answer: &["18世紀"],
    },
    Question {
        id: 298,
        question: "西暦1798年は何世紀？",
        answer: &["18世紀"],
    },
    Question {
        id: 299,
        question: "西暦1799年は何世紀？",
        answer: &["18世紀"],
    },
    Question {
        id: 300,
        question: "西暦1800年は何世紀？",
        answer: &["18世紀"],
    },
    Question {
        id: 301,
        question: "西暦1801年は何世紀？",
        answer: &["19世紀"],
    },
    Question {
        id: 302,
        question: "西暦1802年は何世紀？",
        answer: &["19世紀"],
    },
    Question {
        id: 303,
        question: "西暦1803年は何世紀？",
        answer: &["19世紀"],
    },
    Question {
        id: 304,
        question: "西暦1804年は何世紀？",
        answer: &["19世紀"],
    },
    Question {
        id: 305,
        question: "西暦1805年は何世紀？",
        answer: &["19世紀"],
    },
    Question {
        id: 306,
        question: "西暦1806年は何世紀？",
        answer: &["19世紀"],
    },
    Question {
        id: 307,
        question: "西暦1807年は何世紀？",
        answer: &["19世紀"],
    },
    Question {
        id: 308,
        question: "西暦1808年は何世紀？",
        answer: &["19世紀"],
    },
    Question {
        id: 309,
        question: "西暦1809年は何世紀？",
        answer: &["19世紀"],
    },
    Question {
        id: 310,
        question: "西暦1810年は何世紀？",
        answer: &["19世紀"],
    },
    Question {
        id: 311,
        question: "西暦1811年は何世紀？",
        answer: &["19世紀"],
    },
    Question {
        id: 312,
        question: "西暦1812年は何世紀？",
        answer: &["19世紀"],
    },
    Question {
        id: 313,
        question: "西暦1813年は何世紀？",
        answer: &["19世紀"],
    },
    Question {
        id: 314,
        question: "西暦1814年は何世紀？",
        answer: &["19世紀"],
    },
    Question {
        id: 315,
        question: "西暦1815年は何世紀？",
        answer: &["19世紀"],
    },
    Question {
        id: 316,
        question: "西暦1816年は何世紀？",
        answer: &["19世紀"],
    },
    Question {
        id: 317,
        question: "西暦1817年は何世紀？",
        answer: &["19世紀"],
    },
    Question {
        id: 318,
        question: "西暦1818年は何世紀？",
        answer: &["19世紀"],
    },
    Question {
        id: 319,
        question: "西暦1819年は何世紀？",
        answer: &["19世紀"],
    },
    Question {
        id: 320,
        question: "西暦1820年は何世紀？",
        answer: &["19世紀"],
    },
    Question {
        id: 321,
        question: "西暦1821年は何世紀？",
        answer: &["19世紀"],
    },
    Question {
        id: 322,
        question: "西暦1822年は何世紀？",
        answer: &["19世紀"],
    },
    Question {
        id: 323,
        question: "西暦1823年は何世紀？",
        answer: &["19世紀"],
    },
    Question {
        id: 324,
        question: "西暦1824年は何世紀？",
        answer: &["19世紀"],
    },
    Question {
        id: 325,
        question: "西暦1825年は何世紀？",
        answer: &["19世紀"],
    },
    Question {
        id: 326,
        question: "西暦1826年は何世紀？",
        answer: &["19世紀"],
    },
    Question {
        id: 327,
        question: "西暦1827年は何世紀？",
        answer: &["19世紀"],
    },
    Question {
        id: 328,
        question: "西暦1828年は何世紀？",
        answer: &["19世紀"],
    },
    Question {
        id: 329,
        question: "西暦1829年は何世紀？",
        answer: &["19世紀"],
    },
    Question {
        id: 330,
        question: "西暦1830年は何世紀？",
        answer: &["19世紀"],
    },
    Question {
        id: 331,
        question: "西暦1831年は何世紀？",
        answer: &["19世紀"],
    },
    Question {
        id: 332,
        question: "西暦1832年は何世紀？",
        answer: &["19世紀"],
    },
    Question {
        id: 333,
        question: "西暦1833年は何世紀？",
        answer: &["19世紀"],
    },
    Question {
        id: 334,
        question: "西暦1834年は何世紀？",
        answer: &["19世紀"],
    },
    Question {
        id: 335,
        question: "西暦1835年は何世紀？",
        answer: &["19世紀"],
    },
    Question {
        id: 336,
        question: "西暦1836年は何世紀？",
        answer: &["19世紀"],
    },
    Question {
        id: 337,
        question: "西暦1837年は何世紀？",
        answer: &["19世紀"],
    },
    Question {
        id: 338,
        question: "西暦1838年は何世紀？",
        answer: &["19世紀"],
    },
    Question {
        id: 339,
        question: "西暦1839年は何世紀？",
        answer: &["19世紀"],
    },
    Question {
        id: 340,
        question: "西暦1840年は何世紀？",
        answer: &["19世紀"],
    },
    Question {
        id: 341,
        question: "西暦1841年は何世紀？",
        answer: &["19世紀"],
    },
    Question {
        id: 342,
        question: "西暦1842年は何世紀？",
        answer: &["19世紀"],
    },
    Question {
        id: 343,
        question: "西暦1843年は何世紀？",
        answer: &["19世紀"],
    },
    Question {
        id: 344,
        question: "西暦1844年は何世紀？",
        answer: &["19世紀"],
    },
    Question {
        id: 345,
        question: "西暦1845年は何世紀？",
        answer: &["19世紀"],
    },
    Question {
        id: 346,
        question: "西暦1846年は何世紀？",
        answer: &["19世紀"],
    },
    Question {
        id: 347,
        question: "西暦1847年は何世紀？",
        answer: &["19世紀"],
    },
    Question {
        id: 348,
        question: "西暦1848年は何世紀？",
        answer: &["19世紀"],
    },
    Question {
        id: 349,
        question: "西暦1849年は何世紀？",
        answer: &["19世紀"],
    },
    Question {
        id: 350,
        question: "西暦1850年は何世紀？",
        answer: &["19世紀"],
    },
    Question {
        id: 351,
        question: "西暦1851年は何世紀？",
        answer: &["19世紀"],
    },
    Question {
        id: 352,
        question: "西暦1852年は何世紀？",
        answer: &["19世紀"],
    },
    Question {
        id: 353,
        question: "西暦1853年は何世紀？",
        answer: &["19世紀"],
    },
    Question {
        id: 354,
        question: "西暦1854年は何世紀？",
        answer: &["19世紀"],
    },
    Question {
        id: 355,
        question: "西暦1855年は何世紀？",
        answer: &["19世紀"],
    },
    Question {
        id: 356,
        question: "西暦1856年は何世紀？",
        answer: &["19世紀"],
    },
    Question {
        id: 357,
        question: "西暦1857年は何世紀？",
        answer: &["19世紀"],
    },
    Question {
        id: 358,
        question: "西暦1858年は何世紀？",
        answer: &["19世紀"],
    },
    Question {
        id: 359,
        question: "西暦1859年は何世紀？",
        answer: &["19世紀"],
    },
    Question {
        id: 360,
        question: "西暦1860年は何世紀？",
        answer: &["19世紀"],
    },
    Question {
        id: 361,
        question: "西暦1861年は何世紀？",
        answer: &["19世紀"],
    },
    Question {
        id: 362,
        question: "西暦1862年は何世紀？",
        answer: &["19世紀"],
    },
    Question {
        id: 363,
        question: "西暦1863年は何世紀？",
        answer: &["19世紀"],
    },
    Question {
        id: 364,
        question: "西暦1864年は何世紀？",
        answer: &["19世紀"],
    },
    Question {
        id: 365,
        question: "西暦1865年は何世紀？",
        answer: &["19世紀"],
    },
    Question {
        id: 366,
        question: "西暦1866年は何世紀？",
        answer: &["19世紀"],
    },
    Question {
        id: 367,
        question: "西暦1867年は何世紀？",
        answer: &["19世紀"],
    },
    Question {
        id: 368,
        question: "西暦1868年は何世紀？",
        answer: &["19世紀"],
    },
    Question {
        id: 369,
        question: "西暦1869年は何世紀？",
        answer: &["19世紀"],
    },
    Question {
        id: 370,
        question: "西暦1870年は何世紀？",
        answer: &["19世紀"],
    },
    Question {
        id: 371,
        question: "西暦1871年は何世紀？",
        answer: &["19世紀"],
    },
    Question {
        id: 372,
        question: "西暦1872年は何世紀？",
        answer: &["19世紀"],
    },
    Question {
        id: 373,
        question: "西暦1873年は何世紀？",
        answer: &["19世紀"],
    },
    Question {
        id: 374,
        question: "西暦1874年は何世紀？",
        answer: &["19世紀"],
    },
    Question {
        id: 375,
        question: "西暦1875年は何世紀？",
        answer: &["19世紀"],
    },
    Question {
        id: 376,
        question: "西暦1876年は何世紀？",
        answer: &["19世紀"],
    },
    Question {
        id: 377,
        question: "西暦1877年は何世紀？",
        answer: &["19世紀"],
    },
    Question {
        id: 378,
        question: "西暦1878年は何世紀？",
        answer: &["19世紀"],
    },
    Question {
        id: 379,
        question: "西暦1879年は何世紀？",
        answer: &["19世紀"],
    },
    Question {
        id: 380,
        question: "西暦1880年は何世紀？",
        answer: &["19世紀"],
    },
    Question {
        id: 381,
        question: "西暦1881年は何世紀？",
        answer: &["19世紀"],
    },
    Question {
        id: 382,
        question: "西暦1882年は何世紀？",
        answer: &["19世紀"],
    },
    Question {
        id: 383,
        question: "西暦1883年は何世紀？",
        answer: &["19世紀"],
    },
    Question {
        id: 384,
        question: "西暦1884年は何世紀？",
        answer: &["19世紀"],
    },
    Question {
        id: 385,
        question: "西暦1885年は何世紀？",
        answer: &["19世紀"],
    },
    Question {
        id: 386,
        question: "西暦1886年は何世紀？",
        answer: &["19世紀"],
    },
    Question {
        id: 387,
        question: "西暦1887年は何世紀？",
        answer: &["19世紀"],
    },
    Question {
        id: 388,
        question: "西暦1888年は何世紀？",
        answer: &["19世紀"],
    },
    Question {
        id: 389,
        question: "西暦1889年は何世紀？",
        answer: &["19世紀"],
    },
    Question {
        id: 390,
        question: "西暦1890年は何世紀？",
        answer: &["19世紀"],
    },
    Question {
        id: 391,
        question: "西暦1891年は何世紀？",
        answer: &["19世紀"],
    },
    Question {
        id: 392,
        question: "西暦1892年は何世紀？",
        answer: &["19世紀"],
    },
    Question {
        id: 393,
        question: "西暦1893年は何世紀？",
        answer: &["19世紀"],
    },
    Question {
        id: 394,
        question: "西暦1894年は何世紀？",
        answer: &["19世紀"],
    },
    Question {
        id: 395,
        question: "西暦1895年は何世紀？",
        answer: &["19世紀"],
    },
    Question {
        id: 396,
        question: "西暦1896年は何世紀？",
        answer: &["19世紀"],
    },
    Question {
        id: 397,
        question: "西暦1897年は何世紀？",
        answer: &["19世紀"],
    },
    Question {
        id: 398,
        question: "西暦1898年は何世紀？",
        answer: &["19世紀"],
    },
    Question {
        id: 399,
        question: "西暦1899年は何世紀？",
        answer: &["19世紀"],
    },
    Question {
        id: 400,
        question: "西暦1900年は何世紀？",
        answer: &["19世紀"],
    },
    Question {
        id: 401,
        question: "西暦1901年は何世紀？",
        answer: &["20世紀"],
    },
    Question {
        id: 402,
        question: "西暦1902年は何世紀？",
        answer: &["20世紀"],
    },
    Question {
        id: 403,
        question: "西暦1903年は何世紀？",
        answer: &["20世紀"],
    },
    Question {
        id: 404,
        question: "西暦1904年は何世紀？",
        answer: &["20世紀"],
    },
    Question {
        id: 405,
        question: "西暦1905年は何世紀？",
        answer: &["20世紀"],
    },
    Question {
        id: 406,
        question: "西暦1906年は何世紀？",
        answer: &["20世紀"],
    },
    Question {
        id: 407,
        question: "西暦1907年は何世紀？",
        answer: &["20世紀"],
    },
    Question {
        id: 408,
        question: "西暦1908年は何世紀？",
        answer: &["20世紀"],
    },
    Question {
        id: 409,
        question: "西暦1909年は何世紀？",
        answer: &["20世紀"],
    },
    Question {
        id: 410,
        question: "西暦1910年は何世紀？",
        answer: &["20世紀"],
    },
    Question {
        id: 411,
        question: "西暦1911年は何世紀？",
        answer: &["20世紀"],
    },
    Question {
        id: 412,
        question: "西暦1912年は何世紀？",
        answer: &["20世紀"],
    },
    Question {
        id: 413,
        question: "西暦1913年は何世紀？",
        answer: &["20世紀"],
    },
    Question {
        id: 414,
        question: "西暦1914年は何世紀？",
        answer: &["20世紀"],
    },
    Question {
        id: 415,
        question: "西暦1915年は何世紀？",
        answer: &["20世紀"],
    },
    Question {
        id: 416,
        question: "西暦1916年は何世紀？",
        answer: &["20世紀"],
    },
    Question {
        id: 417,
        question: "西暦1917年は何世紀？",
        answer: &["20世紀"],
    },
    Question {
        id: 418,
        question: "西暦1918年は何世紀？",
        answer: &["20世紀"],
    },
    Question {
        id: 419,
        question: "西暦1919年は何世紀？",
        answer: &["20世紀"],
    },
    Question {
        id: 420,
        question: "西暦1920年は何世紀？",
        answer: &["20世紀"],
    },
    Question {
        id: 421,
        question: "西暦1921年は何世紀？",
        answer: &["20世紀"],
    },
    Question {
        id: 422,
        question: "西暦1922年は何世紀？",
        answer: &["20世紀"],
    },
    Question {
        id: 423,
        question: "西暦1923年は何世紀？",
        answer: &["20世紀"],
    },
    Question {
        id: 424,
        question: "西暦1924年は何世紀？",
        answer: &["20世紀"],
    },
    Question {
        id: 425,
        question: "西暦1925年は何世紀？",
        answer: &["20世紀"],
    },
    Question {
        id: 426,
        question: "西暦1926年は何世紀？",
        answer: &["20世紀"],
    },
    Question {
        id: 427,
        question: "西暦1927年は何世紀？",
        answer: &["20世紀"],
    },
    Question {
        id: 428,
        question: "西暦1928年は何世紀？",
        answer: &["20世紀"],
    },
    Question {
        id: 429,
        question: "西暦1929年は何世紀？",
        answer: &["20世紀"],
    },
    Question {
        id: 430,
        question: "西暦1930年は何世紀？",
        answer: &["20世紀"],
    },
    Question {
        id: 431,
        question: "西暦1931年は何世紀？",
        answer: &["20世紀"],
    },
    Question {
        id: 432,
        question: "西暦1932年は何世紀？",
        answer: &["20世紀"],
    },
    Question {
        id: 433,
        question: "西暦1933年は何世紀？",
        answer: &["20世紀"],
    },
    Question {
        id: 434,
        question: "西暦1934年は何世紀？",
        answer: &["20世紀"],
    },
    Question {
        id: 435,
        question: "西暦1935年は何世紀？",
        answer: &["20世紀"],
    },
    Question {
        id: 436,
        question: "西暦1936年は何世紀？",
        answer: &["20世紀"],
    },
    Question {
        id: 437,
        question: "西暦1937年は何世紀？",
        answer: &["20世紀"],
    },
    Question {
        id: 438,
        question: "西暦1938年は何世紀？",
        answer: &["20世紀"],
    },
    Question {
        id: 439,
        question: "西暦1939年は何世紀？",
        answer: &["20世紀"],
    },
    Question {
        id: 440,
        question: "西暦1940年は何世紀？",
        answer: &["20世紀"],
    },
    Question {
        id: 441,
        question: "西暦1941年は何世紀？",
        answer: &["20世紀"],
    },
    Question {
        id: 442,
        question: "西暦1942年は何世紀？",
        answer: &["20世紀"],
    },
    Question {
        id: 443,
        question: "西暦1943年は何世紀？",
        answer: &["20世紀"],
    },
    Question {
        id: 444,
        question: "西暦1944年は何世紀？",
        answer: &["20世紀"],
    },
    Question {
        id: 445,
        question: "西暦1945年は何世紀？",
        answer: &["20世紀"],
    },
    Question {
        id: 446,
        question: "西暦1946年は何世紀？",
        answer: &["20世紀"],
    },
    Question {
        id: 447,
        question: "西暦1947年は何世紀？",
        answer: &["20世紀"],
    },
    Question {
        id: 448,
        question: "西暦1948年は何世紀？",
        answer: &["20世紀"],
    },
    Question {
        id: 449,
        question: "西暦1949年は何世紀？",
        answer: &["20世紀"],
    },
    Question {
        id: 450,
        question: "西暦1950年は何世紀？",
        answer: &["20世紀"],
    },
    Question {
        id: 451,
        question: "西暦1951年は何世紀？",
        answer: &["20世紀"],
    },
    Question {
        id: 452,
        question: "西暦1952年は何世紀？",
        answer: &["20世紀"],
    },
    Question {
        id: 453,
        question: "西暦1953年は何世紀？",
        answer: &["20世紀"],
    },
    Question {
        id: 454,
        question: "西暦1954年は何世紀？",
        answer: &["20世紀"],
    },
    Question {
        id: 455,
        question: "西暦1955年は何世紀？",
        answer: &["20世紀"],
    },
    Question {
        id: 456,
        question: "西暦1956年は何世紀？",
        answer: &["20世紀"],
    },
    Question {
        id: 457,
        question: "西暦1957年は何世紀？",
        answer: &["20世紀"],
    },
    Question {
        id: 458,
        question: "西暦1958年は何世紀？",
        answer: &["20世紀"],
    },
    Question {
        id: 459,
        question: "西暦1959年は何世紀？",
        answer: &["20世紀"],
    },
    Question {
        id: 460,
        question: "西暦1960年は何世紀？",
        answer: &["20世紀"],
    },
    Question {
        id: 461,
        question: "西暦1961年は何世紀？",
        answer: &["20世紀"],
    },
    Question {
        id: 462,
        question: "西暦1962年は何世紀？",
        answer: &["20世紀"],
    },
    Question {
        id: 463,
        question: "西暦1963年は何世紀？",
        answer: &["20世紀"],
    },
    Question {
        id: 464,
        question: "西暦1964年は何世紀？",
        answer: &["20世紀"],
    },
    Question {
        id: 465,
        question: "西暦1965年は何世紀？",
        answer: &["20世紀"],
    },
    Question {
        id: 466,
        question: "西暦1966年は何世紀？",
        answer: &["20世紀"],
    },
    Question {
        id: 467,
        question: "西暦1967年は何世紀？",
        answer: &["20世紀"],
    },
    Question {
        id: 468,
        question: "西暦1968年は何世紀？",
        answer: &["20世紀"],
    },
    Question {
        id: 469,
        question: "西暦1969年は何世紀？",
        answer: &["20世紀"],
    },
    Question {
        id: 470,
        question: "西暦1970年は何世紀？",
        answer: &["20世紀"],
    },
    Question {
        id: 471,
        question: "西暦1971年は何世紀？",
        answer: &["20世紀"],
    },
    Question {
        id: 472,
        question: "西暦1972年は何世紀？",
        answer: &["20世紀"],
    },
    Question {
        id: 473,
        question: "西暦1973年は何世紀？",
        answer: &["20世紀"],
    },
    Question {
        id: 474,
        question: "西暦1974年は何世紀？",
        answer: &["20世紀"],
    },
    Question {
        id: 475,
        question: "西暦1975年は何世紀？",
        answer: &["20世紀"],
    },
    Question {
        id: 476,
        question: "西暦1976年は何世紀？",
        answer: &["20世紀"],
    },
    Question {
        id: 477,
        question: "西暦1977年は何世紀？",
        answer: &["20世紀"],
    },
    Question {
        id: 478,
        question: "西暦1978年は何世紀？",
        answer: &["20世紀"],
    },
    Question {
        id: 479,
        question: "西暦1979年は何世紀？",
        answer: &["20世紀"],
    },
    Question {
        id: 480,
        question: "西暦1980年は何世紀？",
        answer: &["20世紀"],
    },
    Question {
        id: 481,
        question: "西暦1981年は何世紀？",
        answer: &["20世紀"],
    },
    Question {
        id: 482,
        question: "西暦1982年は何世紀？",
        answer: &["20世紀"],
    },
    Question {
        id: 483,
        question: "西暦1983年は何世紀？",
        answer: &["20世紀"],
    },
    Question {
        id: 484,
        question: "西暦1984年は何世紀？",
        answer: &["20世紀"],
    },
    Question {
        id: 485,
        question: "西暦1985年は何世紀？",
        answer: &["20世紀"],
    },
    Question {
        id: 486,
        question: "西暦1986年は何世紀？",
        answer: &["20世紀"],
    },
    Question {
        id: 487,
        question: "西暦1987年は何世紀？",
        answer: &["20世紀"],
    },
    Question {
        id: 488,
        question: "西暦1988年は何世紀？",
        answer: &["20世紀"],
    },
    Question {
        id: 489,
        question: "西暦1989年は何世紀？",
        answer: &["20世紀"],
    },
    Question {
        id: 490,
        question: "西暦1990年は何世紀？",
        answer: &["20世紀"],
    },
    Question {
        id: 491,
        question: "西暦1991年は何世紀？",
        answer: &["20世紀"],
    },
    Question {
        id: 492,
        question: "西暦1992年は何世紀？",
        answer: &["20世紀"],
    },
    Question {
        id: 493,
        question: "西暦1993年は何世紀？",
        answer: &["20世紀"],
    },
    Question {
        id: 494,
        question: "西暦1994年は何世紀？",
        answer: &["20世紀"],
    },
    Question {
        id: 495,
        question: "西暦1995年は何世紀？",
        answer: &["20世紀"],
    },
    Question {
        id: 496,
        question: "西暦1996年は何世紀？",
        answer: &["20世紀"],
    },
    Question {
        id: 497,
        question: "西暦1997年は何世紀？",
        answer: &["20世紀"],
    },
    Question {
        id: 498,
        question: "西暦1998年は何世紀？",
        answer: &["20世紀"],
    },
    Question {
        id: 499,
        question: "西暦1999年は何世紀？",
        answer: &["20世紀"],
    },
    Question {
        id: 500,
        question: "西暦2000年は何世紀？",
        answer: &["20世紀"],
    },
];

const TOTAL_QUESTION_COUNT: usize = 500;
const ROUND_QUESTION_COUNT: usize = 100;
#[cfg(not(target_arch = "wasm32"))]
static RNG_STATE: AtomicU64 = AtomicU64::new(0x9E37_79B9_7F4A_7C15);

fn build_question_pool() -> Vec<Question> {
    debug_assert_eq!(BASE_QUESTIONS.len(), TOTAL_QUESTION_COUNT);
    BASE_QUESTIONS.to_vec()
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

#[cfg(not(target_arch = "wasm32"))]
fn random_index(upper_exclusive: usize) -> usize {
    let mut prev = RNG_STATE.load(Ordering::Relaxed);
    loop {
        // xorshift64*
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

#[cfg(target_arch = "wasm32")]
fn random_index(upper_exclusive: usize) -> usize {
    (js_sys::Math::random() * upper_exclusive as f64) as usize
}

fn sample_round_questions() -> Vec<Question> {
    let mut questions = build_question_pool();
    shuffle_in_place(&mut questions);
    questions.truncate(ROUND_QUESTION_COUNT.min(questions.len()));
    questions
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

fn init_answers(player_count: usize) -> Vec<String> {
    vec![String::new(); player_count]
}

fn is_all_answered(answers: &[String], player_count: usize) -> bool {
    answers.len() == player_count && answers.iter().all(|a| !a.trim().is_empty())
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

fn should_debug_force_clear(
    player_count: usize,
    selected_question_idx: Option<usize>,
    answers: &[String],
) -> bool {
    player_count == 2
        && selected_question_idx == Some(0)
        && answers.len() == 2
        && normalize_answer(&answers[0]) == "ろ"
        && normalize_answer(&answers[1]) == "で"
}

fn selected_question_by_idx(
    selected_question_idx: Option<usize>,
    questions: &[Question],
) -> Option<Question> {
    selected_question_idx.and_then(|idx| questions.get(idx).copied())
}

struct RoundStateUpdate {
    round_result: Option<bool>,
    success_streak: usize,
    current_player: usize,
    answered_questions: Vec<u32>,
    screen: Screen,
    should_reset_round_inputs: bool,
}

fn evaluate_confirm_round(
    player_count: usize,
    selected_question_idx: Option<usize>,
    questions: &[Question],
    answers: &[String],
    success_streak: usize,
    current_player: usize,
    answered_questions: &[u32],
) -> Option<RoundStateUpdate> {
    if should_debug_force_clear(player_count, selected_question_idx, answers) {
        return Some(RoundStateUpdate {
            round_result: Some(true),
            success_streak,
            current_player,
            answered_questions: answered_questions.to_vec(),
            screen: Screen::Clear,
            should_reset_round_inputs: false,
        });
    }

    let question = selected_question_by_idx(selected_question_idx, questions)?;
    let all_correct = answers
        .iter()
        .all(|answer| is_correct_answer(answer, question.answer));

    if all_correct {
        let next_success_streak = success_streak + 1;
        let mut next_answered_questions = answered_questions.to_vec();
        next_answered_questions.push(question.id);

        if next_success_streak >= 5 {
            return Some(RoundStateUpdate {
                round_result: Some(true),
                success_streak: next_success_streak,
                current_player,
                answered_questions: next_answered_questions,
                screen: Screen::Clear,
                should_reset_round_inputs: false,
            });
        }

        let next_player = next_player_index(current_player, player_count);
        Some(RoundStateUpdate {
            round_result: Some(true),
            success_streak: next_success_streak,
            current_player: next_player,
            answered_questions: next_answered_questions,
            screen: Screen::GameSelect,
            should_reset_round_inputs: true,
        })
    } else {
        Some(RoundStateUpdate {
            round_result: Some(false),
            success_streak: 0,
            current_player: 0,
            answered_questions: Vec::new(),
            screen: Screen::GameSelect,
            should_reset_round_inputs: true,
        })
    }
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
    let (round_questions, set_round_questions) = signal(sample_round_questions());
    let (answered_questions, set_answered_questions) = signal(Vec::<u32>::new());
    let (selected_question_idx, set_selected_question_idx) = signal(None::<usize>);
    let (answers, set_answers) = signal(Vec::<String>::new());
    let (input_player_idx, set_input_player_idx) = signal(None::<usize>);
    let (temp_answer, set_temp_answer) = signal(String::new());
    let (round_result, set_round_result) = signal(None::<bool>);

    let update_player_count = move |new_count: usize| {
        set_player_count.set(new_count);
        set_player_names.update(|names| {
            *names = resize_player_names(names, new_count);
        });
    };

    let start_game = move || {
        set_success_streak.set(0);
        set_current_player.set(0);
        set_round_questions.set(sample_round_questions());
        set_answered_questions.set(Vec::new());
        set_selected_question_idx.set(None);
        set_answers.set(Vec::new());
        set_input_player_idx.set(None);
        set_temp_answer.set(String::new());
        set_round_result.set(None);
        set_screen.set(Screen::GameSelect);
    };

    let selected_question = move || {
        let questions = round_questions.get();
        selected_question_by_idx(selected_question_idx.get(), &questions)
    };

    let all_answered = move || {
        let player_total = player_count.get();
        let current_answers = answers.get();
        is_all_answered(&current_answers, player_total)
    };

    // 「結果を確定」ボタンを押したときの処理
    let confirm_round = move || {
        let current_answers = answers.get();
        let update = match evaluate_confirm_round(
            player_count.get_untracked(),
            selected_question_idx.get_untracked(),
            &round_questions.get_untracked(),
            &current_answers,
            success_streak.get_untracked(),
            current_player.get_untracked(),
            &answered_questions.get_untracked(),
        ) {
            Some(update) => update,
            None => return,
        };

        set_round_result.set(update.round_result);
        set_success_streak.set(update.success_streak);
        set_current_player.set(update.current_player);
        set_answered_questions.set(update.answered_questions);
        set_screen.set(update.screen);

        if update.should_reset_round_inputs {
            set_selected_question_idx.set(None);
            set_answers.set(Vec::new());
            set_input_player_idx.set(None);
            set_temp_answer.set(String::new());
        }
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
                                            player_name_or_default(&names, current_player.get())
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
                                                        {round_questions
                                                            .get()
                                                            .into_iter()
                                                            .enumerate()
                                                            .map(|(idx, q)| {
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
                                                                                .set(Some(idx));
                                                                            set_answers
                                                                                .set(
                                                                                    init_answers(
                                                                                        player_count
                                                                                            .get_untracked(),
                                                                                    ),
                                                                                );
                                                                            set_round_result.set(None);
                                                                            set_screen.set(Screen::GameAnswerList);
                                                                        }
                                                                    >
                                                                        <span class="qid">{idx + 1}</span>
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
                                                                    <p>{format!(
                                                                        "問題 {}",
                                                                        selected_question_idx
                                                                            .get()
                                                                            .map(|idx| idx + 1)
                                                                            .unwrap_or(1),
                                                                    )}</p>
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
                                                                    .unwrap_or_else(|| default_player_name(0))
                                                            }}
                                                        </h3>
                                                        <p>"他の人は見ないでください"</p>
                                                    </div>

                                                    {move || {
                                                        if let Some(question) = selected_question() {
                                                            view! {
                                                                <div class="question-box">
                                                                    <p>{format!(
                                                                        "問題 {}",
                                                                        selected_question_idx
                                                                            .get()
                                                                            .map(|idx| idx + 1)
                                                                            .unwrap_or(1),
                                                                    )}</p>
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
                                                                    <p>{format!(
                                                                        "問題 {}",
                                                                        selected_question_idx
                                                                            .get()
                                                                            .map(|idx| idx + 1)
                                                                            .unwrap_or(1),
                                                                    )}</p>
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
                                                                                    player_name_or_default(
                                                                                        &player_names.get(),
                                                                                        idx,
                                                                                    )
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
                                <p class="clear-text">
                                    {move || {
                                        format!(
                                            "{}人全員で5問連続正解を達成しました",
                                            player_count.get()
                                        )
                                    }}
                                </p>

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
    mod question_pool_tests {
        use std::collections::HashSet;

        use super::super::{
            build_question_pool, sample_round_questions, ROUND_QUESTION_COUNT, TOTAL_QUESTION_COUNT,
        };

        #[test]
        fn build_question_pool_creates_500_questions() {
            let questions = build_question_pool();
            assert_eq!(questions.len(), TOTAL_QUESTION_COUNT);
        }

        #[test]
        fn sample_round_questions_picks_100_unique_questions() {
            let questions = sample_round_questions();
            let unique_ids = questions.iter().map(|q| q.id).collect::<HashSet<_>>();

            assert_eq!(questions.len(), ROUND_QUESTION_COUNT);
            assert_eq!(unique_ids.len(), ROUND_QUESTION_COUNT);
        }
    }

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

    mod should_debug_force_clear_tests {
        use super::super::should_debug_force_clear;

        #[test]
        fn returns_true_only_for_debug_pattern() {
            let answers = vec!["ろ".to_string(), "で".to_string()];
            assert!(should_debug_force_clear(2, Some(0), &answers));
        }

        #[test]
        fn returns_false_when_player_count_is_not_two() {
            let answers = vec!["ろ".to_string(), "で".to_string()];
            assert!(!should_debug_force_clear(3, Some(0), &answers));
        }

        #[test]
        fn returns_false_when_question_is_not_first() {
            let answers = vec!["ろ".to_string(), "で".to_string()];
            assert!(!should_debug_force_clear(2, Some(1), &answers));
        }

        #[test]
        fn returns_false_when_answers_do_not_match_debug_pattern() {
            let answers = vec!["ろ".to_string(), "ど".to_string()];
            assert!(!should_debug_force_clear(2, Some(0), &answers));
        }
    }

    mod selected_question_tests {
        use super::super::{selected_question_by_idx, BASE_QUESTIONS};

        #[test]
        fn returns_none_when_selection_is_none() {
            assert!(selected_question_by_idx(None, &BASE_QUESTIONS).is_none());
        }

        #[test]
        fn returns_question_when_index_is_in_range() {
            let question =
                selected_question_by_idx(Some(0), &BASE_QUESTIONS).expect("question should exist");
            assert_eq!(question.id, 1);
            assert_eq!(question.question, "太陽から一番近い惑星は？");
        }

        #[test]
        fn returns_none_when_index_is_out_of_range() {
            assert!(selected_question_by_idx(Some(usize::MAX), &BASE_QUESTIONS).is_none());
            assert!(
                selected_question_by_idx(Some(BASE_QUESTIONS.len()), &BASE_QUESTIONS).is_none()
            );
        }
    }

    mod confirm_round_logic_tests {
        use super::super::{evaluate_confirm_round, Screen, BASE_QUESTIONS};

        #[test]
        fn returns_none_when_no_question_selected() {
            let answers = vec!["水星".to_string(), "すいせい".to_string()];
            let update = evaluate_confirm_round(2, None, &BASE_QUESTIONS, &answers, 0, 0, &[]);
            assert!(update.is_none());
        }

        #[test]
        fn debug_force_clear_takes_priority() {
            let answers = vec!["ろ".to_string(), "で".to_string()];
            let update =
                evaluate_confirm_round(2, Some(0), &BASE_QUESTIONS, &answers, 4, 1, &[1, 2])
                    .expect("debug update should be returned");

            assert_eq!(update.round_result, Some(true));
            assert_eq!(update.success_streak, 4);
            assert_eq!(update.current_player, 1);
            assert_eq!(update.answered_questions, vec![1, 2]);
            assert_eq!(update.screen, Screen::Clear);
            assert!(!update.should_reset_round_inputs);
        }

        #[test]
        fn all_correct_moves_to_next_player_and_keeps_playing() {
            let answers = vec!["水星".to_string(), "スイセイ".to_string()];
            let update =
                evaluate_confirm_round(3, Some(0), &BASE_QUESTIONS, &answers, 2, 1, &[10, 11])
                    .expect("round update should be returned");

            assert_eq!(update.round_result, Some(true));
            assert_eq!(update.success_streak, 3);
            assert_eq!(update.current_player, 2);
            assert_eq!(update.answered_questions, vec![10, 11, 1]);
            assert_eq!(update.screen, Screen::GameSelect);
            assert!(update.should_reset_round_inputs);
        }

        #[test]
        fn all_correct_reaching_five_transitions_to_clear() {
            let answers = vec!["水星".to_string()];
            let update =
                evaluate_confirm_round(5, Some(0), &BASE_QUESTIONS, &answers, 4, 3, &[1, 2, 3, 4])
                    .expect("round update should be returned");

            assert_eq!(update.round_result, Some(true));
            assert_eq!(update.success_streak, 5);
            assert_eq!(update.current_player, 3);
            assert_eq!(update.answered_questions, vec![1, 2, 3, 4, 1]);
            assert_eq!(update.screen, Screen::Clear);
            assert!(!update.should_reset_round_inputs);
        }

        #[test]
        fn any_incorrect_answer_resets_progress() {
            let answers = vec!["水星".to_string(), "金星".to_string()];
            let update =
                evaluate_confirm_round(2, Some(0), &BASE_QUESTIONS, &answers, 3, 1, &[7, 8, 9])
                    .expect("round update should be returned");

            assert_eq!(update.round_result, Some(false));
            assert_eq!(update.success_streak, 0);
            assert_eq!(update.current_player, 0);
            assert!(update.answered_questions.is_empty());
            assert_eq!(update.screen, Screen::GameSelect);
            assert!(update.should_reset_round_inputs);
        }
    }

    mod player_name_tests {
        use super::super::{player_name_or_default, resize_player_names};

        #[test]
        fn resize_player_names_preserves_existing_entries() {
            let current = vec!["Alice".to_string(), "Bob".to_string(), "Carol".to_string()];
            let resized = resize_player_names(&current, 2);
            assert_eq!(resized, vec!["Alice".to_string(), "Bob".to_string()]);
        }

        #[test]
        fn resize_player_names_fills_new_slots_with_defaults() {
            let current = vec!["Alice".to_string(), "Bob".to_string()];
            let resized = resize_player_names(&current, 4);
            assert_eq!(
                resized,
                vec![
                    "Alice".to_string(),
                    "Bob".to_string(),
                    "プレイヤー3".to_string(),
                    "プレイヤー4".to_string()
                ]
            );
        }

        #[test]
        fn player_name_or_default_returns_custom_name_when_present() {
            let names = vec!["Alice".to_string(), "Bob".to_string()];
            assert_eq!(player_name_or_default(&names, 1), "Bob".to_string());
        }

        #[test]
        fn player_name_or_default_returns_default_when_missing() {
            let names = vec!["Alice".to_string()];
            assert_eq!(player_name_or_default(&names, 2), "プレイヤー3".to_string());
        }
    }

    mod player_count_logic_tests {
        use super::super::{init_answers, is_all_answered, next_player_index};

        #[test]
        fn init_answers_creates_slots_for_player_count() {
            let answers = init_answers(4);
            assert_eq!(answers.len(), 4);
            assert!(answers.iter().all(|a| a.is_empty()));
        }

        #[test]
        fn is_all_answered_returns_true_when_every_player_has_non_blank_answer() {
            let answers = vec!["東京".to_string(), "パリ".to_string(), "ローマ".to_string()];
            assert!(is_all_answered(&answers, 3));
        }

        #[test]
        fn is_all_answered_returns_false_when_player_count_does_not_match() {
            let answers = vec!["東京".to_string(), "パリ".to_string()];
            assert!(!is_all_answered(&answers, 3));
        }

        #[test]
        fn is_all_answered_returns_false_when_any_answer_is_blank() {
            let answers = vec!["東京".to_string(), "   ".to_string(), "ローマ".to_string()];
            assert!(!is_all_answered(&answers, 3));
        }

        #[test]
        fn next_player_index_rotates_within_player_count() {
            assert_eq!(next_player_index(0, 5), 1);
            assert_eq!(next_player_index(3, 5), 4);
            assert_eq!(next_player_index(4, 5), 0);
        }

        #[test]
        fn next_player_index_handles_zero_player_count_safely() {
            assert_eq!(next_player_index(0, 0), 0);
            assert_eq!(next_player_index(3, 0), 0);
        }
    }
}
