const digits_only = string => [...string].every(c => '-0123456789'.includes(c));


function buffer_to_string(buffer){
    let string = "";
    for (let i = 0; i < buffer.length; i++){
        if (buffer[i].x == undefined || buffer[i].y == undefined || buffer[i].color == undefined){
            continue;
        }
        string += buffer[i].x.toString().padStart(3, '0') + buffer[i].y.toString().padStart(3, '0');
        if (buffer[i].color < 0) {
            string += buffer[i].color.toString().padStart(2, '0') + "\n";
        }else{
            string += buffer[i].color.toString().padStart(3, '0') + "\n";
        }
    }

    return string;
}



function string_to_buffer(string){
    let buffer = [];
    let parameters = string.split('\n');
    for (let i = 0; i < parameters.length; i++){
        if (parameters[i].length != 9){
            continue;
        }
        if (!digits_only(parameters[i])){
            continue;
        }
        buffer.push({x: parseInt(parameters[i].substring(0, 3)), y: parseInt(parameters[i].substring(3, 6)), color: parseInt(parameters[i].substring(6, 9))});

    }

    return buffer;

}
   

class Client{


    add_in_buffer = function add_in_buffer(data){
        this.buffer.push(data);
    }
 
    download_pixels =  function download_pixels(adress){
        const request = new XMLHttpRequest();
        const url= window.location.protocol + '//' + window.location.host + "/tile/get/" + adress;
        request.open("GET", url, false);
        request.send(null);
        
        if (request.status === 200) {
            console.log(request.responseText);
            return request.responseText;
        }
    }
    register(x, y) {
        this.socket.send("/register map/" + x.toString() + "_" + y.toString());
    }

    init_client (){
        this.wsUri = (window.location.protocol=='https:'&&'wss://'||'ws://')+window.location.host + '/ws/';
        this.buffer = [];
        this.socket = new WebSocket(this.wsUri);    
        console.log('Connecting...');
        this.socket.onopen = () =>{
            console.log('Connected.');
            this.register(0,0)
        };

        this.socket.onmessage = (event) => {
            let param = event.data.split(' ');
            if (param[0] == "/update"){
                let data = string_to_buffer(param[1]);
                this.update(data);
                console.log(data);

            }
        } 

        setInterval( ()=>{
            if (this.buffer.length > 0) {
                if(this.socket.readyState){
                    this.socket.send("/send " + buffer_to_string(this.buffer));
                }
                this.buffer = [];
            }
        }, 250);

    }
}

var client = new Client();  

