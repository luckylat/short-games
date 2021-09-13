use std::{thread, time};
use std::time::Duration;

mod life;
mod table;

fn main() {
    let sz : usize = 30;//盤面サイズ
    //盤面の初期化
    let mut stge = table::init(sz);
    stge[0][1] = true;
    stge[1][3] = true;
    stge[2][0] = true;
    stge[2][1] = true;
    stge[2][4] = true;
    stge[2][5] = true;
    stge[2][6] = true;
    
    //盤面をクリックでLifeの追加
    //再生停止ボタン
    loop {
        print!("\x1B[2J");
        for x in &stge {
            let mut text = String::new();
            for y in x{
                text = format!("{}{}", text, if y == &true {"#"} else {"."});
            }
            println!("{}",text);
        }
        let nxt = life::life(sz,&stge);
        stge = nxt;
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 5));
    }
}
