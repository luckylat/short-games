use rand::Rng;
use super::table;


static dist: [[i32; 2]; 8] = [
    [-1, 0], [-1, -1], [0, -1], [1, -1],
    [1, 0], [1, 1], [0, 1], [-1, 1]
];



pub fn life(sz:usize,stge:&Vec<Vec<bool>>) -> Vec<Vec<bool>>{
  let mut nextstge = table::init(sz);
  for x in 0..sz {
    for y in 0..sz{
      //4つ調べる
      let mut adj = 0;
      for pos in dist.iter() {
        let mut _x = x as i32 + pos[0];
        let mut _y = y as i32 + pos[1];
        if _x < 0 {_x = sz as i32 - 2;};
        if _y < 0 {_y = sz as i32 - 2;};
        if _x >= sz as i32 - 1 {_x = 0;};
        if _y >= sz as i32 - 1 {_y = 0;};
        if stge[_x as usize][_y as usize] {
          adj+=1;
        }
        
      }
      if stge[x][y] {
        if adj == 2 || adj == 3 {
          nextstge[x][y] = true;
        } else{
          nextstge[x][y] = false;
        }
      } else {
        if adj == 3 {
          nextstge[x][y] = true;
        }
      }
    }
  }
  nextstge
}