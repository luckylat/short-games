window.addEventListener('load', (event) => {
  const canvas = document.querySelector('#canvas');
  const ctx = canvas.getContext('2d');

  ctx.fillStyle = 'black';
  ctx.fillRect(10,10,100,100);
})