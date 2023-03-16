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

CanvasRenderingContext2D.prototype.drawTriangle = function(x, y, size) {
  this.beginPath();
  this.moveTo(x, y);
  this.lineTo(x + size, y + size);
  this.lineTo(x - size, y + size);
  this.fillStyle = 'rgb(0, 0, 0)';
  this.fill();
};

for (const animal of simulation.world().animals) {
  context.drawTriangle(animal.x * width, animal.y * height, 0.02 * width);
}
