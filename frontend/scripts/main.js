//COLORS

//store colors in an array
var colors = { "black": "#000000", "red": "#ff0000", "orange": "#ffa500", "yellow": "#FFFF00", "lightGreen": "#9acd32", "green": "#00ff00", "lightBlue": "#00ffff", "blue": "#00a5ff", "darkPurpule": "#0000a0", "purpule": "#800080", "pink": "#ff00ff"};

for (key in colors){
    //Create a li 
    //Add the color to the li
    document.getElementById('colors-id').innerHTML += "<li class='color-element' style='background-color:" + colors[key] + "'>" + key + "</li>";
}

var color = "#000000";

//CANVAS

//Variables
var canvas, ctx, canvasWidth, canvasHeight, mouseX, mouseY, lastMouseX, lastMouseY;

//VAR COLOR WITH BUTTONS
//If a "color-element" is clicked then change the color variable

document.addEventListener('click', function(e) {
    e = e || window.event;
    var target = e.target;
    if (target.className == "color-element"){
        color_name = target.textContent;
        color = colors[color_name];
        }
}, false);

window.addEventListener("resize", function(e){
    canvas.width = document.body.clientWidth; 
    canvas.height = document.body.clientHeight; 
    canvasWidth = canvas.width;
    canvasHeight = canvas.height;
    //Resize canvas server
    serverCanvas.width = document.body.clientWidth;
    serverCanvas.height = document.body.clientHeight;
    serverCanvasWidth = serverCanvas.width;
    serverCanvasHeight = serverCanvas.height;
}, true);

const PIXEL_SIZE = 10;

function draw(){
    ctx.beginPath();
    ctx.fillStyle = color;
    ctx.fillRect( (2*mouseX)/2, (2*lastMouseY)/2, PIXEL_SIZE, PIXEL_SIZE );
    ctx.closePath();
    save(Math.round(mouseX/PIXEL_SIZE), Math.round(mouseY/10), color);
}

function moveMouse(e){
    mouseX =  Math.round((e.clientX - canvas.offsetLeft)/PIXEL_SIZE)*PIXEL_SIZE;
    mouseY =  Math.round((e.clientY - canvas.offsetTop)/PIXEL_SIZE)*PIXEL_SIZE;
    if (e.buttons == 1){
        draw();
    }
    lastMouseX = mouseX;
    lastMouseY = mouseY;
}

function init(){
    canvas = document.getElementById('drawing-canvas');
    canvas.width = document.body.clientWidth; 
    canvas.height = document.body.clientHeight; 

    ctx = canvas.getContext("2d");
    canvasWidth = canvas.width;
    canvasHeight = canvas.height;
    canvas.addEventListener("mousemove", moveMouse);
} 

init();

//SAVE IMAGE
let image = [];

function save(x, y, color){
    var pixel = {x: x, y: y, color: color};
    let newLength = image.push(pixel);
}

//LOAD IMAGE
function load(){
    for (let i = 0; i < image.length; i++){
        ctx.beginPath();
        ctx.fillStyle = image[i].color;
        ctx.fillRect(2*(image[i].x*PIXEL_SIZE)/2, (2*image[i].y*PIXEL_SIZE)/2, PIXEL_SIZE, PIXEL_SIZE);
        ctx.closePath();
    }
}

//Canvas Server
//Size of canvas
 var serverCanvas, serverCtx, serverCanvasWidth, serverCanvasHeight;
 
 function initServerCanvas(){
    serverCanvas = document.getElementById('server-canvas');
    serverCanvas.width = document.body.clientWidth;
    serverCanvas.height = document.body.clientHeight;
    serverCtx = serverCanvas.getContext("2d");
    serverCanvasWidth = serverCanvas.width;
    serverCanvasHeight = serverCanvas.height;
}

initServerCanvas();

//LOAD IMAGE FROM SERVER
let serverImage = [{x: 0, y: 0, color: "#000000"}, {x: 72, y: 23, color: "#000000"}, {x: 72, y: 23, color: "#000000"}, {x: 71, y: 23, color: "#000000"}, {x: 71, y: 23, color: "#000000"}, {x: 70, y: 23, color: "#000000"}, {x: 69, y: 23, color: "#000000"}, {x: 69, y: 23, color: "#000000"}, {x: 68, y: 23, color: "#000000"}, {x: 68, y: 23, color: "#000000"}];

function loadServer(){
    for (let i = 0; i < serverImage.length; i++){
        serverCtx.beginPath();
        serverCtx.fillStyle = serverImage[i].color;
        serverCtx.fillRect(2*(serverImage[i].x*PIXEL_SIZE)/2, (2*serverImage[i].y*PIXEL_SIZE)/2, PIXEL_SIZE, PIXEL_SIZE);
        serverCtx.closePath();
    }
}

loadServer();

import { init_client } from "./client.js";

init_client();