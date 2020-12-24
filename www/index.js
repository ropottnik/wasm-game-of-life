import { Universe } from "wasm-game-of-life";

const canvas = document.getElementById("game-of-life-canvas");

const universe = Universe.new(100, 100);
universe.set_rle_shape('28b2o17b$28bobo16b$28bo18b$31bo15b$24b3o4bobo13b$24bo6bo2bo3b3o6b$25bo7b2o2bo2b2o5b$27b3o7bo5b2o2b$20b2o15b2o4b3ob$20bobo5bobo3b3o9bo$20bo8b2o2bo2bo6bo2bo$23bo9bo9bo3b$16b3o4bobo7b2o8bobob$16bo6bo2bo3b3o14b$17bo7b2o2bo2bo14b$19b3o7bo17b$12b2o15b2o16b$12bobo5bobo3b3o18b$12bo8b2o2bo2bo18b$15bo9bo21b$8b3o4bobo7b2o20b$8bo6bo2bo3b3o22b$9bo7b2o2bo2bo22b$11b3o7bo25b$4b2o15b2o24b$4bobo5bobo3b3o26b$4bo8b2o2bo2bo26b$7bo9bo29b$3o4bobo7b2o28b$o6bo2bo3b3o30b$bo7b2o2bo2bo30b$3b3o7bo33b$13b2o32b$4bobo3b3o34b$5b2o2bo2bo34b$9bo37b$9b2o36b$6b3o38b$5bo2bo38b$5bo41b$5b2o40b$6bo40b2$7b2ob3o34b$7b2o38b$8bo3bo34b$9b2o', 50, 100);


let animationId = null;
const isPaused = () => {
  return animationId === null;
}

const playPausButton = document.getElementById("play-pause");

const play = () => {
  playPausButton.textContent = "\u23F8";
  renderLoop();
}

const pause = () => {
  playPausButton.textContent = "\u23EF";
  cancelAnimationFrame(animationId);
  animationId = null;
}

playPausButton.addEventListener("click", event => {
  if (isPaused()) {
    play();
  } else {
    pause();
  }
});

const renderLoop = () => {
  canvas.textContent = universe.render();
  universe.tick();

  animationId = requestAnimationFrame(renderLoop);
}

play();
