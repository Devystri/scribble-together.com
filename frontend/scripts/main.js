//COLORS

//store colors in an array
var colors = {  "black": "rgb(0, 0, 0)", "red": "rgb(255, 0, 0)", "orange": "rgb(255, 165, 0)", "yellow": "rgb(255, 255, 0)", "lightGreen": "rgb(154, 205, 50)", "green": "rgb(0, 255, 0)", "lightBlue": "rgb(0, 255, 255)", "blue": "rgb(0, 165, 255)", "darkPurpule": "rgb(0, 0, 160)", "purpule": "rgb(128, 0, 128)", "pink": "rgb(255, 0, 255)"};
var colors_indexes_by_hex = {"rgb(0, 0, 0)": 0, "rgb(255, 0, 0)": 1, "rgb(255, 165, 0)": 2, "rgb(255, 255, 0)": 3, "rgb(154, 205, 50)": 4, "rgb(0, 255, 0)": 5, "rgb(0, 255, 255)": 6, "rgb(0, 165, 255)": 7, "rgb(0, 0, 160)": 8, "rgb(128, 0, 128)": 9, "rgb(255, 0, 255)": 10};
var colors_hex_by_index = {0: "rgb(0, 0, 0)", 1: "rgb(255, 0, 0)", 2: "rgb(255, 165, 0)", 3: "rgb(255, 255, 0)", 4: "rgb(154, 205, 50)", 5: "rgb(0, 255, 0)", 6: "rgb(0, 255, 255)", 7: "rgb(0, 165, 255)", 8: "rgb(0, 0, 160)", 9: "rgb(128, 0, 128)", 10: "rgb(255, 0, 255)"};

for (key in colors){
    //Create a li 
    //Add the color to the li
    document.getElementById('colors-id').innerHTML += "<li class='color-element' style='background-color:" + colors[key] + "'>" + key + "</li>";
}

//Generate Different Shades of the Same Color
function generateShades(colorClicked){
    //Create a shades-colors class div in the colors-id element of id
    document.getElementById('colors-id').innerHTML += "<div class='shades-colors'></div>";
    colorRGB = colorClicked.replace('rgb(', '').replace(')', '').split(',');
    alert(colorRGB);
}


var color = 0;

//CANVAS

//Variables
var canvas, ctx, canvasWidth, canvasHeight, mouseX, mouseY, lastMouseX, lastMouseY;

//VAR COLOR WITH BUTTONS
//If a "color-element" is clicked then change the color variable

document.addEventListener('click', function(e) {
    e = e || window.event;
    var target = e.target;
    color_name = target.textContent;
    if (target.className == 'color-element' && color == colors[color_name]){
        generateShades(colors[color_name]);
    }
    else if (target.className == "color-element"){
        color = colors_indexes_by_hex[colors[color_name]];
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

    loadServer();
    load();
}, true);

const PIXEL_SIZE = 10;

function draw(){
    ctx.beginPath();
    ctx.fillStyle = colors_hex_by_index[color];
    ctx.fillRect( mouseX, mouseY, PIXEL_SIZE, PIXEL_SIZE );
    ctx.closePath();
    save(Math.round(mouseX/PIXEL_SIZE), Math.round(mouseY/PIXEL_SIZE), color);
}

function moveMouse(e){
    mouseX =  Math.round((e.clientX - canvas.offsetLeft)/PIXEL_SIZE)*PIXEL_SIZE;
    mouseY =  Math.round((e.clientY - canvas.offsetTop)/PIXEL_SIZE)*PIXEL_SIZE;
    //If the MouseX en MouseY coordinates are not in the serverImage array
    if (e.buttons == 1){
        var drawed = false;
        for( i = 0; i < serverImage.length; i++) {
            if(Math.round(mouseX/PIXEL_SIZE) == serverImage[i].x && Math.round(mouseY/PIXEL_SIZE) == serverImage[i].y){
                //drawed = true;
                break;
            }
             
        }
        var target_color = colors_indexes_by_hex[color]
        for(i = 0; i < image.length; i++){
            var pixel = image[i];
            if(pixel.color == target_color && Math.round(mouseX/PIXEL_SIZE) == pixel.x && Math.round(mouseY/PIXEL_SIZE) == pixel.y){
                drawed = true;
                break;
            }
        }

        if(!drawed){
            draw();

        }    
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

// To send
buffer = [];

//SAVE IMAGE
let image = [];

function save(x, y, color){
    var pixel = {x: x, y: y, color: color};
    client.add_in_buffer(pixel);
    let newLength = image.push(pixel);
}

//LOAD IMAGE
function load(){
    for (let i = 0; i < image.length; i++){
        ctx.beginPath();
        ctx.fillStyle = colors_hex_by_index[image[i].color];
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
let serverImage = [];

function loadServer(){
    csv_pixels = client.download_pixels("map/0_0");
    serverImage = string_to_buffer(csv_pixels);
    
    for (let i = 0; i < serverImage.length; i++){
        if (serverImage[i].color < 0){
            continue;
        }
        serverCtx.beginPath();
        
        serverCtx.fillStyle = colors_hex_by_index[serverImage[i].color];
        serverCtx.fillRect(serverImage[i].x*PIXEL_SIZE, serverImage[i].y*PIXEL_SIZE, PIXEL_SIZE, PIXEL_SIZE);
        serverCtx.closePath();
    }
}
client.init_client();
loadServer();

client.update = (data) =>{
    for (let i = 0; i < data.length; i++){
        if (data[i].color < 0){
            serverCtx.fillStyle = "rgba(0,0,0,0)";
        }else{
            serverCtx.fillStyle = colors_hex_by_index[data[i].color];
        }
        serverCtx.beginPath();
        
        serverCtx.fillRect(data[i].x*PIXEL_SIZE, data[i].y*PIXEL_SIZE, PIXEL_SIZE, PIXEL_SIZE);
        serverCtx.closePath();
    }
}

// Disable the contextual menu
canvas.oncontextmenu = function() {
    return false;
}
//put the cursor move when the mouse is right clicked
canvas.onmousedown = function(e){
    if (e.button == 2){
        canvas.style.cursor = "move";
    }else{
        canvas.style.cursor = "crosshair";
    }
}
//When the right click is released, set the cursor to crosshair
canvas.onmouseup = function(e){
    canvas.style.cursor = "crosshair";
}