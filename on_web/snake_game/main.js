const ready = (blockSize, boardSize) => {
  //宣言
  const background_layer = document.querySelector('#background-layer');
  const ctx_bg = background_layer.getContext('2d');
  const game_layer = document.querySelector('#game-layer');
  const ctx_game = game_layer.getContext('2d');
  
  //背景部分の準備
  ctx_bg.fillStyle = 'black';
  for(let i = 0; boardSize > i; i++){
    for(let j = 0; boardSize > j; j++){
      ctx_bg.strokeRect(i*blockSize,j*blockSize,blockSize,blockSize);
    }
  }
}

const game = (blockSize, boardSize) => {
  const randomint = (max) => {
    return Math.floor(Math.random()*max);
  }

  const wall_touched = (boardSize, now_location) => {//枠内にあるか
    return now_location.x < 0 || now_location.y < 0 || boardSize <= now_location.x || boardSize <= now_location.y
  }

  const self_touched = (now_location, queue) => {//queue内にnow_locationがないか
    console.log(now_location);
    console.log(queue);
    let ret = false;
    for(let i = 1; queue.length > i; i++){
      if(now_location.x === queue[i].x && now_location.y === queue[i].y){
        ret = true;
      }
    }
    return ret;
  }

  const point_touched = (now_location, point_location) => {
    return now_location.x === point_location.x && now_location.y === point_location.y;
  }
  const game_layer = document.querySelector('#game-layer');
  const ctx_game = game_layer.getContext('2d');
  let score = 0;
  let now_location = {x: boardSize/2, y: boardSize/2};
  let point_location = {x: randomint(boardSize), y: randomint(boardSize)};
  while(now_location === point_location){
    point_location = {x: randomint(boardSize), y: randomint(boardSize)};
  }
  const queue = [now_location];
  //0:→
  //1:↓
  //2:←
  //3:↑
  let arrow = 0;
  let arrow_prev = 0;
  
  //keys
  window.addEventListener('keydown', (e) => {
    switch(e.key){
      case 'ArrowRight':
        if(arrow_prev !== 2){
          arrow = 0;
        }
        break;
      case 'ArrowDown':
        if(arrow_prev !== 3){
          arrow = 1;
        }
        break;
      case 'ArrowLeft':
        if(arrow_prev !== 0){
          arrow = 2;
        }
        break;
      case 'ArrowUp':
        if(arrow_prev !== 1){
          arrow = 3;
        }
        break;
      default:
        break;
    }
  })
  const core = () => {
    const table = setInterval(() => {
      switch(arrow){
        case 0:
          now_location.x = now_location.x+1;
          break;
        case 1:
          now_location.y = now_location.y+1;
          break;
        case 2:
          now_location.x = now_location.x-1;
          break;
        case 3:
          now_location.y = now_location.y-1
      }
      arrow_prev = arrow;
  

      console.log(now_location);
      console.log(queue);
      if(wall_touched(boardSize, now_location) || self_touched(now_location, queue)){
        clearInterval(table);
        result(score);
      }
  
      //score
      if(point_touched(now_location, point_location)){
        score++;
        queue.push(JSON.parse(JSON.stringify(point_location)))
        ctx_game.fillStyle = 'black';
        ctx_game.fillRect(now_location.x*blockSize,now_location.y*blockSize,blockSize,blockSize);
        // console.log("+++++++++++++")
        // console.log(touched(boardSize, point_location,queue))
        // console.log("--------------")
        while(point_touched(now_location, point_location)){
          const new_x = randomint(boardSize);
          const new_y = randomint(boardSize);
          console.log([new_x, new_y]);
          point_location = {x: new_x, y: new_y};
        }
        //console.log("***************")
        
  
        //graphic
        ctx_game.fillStyle = 'green';
        ctx_game.fillRect(point_location.x*blockSize,point_location.y*blockSize,blockSize,blockSize);
  
  
      }else{
        queue.push(JSON.parse(JSON.stringify(now_location)));
        ctx_game.fillStyle = 'black';
        ctx_game.fillRect(now_location.x*blockSize,now_location.y*blockSize,blockSize,blockSize);
        //console.log(JSON.parse(JSON.stringify(now_location)))
        //console.log(JSON.parse(JSON.stringify(point_location)))
        //console.log(now_location === point_location)
        if(queue.length > score+2){
          const delete_location = queue.shift();
          ctx_game.clearRect(delete_location.x*blockSize,delete_location.y*blockSize,blockSize,blockSize);
        }
      }


      
    }, 500);
  }
  
  ctx_game.fillStyle = 'green';
  ctx_game.fillRect(point_location.x*blockSize,point_location.y*blockSize,blockSize,blockSize);
  core();
  //coreが終了した時にresultを用意する
}

const result = (score) => {
  window.alert(score);
}

window.addEventListener('load', (event) => {
  //element
  const startButton = document.querySelector('#start');
  //変数
  const blockSize = 30;
  const boardSize = 20;


  ready(blockSize, boardSize);

  
  startButton.addEventListener('click', (e) => {
    game(blockSize, boardSize);
  });
  
})