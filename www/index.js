import * as sim from "lib-simulation-wasm";

var simulation = new sim.Simulation(sim.Simulation.default_config());
const viewport = document.getElementById("viewport");
const scale = window.devicePixelRatio || 1;
const width = viewport.width * scale;
const height = viewport.height * scale;

viewport.style.width = width + "px";
viewport.style.height = height + "px";

const context = viewport.getContext("2d");
context.scale(scale, scale);

const selection = simulation.selection_methods();
var select = document.getElementById("selectionMethod");

for (const i in selection) {
  var opt = document.createElement("option");

  opt.text = opt.value = selection[i];
  select.add(opt, 0);
}

const mutation = simulation.mutation_methods();
select = document.getElementById("mutationMethod");

for (const i in mutation) {
  var opt = document.createElement("option");

  opt.text = opt.value = mutation[i];
  select.add(opt, 0);
}

const crossover = simulation.crossover_methods();
select = document.getElementById("crossoverMethod");

for (const i in crossover) {
  var opt = document.createElement("option");

  opt.text = opt.value = crossover[i];
  select.add(opt, 0);
}

CanvasRenderingContext2D.prototype.drawTriangle = function (x, y, size, rot) {
  this.beginPath();
  this.moveTo(x - Math.sin(rot) * size * 1.5, y + Math.cos(rot) * size * 1.5);
  this.lineTo(
    x - Math.sin(rot + (2.0 / 3.0) * Math.PI) * size,
    y + Math.cos(rot + (2.0 / 3.0) * Math.PI) * size
  );
  this.lineTo(
    x - Math.sin(rot + (4.0 / 3.0) * Math.PI) * size,
    y + Math.cos(rot + (4.0 / 3.0) * Math.PI) * size
  );
  this.lineTo(x - Math.sin(rot) * size * 1.5, y + Math.cos(rot) * size * 1.5);
  this.fillStyle = "#0E8388";
  this.strokeStyle = "#DDDDDD";
  this.stroke();
  this.fill();
};

CanvasRenderingContext2D.prototype.drawCircle = function (x, y, radius) {
  this.beginPath();
  this.arc(x, y, radius, 0, 2.0 * Math.PI);
  this.fillStyle = "#CBE4DE";
  this.fill();
};

document.getElementById("train").onclick = function () {
  console.log(simulation.train());
};

document.getElementById("submit").onclick= function () {
  let conf = sim.Simulation.default_config();

  const neurons = parseInt(document.getElementById("neurons").value);
  const minSpeed = parseFloat(document.getElementById("minSpeed").value);
  const maxSpeed = parseFloat(document.getElementById("maxSpeed").value);
  const accel = parseFloat(document.getElementById("accel").value);
  const genLen = parseInt(document.getElementById("genLen").value);
  const actCount = parseInt(document.getElementById("actCount").value);
  const pntCount = parseInt(document.getElementById("pntCount").value);
  const selectionMethod = document.getElementById("selectionMethod").value;

  conf.brain_neurons = neurons;
  conf.speed_min = minSpeed;
  conf.speed_max = maxSpeed;
  conf.speed_accel = accel;
  conf.gen_len = genLen;
  conf.count_animal = actCount;
  conf.count_food = pntCount;
  conf.selection_method = selectionMethod;

  simulation = new sim.Simulation(conf);
};

function redraw() {
  const world = simulation.world();

  context.clearRect(0, 0, width, height);

  simulation.step();

  for (const food of world.food) {
    context.drawCircle(food.x * width, food.y * height, (0.01 / 2.0) * width);
  }

  for (const animal of world.animals) {
    context.drawTriangle(
      animal.x * width,
      animal.y * height,
      0.01 * width,
      animal.rot
    );
  }

  requestAnimationFrame(redraw);
}

console.log(simulation.config.selection_method)
redraw();
