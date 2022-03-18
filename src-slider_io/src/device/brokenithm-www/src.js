/*
  Post-process with https://babeljs.io/repl and https://javascript-minifier.com/
*/

const throttle = (func, wait) => {
  var ready = true;
  var args = null;
  return function throttled() {
    var context = this;
    if (ready) {
      ready = false;
      setTimeout(function () {
        ready = true;
        if (args) {
          throttled.apply(context);
        }
      }, wait);
      if (args) {
        func.apply(this, args);
        args = null;
      } else {
        func.apply(this, arguments);
      }
    } else {
      args = arguments;
    }
  };
};

// Element refs
var keys = document.getElementsByClassName("key");
var airKeys = [];
var midline = 0;
var touchKeys = [];
var allKeys = [];
var topKeys = airKeys;
var bottomKeys = touchKeys;
const compileKey = (key) => {
  const prev = key.previousElementSibling;
  const next = key.nextElementSibling;
  return {
    top: key.offsetTop,
    bottom: key.offsetTop + key.offsetHeight,
    left: key.offsetLeft,
    right: key.offsetLeft + key.offsetWidth,
    almostLeft: !!prev ? key.offsetLeft + key.offsetWidth / 4 : -99999,
    almostRight: !!next ? key.offsetLeft + (key.offsetWidth * 3) / 4 : 99999,
    kflag: parseInt(key.dataset.kflag) + (parseInt(key.dataset.air) ? 32 : 0),
    isAir: parseInt(key.dataset.air) ? true : window.allAir || false,
    prevKeyRef: prev,
    prevKeyKflag: prev
      ? parseInt(prev.dataset.kflag) + (parseInt(prev.dataset.air) ? 32 : 0)
      : null,
    nextKeyRef: next,
    nextKeyKflag: next
      ? parseInt(next.dataset.kflag) + (parseInt(next.dataset.air) ? 32 : 0)
      : null,
    ref: key,
  };
};
const isInside = (x, y, compiledKey) => {
  return (
    compiledKey.left <= x &&
    x < compiledKey.right &&
    compiledKey.top <= y &&
    y < compiledKey.bottom
  );
};
const compileKeys = () => {
  keys = document.getElementsByClassName("key");
  airKeys = [];
  touchKeys = [];
  for (var i = 0; i < keys.length; i++) {
    const compiledKey = compileKey(keys[i]);
    if (compiledKey.kflag < 32) {
      touchKeys.push(compiledKey);
    } else {
      airKeys.push(compiledKey);
    }
    allKeys.push(compiledKey);
  }

  touchKeys.memo = {};
  airKeys.memo = {};

  touchKeys.getAxis = (x, y) => x;
  airKeys.getAxis = (x, y) => y;

  var getKey = function (x, y) {
    var c = this.getAxis(x, y);
    var res = this.memo[c];
    if (res === undefined) {
      for (var i = 0; i < this.length; i++) {
        if (isInside(x, y, this[i])) {
          res = this[i];
          break;
        }
      }
      this.memo[c] = res;
    }
    return res;
  };

  touchKeys.getKey = getKey;
  airKeys.getKey = getKey;

  for (var i = 0; i < window.outerWidth; i++) {
    touchKeys.getKey(i, touchKeys[0].top);
  }

  for (var i = 0; i < window.outerHeight; i++) {
    airKeys.getKey(airKeys[0].left, i);
  }

  if (!config.invert) {
    // Not inverted
    topKeys = airKeys;
    bottomKeys = touchKeys;
    midline = touchKeys[0].top;
  } else {
    // Inverted
    topKeys = touchKeys;
    bottomKeys = airKeys;
    midline = touchKeys[0].bottom;
  }
};

const getKey = (x, y) => {
  if (y < midline) {
    return topKeys.getKey(x, y);
  } else {
    return bottomKeys.getKey(x, y);
  }
  return null;
};

// Button State
// prettier-ignore
var lastState = [
  0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
  0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
  0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
  0, 0, 0, 0, 0, 0, 0, 0,
];

function updateTouches(e) {
  try {
    e.preventDefault();

    // prettier-ignore
    var keyFlags = [
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
      0, 0, 0, 0, 0, 0, 0, 0,
    ];

    throttledRequestFullscreen();

    for (var i = 0; i < e.touches.length; i++) {
      const touch = e.touches[i];

      const x = touch.clientX;
      const y = touch.clientY;

      const key = getKey(x, y);

      if (!key) continue;

      setKey(keyFlags, key.kflag, key.isAir);

      if (key.isAir) continue;

      if (x < key.almostLeft) {
        setKey(keyFlags, key.prevKeyKflag, false);
      }

      if (key.almostRight < x) {
        setKey(keyFlags, key.nextKeyKflag, false);
      }
    }

    // Render keys
    for (var i = 0; i < allKeys.length; i++) {
      const key = allKeys[i];
      const kflag = key.kflag;
      if (keyFlags[kflag] !== lastState[kflag]) {
        if (keyFlags[kflag]) {
          key.ref.setAttribute("data-active", "");
        } else {
          key.ref.removeAttribute("data-active");
        }
      }
    }

    if (keyFlags !== lastState) {
      throttledSendKeys(keyFlags);
    }
    lastState = keyFlags;
  } catch (err) {
    alert(err);
  }
}
const throttledUpdateTouches = throttle(updateTouches, 10);

const setKey = (keyFlags, kflag, isAir) => {
  var idx = kflag;
  if (keyFlags[idx] && !isAir) {
    idx++;
  }
  keyFlags[idx] = 1;
};

const sendKeys = (keyFlags) => {
  if (wsConnected) {
    ws.send("b" + keyFlags.join(""));
  }
};
const throttledSendKeys = throttle(sendKeys, 10);

// Websockets
var ws = null;
var wsTimeout = 0;
var wsConnected = false;
const wsConnect = () => {
  ws = new WebSocket("ws://" + location.host + "/ws");
  ws.binaryType = "arraybuffer";
  ws.onopen = () => {
    ws.send("alive?");
  };
  ws.onmessage = (e) => {
    if (e.data.byteLength) {
      updateLed(e.data);
    } else if (e.data == "alive") {
      wsTimeout = 0;
      wsConnected = true;
    }
  };
};
const wsWatch = () => {
  if (wsTimeout++ > 2) {
    wsTimeout = 0;
    ws.close();
    wsConnected = false;
    wsConnect();
    return;
  }
  if (wsConnected) {
    ws.send("alive?");
  }
};

// Canvas vars
var canvas = document.getElementById("canvas");
var canvasCtx = canvas.getContext("2d");
var canvasData = canvasCtx.getImageData(0, 0, 33, 1);
const setupLed = () => {
  for (var i = 0; i < 33; i++) {
    canvasData.data[i * 4 + 3] = 255;
  }
};
setupLed();
const updateLed = (data) => {
  const buf = new Uint8Array(data);
  for (var i = 0; i < 31; i++) {
    canvasData.data[i * 4 + 4] = buf[i * 3]; // r
    canvasData.data[i * 4 + 5] = buf[i * 3 + 1]; // g
    canvasData.data[i * 4 + 6] = buf[i * 3 + 2]; // b
  }
  canvasData.data[0] = buf[0];
  canvasData.data[1] = buf[1];
  canvasData.data[2] = buf[2];
  canvasData.data[128] = buf[90];
  canvasData.data[129] = buf[91];
  canvasData.data[130] = buf[92];

  canvasCtx.putImageData(canvasData, 0, 0);
};

// Fullscreener
const fs = document.getElementById("fullscreen");
const requestFullscreen = () => {
  if (!document.fullscreenElement && screen.height <= 1024) {
    if (fs.requestFullscreen) {
      fs.requestFullscreen();
    } else if (fs.mozRequestFullScreen) {
      fs.mozRequestFullScreen();
    } else if (fs.webkitRequestFullScreen) {
      fs.webkitRequestFullScreen();
    }
  }
};
const throttledRequestFullscreen = throttle(requestFullscreen, 3000);

// Do update hooks
const cnt = document.getElementById("main");

cnt.addEventListener("touchstart", updateTouches);
cnt.addEventListener("touchmove", updateTouches);
cnt.addEventListener("touchend", updateTouches);

// cnt.addEventListener("touchstart", throttledUpdateTouches);
// cnt.addEventListener("touchmove", throttledUpdateTouches);
// cnt.addEventListener("touchend", throttledUpdateTouches);

// Load config
const readConfig = (config) => {
  var style = "";

  if (!!config.invert) {
    style += `.container, .air-container {flex-flow: column-reverse nowrap;} `;
  }

  var bgColor = config.bgColor || "rbga(0, 0, 0, 0.9)";
  if (!config.bgImage) {
    style += `#fullscreen {background: ${bgColor};} `;
  } else {
    style += `#fullscreen {background: ${bgColor} url("${config.bgImage}") fixed center / cover!important; background-repeat: no-repeat;} `;
  }

  if (typeof config.ledOpacity === "number") {
    if (config.ledOpacity === 0) {
      style += `#canvas {display: none} `;
    } else {
      style += `#canvas {opacity: ${config.ledOpacity}} `;
    }
  }

  if (typeof config.keyColor === "string") {
    style += `.key[data-active] {background-color: ${config.keyColor};} `;
  }
  if (typeof config.keyColor === "string") {
    style += `.key.air[data-active] {background-color: ${config.lkeyColor};} `;
  }
  if (typeof config.keyBorderColor === "string") {
    style += `.key {border: 1px solid ${config.keyBorderColor};} `;
  }
  if (!!config.keyColorFade && typeof config.keyColorFade === "number") {
    style += `.key:not([data-active]) {transition: background ${config.keyColorFade}ms ease-out;} `;
  }

  if (typeof config.keyHeight === "number") {
    if (config.keyHeight === 0) {
      style += `.touch-container {display: none;} `;
    } else {
      style += `.touch-container {flex: ${config.keyHeight};} `;
    }
  }

  if (typeof config.lkeyHeight === "number") {
    if (config.lkeyHeight === 0) {
      style += `.air-container {display: none;} `;
    } else {
      style += `.air-container {flex: ${config.keyHeight};} `;
    }
  }

  var styleRef = document.createElement("style");
  styleRef.innerHTML = style;
  document.head.appendChild(styleRef);
};

// Initialize
const initialize = () => {
  readConfig(config);
  compileKeys();
  wsConnect();
  setInterval(wsWatch, 1000);
};
initialize();

// Update keys on resize
window.onresize = compileKeys;
