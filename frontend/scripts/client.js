


var exampleSocket;

class Client{
    init_client =  function init_client() {
        var wsUri = (window.location.protocol=='https:'&&'wss://'||'ws://')+window.location.host + '/ws/';
    
        var exampleSocket = new WebSocket(wsUri);    
        console.log('Connecting...');
        exampleSocket.onopen = function() {
            console.log('Connected.');
        };
    
    }
    send_update(data){
        exampleSocket.send(data);
    }
}

var client = new Client();  

