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

const PIXEL_SIZE = 10;

function draw(){
    ctx.beginPath();
    ctx.fillStyle = color;

    ctx.fillRect( (2*mouseX - PIXEL_SIZE)/2, (2*lastMouseY - PIXEL_SIZE)/2, PIXEL_SIZE, PIXEL_SIZE );

    ctx.closePath();

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
    console.log(canvasHeight, canvasWidth);
    canvas.addEventListener("mousemove", moveMouse);
} 

init();