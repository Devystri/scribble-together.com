


var exampleSocket;

class Client{
    init_client =  function init_client() {
        var wsUri = (window.location.protocol=='https:'&&'wss://'||'ws://')+window.location.host + '/ws/';
    
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
}

var client = new Client();  

