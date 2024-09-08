use std::ffi::OsString;
use std::fs::{self, OpenOptions};
use std::io::Write;

use clap::Parser;
use mahc::calc;
use mahc::hand::error::HandErr;
use mahc::hand::Hand;
use mahc::payment::Payment;
use mahc::score::{FuValue, HanValue, HonbaCounter, Score};
use mahc::tile_group::TileGroup;
use serde_json::json;

/// riichi mahjong calculator tool
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Hand tiles
    #[clap(long, value_delimiter = ' ', num_args = 1..)]
    tiles: Option<Vec<String>>,

    /// Winning tile
    #[arg(short, long)]
    win: Option<String>,

    /// Dora indicator tiles
    #[arg(short, long, value_delimiter = ' ', num_args = 1..)]
    dora: Option<Vec<String>>,

    /// seat wind
    #[arg(short, long, default_value = "Ew")]
    seat: String,

    /// prevelant wind
    #[arg(short, long, default_value = "Ew")]
    prev: String,

    /// is tsumo
    #[arg(short, long, default_value_t = false)]
    tsumo: bool,

    /// is riichi
    #[arg(short, long, default_value_t = false)]
    riichi: bool,

    /// is double riichi
    #[arg(long, default_value_t = false)]
    doubleriichi: bool,

    /// is ippatsu
    #[arg(short, long, default_value_t = false)]
    ippatsu: bool,

    /// is haitei
    #[arg(long, default_value_t = false)]
    haitei: bool,

    /// is rinshan
    #[arg(long, default_value_t = false)]
    rinshan: bool,

    /// is chankan
    #[arg(long, default_value_t = false)]
    chankan: bool,

    /// is tenhou/chihou
    #[arg(long, default_value_t = false)]
    tenhou: bool,

    /// honba count
    #[arg(short, long, default_value_t = 0)]
    ba: HonbaCounter,

    /// calculator mode
    #[arg(short, long, default_value = None, value_delimiter = ' ', num_args = 2)]
    manual: Option<Vec<u32>>,

    /// file input
    #[arg(short, long, default_value = None)]
    file: Option<String>,

    /// stdout as json
    #[arg(long, default_value_t = false)]
    json: bool,

    /// file output
    #[arg(short, default_value = "mahc.txt")]
    output: Option<String>,
}

pub fn parse_calculator(args: &Args) -> Result<String, HandErr> {
    let honba = args.ba;
    let han = args.manual.as_ref().unwrap()[0];
    let fu = args.manual.as_ref().unwrap()[1].into();
    let payment = calc::calculate(han, fu)?;

    if args.json {
        Ok(json_calc_out(&payment, honba, han, fu))
    } else {
        Ok(default_calc_out(&payment, honba, han, fu))
    }
}

pub fn parse_hand(args: &Args) -> Result<String, HandErr> {
    if args.tiles.is_none() {
        return Err(HandErr::NoHandTiles);
    }
    if args.win.is_none() {
        return Err(HandErr::NoWinTile);
    }
    if args.tsumo && args.chankan {
        return Err(HandErr::ChankanTsumo);
    }
    if args.rinshan && (!args.tsumo) {
        return Err(HandErr::RinshanWithoutTsumo);
    }
    if args.rinshan && args.ippatsu {
        return Err(HandErr::RinshanIppatsu);
    }
    if args.riichi && args.doubleriichi {
        return Err(HandErr::DuplicateRiichi);
    }
    if args.ippatsu && !(args.riichi || args.doubleriichi) {
        return Err(HandErr::IppatsuWithoutRiichi);
    }
    if args.doubleriichi && args.ippatsu && args.haitei {
        return Err(HandErr::DoubleRiichiHaiteiIppatsu);
    }
    if args.doubleriichi && args.haitei && args.chankan {
        return Err(HandErr::DoubleRiichiHaiteiChankan);
    }
    let hand = Hand::new_from_strings(
        args.tiles.clone().unwrap(),
        args.win.clone().unwrap(),
        args.prev.clone(),
        args.seat.clone(),
    )?;
    let doras: Option<Vec<TileGroup>> = args.dora.clone().map(|dora_tiles| {
        dora_tiles
            .into_iter()
            .filter_map(|tile| tile.try_into().ok())
            .collect()
    });
    let score = calc::get_hand_score(
        hand,
        doras,
        args.tsumo,
        args.riichi,
        args.doubleriichi,
        args.ippatsu,
        args.haitei,
        args.rinshan,
        args.chankan,
        args.tenhou,
        args.ba,
    )?;

    //TODO VALIDATION (i dont care enough yet)

    let printout = if args.json {
        json_hand_out(&score)
    } else {
        default_hand_out(&score)
    };
    Ok(printout)
}

pub fn json_calc_out(payment: &Payment, honba: HonbaCounter, han: HanValue, fu: FuValue) -> String {
    let out = json!({
    "han" : han,
    "fu" : fu,
    "honba" : honba,
        "scores" : {
            "dealer" : {
                "ron" : payment.dealer_ron(honba),
                "tsumo" : payment.dealer_tsumo(honba)
            },
            "non-dealer" : {
                "ron" : payment.non_dealer_ron(honba),
                "tsumo" : {
                    "dealer" : payment.non_dealer_tsumo_to_dealer(honba),
                    "non-dealer" : payment.non_dealer_tsumo_to_non_dealer(honba)
                }
            }
        }
    });
    out.to_string()
}

pub fn default_calc_out(
    payment: &Payment,
    honba: HonbaCounter,
    han: HanValue,
    fu: FuValue,
) -> String {
    let honba_str = if honba != 0 {
        format!("/ {honba} Honba")
    } else {
        "".to_string()
    };

    format!(
        "\n{han} Han\
        / {fu} Fu\
        {honba}\
        \nDealer: {dealer_ron} ({dealer_each})\
        \nnon-dealer: {non_dealer_ron} ({non_dealer_payment}/{dealer_payment})",
        han = han,
        fu = fu,
        honba = honba_str,
        dealer_ron = payment.dealer_ron(honba),
        dealer_each = payment.dealer_tsumo(honba),
        non_dealer_ron = payment.non_dealer_ron(honba),
        non_dealer_payment = payment.non_dealer_tsumo_to_non_dealer(honba),
        dealer_payment = payment.non_dealer_tsumo_to_dealer(honba)
    )
}

pub fn json_hand_out(score: &Score) -> String {
    let out = json!({
        "han" : score.han(),
        "fu" : score.fu_score(),
        "honba" : score.honba(),
        "dora" : score.dora_count(),
        "fuString" : score.fu().iter().map(|x| x.to_string()).collect::<Vec<String>>(),
        "yakuString" : score.yaku().iter().map(|x| x.to_string(score.is_open())).collect::<Vec<String>>(),
        "scores" : {
            "dealer" : {
                "ron" : score.payment().dealer_ron(score.honba()),
                "tsumo" : score.payment().dealer_tsumo(score.honba())
            },
            "non-dealer" : {
                "ron" : score.payment().non_dealer_ron(score.honba()),
                "tsumo" : {
                "dealer" : score.payment().non_dealer_tsumo_to_dealer(score.honba()),
                "non-dealer" : score.payment().non_dealer_tsumo_to_non_dealer(score.honba())
                }
            }
        }
    });
    out.to_string()
}
pub fn default_hand_out(score: &Score) -> String {
    let mut out: String = String::new();
    if !score.yaku()[0].is_yakuman() {
        if score.honba() != 0 {
            out.push_str(
                format!(
                    "\n{} Han/ {} Fu/ {} Honba",
                    score.han(),
                    score.fu_score(),
                    score.honba()
                )
                .as_str(),
            )
        } else {
            out.push_str(&format!("\n{} Han/ {} Fu", score.han(), score.fu_score()))
        }
    }

    out.push_str(
        format!(
            "\nDealer: {} ({})\nNon-dealer: {} ({}/{})",
            score.payment().dealer_ron(score.honba()),
            score.payment().dealer_tsumo(score.honba()),
            score.payment().non_dealer_ron(score.honba()),
            score
                .payment()
                .non_dealer_tsumo_to_non_dealer(score.honba()),
            score.payment().non_dealer_tsumo_to_dealer(score.honba())
        )
        .as_str(),
    );

    if !score.yaku()[0].is_yakuman() && score.dora_count() != 0 {
        out.push_str(format!("\nDora: {}", score.dora_count()).as_str());
    }

    out.push_str("\nYaku: ");
    for yaku in score.yaku() {
        out.push_str(format!("\n  {}", yaku.to_string(score.is_open())).as_str());
    }

    if !score.yaku()[0].is_yakuman() {
        out.push_str("\nFu: ");
        for fu in score.fu() {
            out.push_str(format!("\n  {}", fu).as_str());
        }
    }

    out
}

pub fn parse_file(args: &Args) {
    let file_contents = match fs::read_to_string(args.file.as_ref().unwrap()) {
        Ok(contents) => contents,
        Err(_) => {
            eprintln!("Error: Unable to read file {}", args.file.as_ref().unwrap());
            return;
        }
    };
    let results: Vec<Result<String, HandErr>> = file_contents
        .lines()
        .filter(|x| !x.is_empty())
        .map(|x| {
            let mut current_line_args = vec![OsString::from("mahc")];
            for arg in x.split_whitespace() {
                current_line_args.push(arg.into());
            }
            let args = Args::parse_from(&current_line_args);
            if let Some(_) = args.file {
                parse_file(&args);
                Ok("".to_string())
            } else if args.manual.is_some() {
                parse_calculator(&args)
            } else {
                parse_hand(&args)
            }
        })
        .collect();

    for result in results {
        writeout(&result, args.output.as_ref().unwrap());
        printout(&result)
    }
}

pub fn printout(result: &Result<String, HandErr>) {
    match result {
        Ok(o) => {
            println!("{}", o);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}

pub fn writeout(result: &Result<String, HandErr>, output: &str) {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(output)
        .expect("unable to open file");

    let mut content = match result {
        Ok(o) => o.clone(),
        Err(e) => e.to_string(),
    };
    content.push_str("\n");

    file.write_all(content.as_bytes())
        .expect("unable to write to file");
}

fn main() {
    let args = Args::parse();

    let result = if let Some(_) = args.file {
        parse_file(&args);
        return;
    } else if args.manual.is_some() {
        parse_calculator(&args)
    } else {
        parse_hand(&args)
    };

    if let Some(output) = &args.output {
        writeout(&result, output);
    }

    printout(&result);
}

#[cfg(test)]
mod test {
    use mahc::hand::error::HandErr;

    use super::*;

    #[test]
    fn no_han_for_calc() {
        let args = Args::parse_from(["", "--manual", "0", "30", "--ba", "3"]);
        let out = parse_calculator(&args);
        assert_eq!(out.unwrap_err(), HandErr::NoHan);
    }

    #[test]
    fn no_fu_for_calc() {
        let args = Args::parse_from(["", "--manual", "4", "0", "--ba", "3"]);
        let out = parse_calculator(&args);
        assert_eq!(out.unwrap_err(), HandErr::NoFu);
    }

    #[test]
    fn valid_calc_input() {
        let args = Args::parse_from(["", "--manual", "4", "30", "--ba", "3"]);
        let out = parse_calculator(&args);
        assert_eq!(
            out.unwrap(),
            ("\n4 Han/ 30 Fu/ 3 Honba\nDealer: 12500 (4200)\nnon-dealer: 8600 (2300/4200)"
                .to_string())
        );
    }
    #[test]
    fn han_1_fu_30_calc() {
        let args = Args::parse_from(["", "--manual", "1", "30"]);
        let out = parse_calculator(&args);
        assert_eq!(
            out.unwrap(),
            ("\n1 Han/ 30 Fu\nDealer: 1500 (500)\nnon-dealer: 1000 (300/500)".to_string())
        );
    }
    #[test]
    fn han_2_fu_80_calc() {
        let args = Args::parse_from(["", "--manual", "2", "80"]);
        let out = parse_calculator(&args);
        assert_eq!(
            out.unwrap(),
            ("\n2 Han/ 80 Fu\nDealer: 7700 (2600)\nnon-dealer: 5200 (1300/2600)".to_string())
        );
    }
    #[test]
    fn han_3_mangan_calc() {
        let args = Args::parse_from(["", "--manual", "3", "70", "--ba", "3"]);
        let out = parse_calculator(&args);
        assert_eq!(
            out.unwrap(),
            ("\n3 Han/ 70 Fu/ 3 Honba\nDealer: 12900 (4300)\nnon-dealer: 8900 (2300/4300)"
                .to_string())
        );
    }
    #[test]
    fn han_4_mangan_calc() {
        let args = Args::parse_from(["", "--manual", "4", "60", "--ba", "3"]);
        let out = parse_calculator(&args);
        assert_eq!(
            out.unwrap(),
            ("\n4 Han/ 60 Fu/ 3 Honba\nDealer: 12900 (4300)\nnon-dealer: 8900 (2300/4300)"
                .to_string())
        );
    }
    #[test]
    fn han_5_mangan_calc() {
        let args = Args::parse_from(["", "--manual", "5", "70", "--ba", "3"]);
        let out = parse_calculator(&args);
        assert_eq!(
            out.unwrap(),
            ("\n5 Han/ 70 Fu/ 3 Honba\nDealer: 12900 (4300)\nnon-dealer: 8900 (2300/4300)"
                .to_string())
        );
    }
    #[test]
    fn haneman_calc() {
        let args = Args::parse_from(["", "--manual", "6", "70", "--ba", "3"]);
        let out = parse_calculator(&args);
        assert_eq!(
            out.unwrap(),
            ("\n6 Han/ 70 Fu/ 3 Honba\nDealer: 18900 (6300)\nnon-dealer: 12900 (3300/6300)"
                .to_string())
        );
        let args = Args::parse_from(["", "--manual", "7", "70", "--ba", "3"]);
        let out = parse_calculator(&args);
        assert_eq!(
            out.unwrap(),
            ("\n7 Han/ 70 Fu/ 3 Honba\nDealer: 18900 (6300)\nnon-dealer: 12900 (3300/6300)"
                .to_string())
        );
    }
    #[test]
    fn baiman_calc() {
        let args = Args::parse_from(["", "--manual", "8", "70", "--ba", "3"]);
        let out = parse_calculator(&args);
        assert_eq!(
            out.unwrap(),
            ("\n8 Han/ 70 Fu/ 3 Honba\nDealer: 24900 (8300)\nnon-dealer: 16900 (4300/8300)"
                .to_string())
        );
        let args = Args::parse_from(["", "--manual", "9", "70", "--ba", "3"]);
        let out = parse_calculator(&args);
        assert_eq!(
            out.unwrap(),
            ("\n9 Han/ 70 Fu/ 3 Honba\nDealer: 24900 (8300)\nnon-dealer: 16900 (4300/8300)"
                .to_string())
        );
        let args = Args::parse_from(["", "--manual", "10", "70", "--ba", "3"]);
        let out = parse_calculator(&args);
        assert_eq!(
            out.unwrap(),
            ("\n10 Han/ 70 Fu/ 3 Honba\nDealer: 24900 (8300)\nnon-dealer: 16900 (4300/8300)"
                .to_string())
        );
    }
    #[test]
    fn sanbaiman_calc() {
        let args = Args::parse_from(["", "--manual", "11", "70", "--ba", "3"]);
        let out = parse_calculator(&args);
        assert_eq!(
            out.unwrap(),
            ("\n11 Han/ 70 Fu/ 3 Honba\nDealer: 36900 (12300)\nnon-dealer: 24900 (6300/12300)"
                .to_string())
        );
        let args = Args::parse_from(["", "--manual", "12", "70", "--ba", "3"]);
        let out = parse_calculator(&args);
        assert_eq!(
            out.unwrap(),
            ("\n12 Han/ 70 Fu/ 3 Honba\nDealer: 36900 (12300)\nnon-dealer: 24900 (6300/12300)"
                .to_string())
        );
    }
    #[test]
    fn kazoeyakuman_calc() {
        let args = Args::parse_from(["", "--manual", "13", "70", "--ba", "3"]);
        let out = parse_calculator(&args);
        assert_eq!(
            out.unwrap(),
            ("\n13 Han/ 70 Fu/ 3 Honba\nDealer: 48900 (16300)\nnon-dealer: 32900 (8300/16300)"
                .to_string())
        );
    }
}
