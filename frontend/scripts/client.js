



class Client{
    init_client =  function init_client() {
        self.wsUri = (window.location.protocol=='https:'&&'wss://'||'ws://')+window.location.host + '/ws/';
    
        self.socket = new WebSocket(wsUri);    
        console.log('Connecting...');
        socket.onopen = function() {
            console.log('Connected.');
        };
    
    }
    send_update(data){
        if(self.socket.readyState){
            self.socket.send(JSON.stringify(data));
        }

    }
    download_pixels(adress){
        const request = new XMLHttpRequest();
        const url= window.location.protocol + '//' + window.location.host + "/tile/get/" + adress;
        request.open("GET", url, false);
        request.send(null);
        
        if (request.status === 200) {
            console.log(request.responseText);
            return request.responseText;
        }
    }
}

var client = new Client();  

