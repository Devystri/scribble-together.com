class Client{


    add_in_buffer = function add_in_buffer(data){
        self.buffer.push(data);
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

    init_client =  function init_client() {
        self.wsUri = (window.location.protocol=='https:'&&'wss://'||'ws://')+window.location.host + '/ws/';
        self.buffer = [];
        self.socket = new WebSocket(self.wsUri);    
        console.log('Connecting...');
        self.socket.onopen = function() {
            console.log('Connected.');
        };

        function send_update(){
            if(self.socket.readyState){
                self.socket.send(JSON.stringify(self.buffer));
            }
    
        }

        setInterval(function send_buffer (){
            if (self.buffer.length > 0) {
                send_update();
                self.buffer = [];
            }
        }, 250);

    }
}

var client = new Client();  

