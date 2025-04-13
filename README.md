
# Riichi Mahjong Scoring Library and Calculator 
CLI tool that calculates the score of a hand in riichi mahjong. <br>
- Manual mode (Calculator Mode): given han and fu, calculates the score <br>
- Normal mode: given a hand, calculates the score with included yaku and fu <br>
- Analyse mode: Given tiles, will find valid tile shapes <br>

**This also doubles as a scoring library!** check out the <a href="https://docs.rs/mahc/latest/mahc/">docs on crates.io</a>

![demo gif](demo.gif)



## Examples
### Library Usage
```rust
use mahc::tile_group::TileGroup;
use mahc::tile::Tile;
use mahc::tile::Wind;
use mahc::hand::Hand;

// creating a tilegroup from a string
let one_two_three_seq: TileGroup = "123s".to_string().try_into().unwrap();
let seven_eight_nine_seq: TileGroup = "789m".to_string().try_into().unwrap

// creating a tilegroup from tiles
let seven_tile: Tile = "7m".to_string().try_into().unwrap();
let seven_pair: TileGroup =
    TileGroup::new(vec![seven_tile.clone(), seven_tile.clone()], false).unwrap();

// creating tiles in a few ways
let win_tile: Tile = "7m".to_string().try_into().unwrap();
let seat_wind: Tile = Tile::new(EAST_VALUE, &Suit::Wind).unwrap();
let prevelent_wind: Tile = Tile::Wind(WValue::West);

let hand = Hand::new(
    vec![
        one_two_three_seq.clone(),
        one_two_three_seq.clone(),
        seven_eight_nine_seq.clone(),
        seven_eight_nine_seq.clone(),
        seven_pair.clone(),
    ],
    win_tile,
    seat_wind,
    prevelent_wind,
)
.unwrap();
assert!(hand.is_ryanpeikou());

// get list of yaku, fu
let score = get_hand_score(
    &hand, &None, false, false, false, false, false, false, false, false, 0,
)
.unwrap();
assert_eq!(score.han(), 3);
assert_eq!(score.fu_score(), 40);

// get payment information
let payment = calculate(&score.han(), &score.fu_score()).unwrap();
assert_eq!(payment.base_points(), 1280);
assert_eq!(payment.dealer_ron(score.honba()), 7700);
```

### Calculator Mode
```bash
~/$ mahc -m 4 30 --ba 3
> Dealer:    12500 (4200) 
  non-dealer: 8600 (2300/ 4200)
```

### Normal Mode
note: the winning group has to go last (this is to calculate fu correctly)
``` bash
~/$ mahc --tiles 777z 111z 234p 234p 11p -w 1p -p Ew -s Ew
> 7 Han/ 50 Fu
  Dealer: 18000 (6000)
  Non-dealer: 12000 (3000/6000)
  Yaku:
    Iipeikou: 1
    Honitsu: 3
    Yakuhai: 1
    Yakuhai: 1
    Yakuhai: 1
  
  Fu:
    BasePoints: 20
    ClosedRon: 10
    NonSimpleClosedTriplet: 8
    NonSimpleClosedTriplet: 8
    SingleWait: 2
```

### Analyse Mode
This is a pretty new feature and needs some work
``` bash
~/$ mahc --tiles 1p 2p 3p 1p 2p 3p 1p 2p 3p rd rd rd rd Ew Ew -w Ew --analyse-tiles

> Handshapes found
  111p 222p 333p EEw rrrrd
  123p 123p 123p EEw rrrrd

~/$ mahc --tiles 1p 2p 3p 1p 2p 3p 1p 2p 3p rd rd rd rd Ew Ew -w Ew --analyse-tiles --json

> {"hands":["111p 222p 333p EEw rrrrd ","123p 123p 123p EEw rrrrd "]}
```

### Using file input
``` 
# hands.txt
--tiles 1p 9p 1s 9s 1m 9m rd gd wd Ew Sw Nw WWw -w Ww -p Ew -s Ew
--tiles 11z NNw SSw WWw rrd wwd ggd -w gd -p Ew -s Ew -d Ew Ew
-m 4 30 --ba 3
```


```bash
~/$ mahc -f hands.txt

❯ Dealer: 96000 (32000)
  Non-dealer: 64000 (16000/32000)
  Yaku:
    KokushiMusou Yakuman
    KokushiMusou Yakuman 13 sided wait
  
  Dealer: 144000 (48000)
  Non-dealer: 96000 (24000/48000)
  Yaku:
    Tsuuiisou Yakuman
    Daichiishin Yakuman
  
  4 Han/ 30 Fu/ 3 Honba
  Dealer: 12500 (4200)
  non-dealer: 8600 (2300/4200)
```
### Json out
in ***normal mode***
```bash
~/$ mahc --tiles 123p 456p 789p rrrdo 99p -w 9p -p Ew -s Ew -d 9p --json
```
yields
```json
{
    "dora":1,
    "fu":30,
    "fuString":[ "BasePoints: 20", "NonSimpleOpenTriplet: 4", "SingleWait: 2"
    ],
    "han":5,
    "honba":0,
    "scores":{
        "dealer":{"ron":12000,"tsumo":4000},
        "non-dealer":{
            "ron":8000, 
            "tsumo":{"dealer":4000,"non-dealer":2000}
        }
    },
    "yakuString":["Honitsu: 2","Ittsuu: 1","Yakuhai: 1"]
}
```
and in ***calculator mode***
```bash
~/$ mahc -m 4 30 --ba 3 --json
```
yields
```json
{
    "fu":30,
    "han":4,
    "honba":3,
    "scores":{
        "dealer":{ "ron":12500, "tsumo":4200 },
        "non-dealer":{ "ron":8600, "tsumo":{ "dealer":4200, "non-dealer":2300 }
        }
    }
}
```

## Notation 

### Suits

| Type  | Notation                                      |
|-------|-----------------------------------------------|
| Man (Characters) | 1m, 2m, 3m, 4m, 5m, 6m, 7m, 8m, 9m |
| Pin (Circles)    | 1p, 2p, 3p, 4p, 5p, 6p, 7p, 8p, 9p |
| Sou (Bamboos)    | 1s, 2s, 3s, 4s, 5s, 6s, 7s, 8s, 9s |

### Honors

| Type    | Notation       | MPSZ Notation      |
|---------|----------------|--------------------|
| Winds   | Ew, Sw, Ww, Nw |1z, 2z, 3z, 4z      |
| Dragons | wd, gd, rd     |5z, 6z, 7z          |

### Special Notation

| Description     | Example                                         |
|-----------------|-------------------------------------------------|
| Open Sets       | 234po (an open sequence of 2, 3, 4 in Pin suit) |
| akadora         | 0m, 0p, 0s                                      | 

- eg: EEEw (triplet of east wind)
- eg: 234m (sequence of 2 3 4 Man)
- eg: 406s (sequence of 4 5 6 Sou with the akadora 5 sou)
- eg: rrrrdo (open quad of red dragon)
- eg: 11s (pair of 1 sou)
- eg: 8m (8 man tile)

## Installation

#### *using <a href="https://doc.rust-lang.org/cargo/getting-started/installation.html"> cargo</a>*
```
cargo install mahc
mahc --version
```
#### *build from source*
```
git clone https://github.com/DrCheeseFace/mahc
cd mahc
cargo build
./target/debug/mahc --version
```
#### *from latest release*
```
curl -s https://api.github.com/repos/DrCheeseFace/rusty-riichi-mahjong-calculator/releases/latest | grep "browser_download_url" | cut -d '"' -f 4 | wget -i -
unzip mahc-vX.X.X-x86_64-unknown-linux-gnu.zip -d mahc
cd mahc/x86_64-unknown-linux-gnu/release
./mahc --version
```

## Contributing
<a href="https://github.com/drcheeseface/mahc">
    <img src="https://contrib.rocks/image?repo=drcheeseface/mahc" />
    <img src="https://avatars.githubusercontent.com/u/113280601?v=4&s=32" style="border-radius: 50%; height: 65px;" />
</a>

- ---- @gondoly @GuoDCZ

- If you spot a bug, put in an issue with how to reproduce it
- if you'd like to contribute, DO IT (send a PR)


![this.jpg](https://64.media.tumblr.com/07006d83e5810b3c651254e7b9a3e713/c4dc091a7806e504-ef/s400x600/cdfb08014450e71074a0a8763a67661485d59f8c.gif)
