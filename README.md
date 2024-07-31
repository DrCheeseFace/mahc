
# Riichi Mahjong Calculator Via the Terminal
CLI tool that calculates the score of a hand in riichi mahjong. <br>
- Manual mode: given han and fu, calculates the score <br>
- Normal mode: given a hand, calculates the score with included yaku and fu

![demo gif](demo.gif)



## Examples

### Calculator Mode
```bash
~/$ mahc -m 4 30 --ba 3
> Dealer:    12500 (4200) 
  non-dealer: 8600 (2300/ 4200)
```

### Normal Mode
note: the winning group has to go last (this is to calculate fu correctly)
``` bash
~/$ mahc --tiles rrrd EEEw 234p 234p 11p -w 1p -p Ew -s Ew
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
### Using file input
``` 
# hands.txt
--tiles EEw NNw SSw WWw rrd wwd ggd -w gd -p Ew -s Ew -d 2
--tiles 123p 456p 789p rrrdo 99p -w 9p -p Ew -s Ew -d 2'
-m 4 30 --ba 3
```


```bash
~/$ mahc -f hands.txt
> Dealer: 144000 (48000)
  Non-dealer: 96000 (24000/48000)
  Yaku:
    Tsuuiisou Yakuman
    Daichiishin Yakuman
    Shousuushii Yakuman
  
  4 Han/ 30 Fu/ 3 Honba
  Dealer: 12500 (4200)
  non-dealer: 8600 (2300/4200)
  
  6 Han/ 30 Fu
  Dealer: 18000 (6000)
  Non-dealer: 12000 (3000/6000)
  Dora: 2
  Yaku:
    Honitsu: 2
    Ittsuu: 1
    Yakuhai: 1
  Fu:
    BasePoints: 20
    NonSimpleOpenTriplet: 4
    SingleWait: 2
```

## Notation 

### Suits

| Suit  | Tiles                                      |
|-------|--------------------------------------------|
| Man (Characters) | 1m, 2m, 3m, 4m, 5m, 6m, 7m, 8m, 9m |
| Pin (Circles)    | 1p, 2p, 3p, 4p, 5p, 6p, 7p, 8p, 9p |
| Sou (Bamboos)    | 1s, 2s, 3s, 4s, 5s, 6s, 7s, 8s, 9s |

### Honors

| Type  | Notation          |
|-------|-------------------|
| Winds | Ew, Sw, Ww, Nw    |
| Dragons | rd, gd, wd      |

### Special Notation

| Description     | Example           |
|-----------------|-------------------|
| Open Sets       | 234po (an open sequence of 2, 3, 4 in Pin suit) |

- eg: EEEw (triplet of east wind)
- eg: 234m (sequence of 2 3 4 Man)
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
git clone https://github.com/DrCheeseFace/rusty-riichi-mahjong-calculator
cd rusty-riichi-mahjong-calculator
cargo build
./target/debug/mahc --version
```
#### *from latest release*
```
curl -s https://api.github.com/repos/DrCheeseFace/rusty-riichi-mahjong-calculator/releases/latest | grep "browser_download_url" | cut -d '"' -f 4 | wget -i -
unzip mahc-v1.1.0-x86_64-unknown-linux-gnu.zip -d mahc
cd mahc/x86_64-unknown-linux-gnu/release
./mahc --version
```

### Implemented hand validations as of yet

##### One Han Yaku
- [x] Tanyao
- [x] Iipeikou 
- [x] Yakuhai 
- [x] MenzenTsumo
- [x] Pinfu
- [x] Riichi
- [x] Ippatsu
- [x] Haitei
- [x] RinshanKaihou
- [x] Chankan

##### Two Han Yaku
- [x] DoubleRiichi
- [x] Toitoi
- [x] Ittsuu
- [x] SanshokuDoujun
- [x] Chantaiyao
- [x] Sanankou
- [x] SanshokuDoukou
- [x] Sankantsu
- [x] Honroutou
- [x] Shousangen
- [x] Chiitoitsu

##### Three Han Yaku
- [x] Honitsu
- [x] JunchanTaiyao
- [x] Ryanpeikou 

##### Six Han Yaku
- [x] Chinitsu

##### Yakuman 
- [x] KazoeYakuman
- [x] KokushiMusou
- [x] KokushiMusou 13 sided wait
- [x] Suuankou
- [x] Suuankou tanki wait
- [x] Daisangen
- [x] Shousuushii
- [x] Daisuushii
- [x] Tsuuiisou
- [x] Daiichishin 
- [x] Chinroutou
- [x] Ryuuiisou
- [x] ChuurenPoutou
- [x] ChuurenPoutou 9 sided wait
- [x] Suukantsu
- [x] Tenhou 
- [x] Chiihou 

## TODO
- [x] validation a hand is possible (eg not having 20 east tiles :) 
- [x] add all da yaku DONE
- [x] validation on if yaku is there DONE
- [x] validate winning tile DONE DONE
- [x] propogate the errors up for a nice printout DONE
- [x] validate stuff like cant riichi and double riichi. all that haitei, chankan rinshan shizz DONE
- [x] file stdIn DONE?

## Contributing
- If you spot a bug (which there probabably are many), put in an issue with how to reproduce it. 
- keep in mind, we pretty far from finishing it currently. so FUCK validation (for the time being) 


![this.jpg](https://64.media.tumblr.com/07006d83e5810b3c651254e7b9a3e713/c4dc091a7806e504-ef/s400x600/cdfb08014450e71074a0a8763a67661485d59f8c.gif)
