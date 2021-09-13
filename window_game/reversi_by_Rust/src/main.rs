use std::io;
static dist: [[i32; 2]; 8] = [
    [-1, 0], [-1, -1], [0, -1], [1, -1],
    [1, 0], [1, 1], [0, 1], [-1, 1]
];

fn main() {
    // init
    let mut banmen :[[usize; 8]; 8] = [[3; 8]; 8];
    banmen[3][3] = 1;
    banmen[3][4] = 0;
    banmen[4][3] = 0;
    banmen[4][4] = 1;

    //打つ
    let mut skp : i32 = 0;
    let mut nw : i32 = 0;//0が先攻、1が後攻
    let mut sets : Vec<usize> = Vec::new();
    let mut sets2 : Vec<usize> = Vec::new();

    loop {
        sets = Vec::new();
        sets2 = Vec::new();
        let mut cn : bool = false;
        for i in 0..8 {
            for j in 0..8{
                if banmen[i][j] == 3{ print!("."); }else if banmen[i][j] == 1{ print!("W"); }else{ print!("B"); }
                if banmen[i][j] != 3 {continue;}
                for pos in dist.iter() {
                    let mut _x:i32 = i as i32 + pos[0];
                    let mut _y:i32 = j as i32 + pos[1];
                    if !(0 <= _x && _x < 8 && 0 <= _y && _y < 8) {continue;}
                    if(banmen[_x as usize][_y as usize] != ((nw+1)%2) as usize){continue;}
                    while 0 <= _x && _x < 8 && 0 <= _y && _y < 8 && banmen[_x as usize][_y as usize]==((nw+1)%2) as usize {
                        _x = _x + pos[0];
                        _y = _y + pos[1];
                    }
                    if 0 <= _x && _x < 8 && 0 <= _y && _y < 8 && banmen[_x as usize][_y as usize] == nw as usize {
                        cn = true;
                        sets.push(i);
                        sets2.push(j);
                    }
                }
            }
            print!("\n");
        }
        // for i in 0..sets.len(){
        //     println!("{:?} {:?}",sets[i],sets2[i]);
        // }
        if !cn {
            skp+=1;
            if(skp == 2){
                println!("end");
                break;
            }
            println!("skip");
        }else{
            skp = 0;
            let mut ok : bool =false;
            while !ok {
                println!("x:");
                let mut x = String::new();
                io::stdin().read_line(&mut x).expect("Failed to read line.");
                let x_pos: i32 = x.trim().parse().expect("Please type a number!");
                println!("y:");
                let mut y = String::new();
                io::stdin().read_line(&mut y).expect("Failed to read line.");
                let y_pos: i32 = y.trim().parse().expect("Please type a number!");
                
                for i in 0..sets.len(){
                    if sets[i] == x_pos as usize && sets2[i] == y_pos as usize {
                        ok = true;
                        break;
                    }
                }
                if ok {
                    for pos in dist.iter(){
                        let mut _x : i32 = x_pos + pos[0];
                        let mut _y : i32 = y_pos + pos[1];
    
                        let mut rev:bool = false;
                        while 0 <= _x && _x < 8 && 0 <= _y && _y < 8 && banmen[_x as usize][_y as usize]==((nw+1)%2) as usize {
                            _x = _x + pos[0];
                            _y = _y + pos[1];
                        }
                        if 0 <= _x && _x < 8 && 0 <= _y && _y < 8 && banmen[_x as usize][_y as usize] == nw as usize {
                            _x = _x - pos[0];
                            _y = _y - pos[1];
                            while banmen[_x as usize][_y as usize]==((nw+1)%2) as usize {
                                banmen[_x as usize][_y as usize] = nw as usize;
                                _x = _x - pos[0];
                                _y = _y - pos[1];
                            }
                        }
                        
                    }
                    banmen[x_pos as usize][y_pos as usize] = nw as usize;
                }else{
                    println!("retry...");
                }
            }
        }
        nw = (nw+1)%2;
        println!("next -> {:?}",nw)
    }
    //結果
    println!("end");
}