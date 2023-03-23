import * as sim from 'lib-simulation-wasm';

const simulation = new sim.Simulation();
const viewport = document.getElementById('viewport');
const scale = window.devicePixelRatio || 1;
const width = viewport.width * scale;
const height = viewport.height * scale;

viewport.style.width = width + 'px';
viewport.style.height = height + 'px';

const context = viewport.getContext('2d');
context.scale(scale, scale);

CanvasRenderingContext2D.prototype.drawTriangle = function(x, y, size, rot) {
  this.beginPath();
  this.moveTo(x - Math.sin(rot) * size * 1.5, 
              y + Math.cos(rot) * size * 1.5);
  this.lineTo(x - Math.sin(rot + 2.0 / 3.0 * Math.PI) * size, 
              y + Math.cos(rot + 2.0 / 3.0 * Math.PI) * size);
  this.lineTo(x - Math.sin(rot + 4.0 / 3.0 * Math.PI) * size, 
              y + Math.cos(rot + 4.0 / 3.0 * Math.PI) * size);
  this.lineTo(x - Math.sin(rot) * size * 1.5, 
              y + Math.cos(rot) * size * 1.5);
  this.fillStyle = '#0E8388';
  this.strokeStyle = '#DDDDDD'
  this.stroke();
  this.fill();
};

CanvasRenderingContext2D.prototype.drawCircle = function(x, y, radius) {
  this.beginPath();
  this.arc(x, y, radius, 0, 2.0 * Math.PI);
  this.fillStyle = '#CBE4DE';
  this.fill();
}

document.getElementById('train').onclick = function() {
  simulation.train();
}

function redraw() {
  const world = simulation.world();

  context.clearRect(0, 0, width, height)

  simulation.step();

  for (const food of world.food) {
    context.drawCircle(food.x * width, food.y * height, (0.01/ 2.0 * width))
  }

  for (const animal of world.animals) {
    context.drawTriangle(animal.x * width, animal.y * height, 0.01 * width, animal.rot);
  }

  requestAnimationFrame(redraw);
}

redraw();