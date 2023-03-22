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
  this.fillStyle = 'rgb(0, 0, 0)';
  this.stroke();
};

for (const animal of simulation.world().animals) {
  context.drawTriangle(animal.x * width, animal.y * height, 0.02 * width, animal.rot);
}
