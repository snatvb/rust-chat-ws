const WebSocket = require('ws')

let count = 0;
for (let index = 0; index < 30; index++) {
  const ws = new WebSocket('ws://localhost:8089', 'rust-websocket')

  ws.on('open', () => {
    console.log('open')
    for (let i = 0; i < 50; i++) {
      ws.send(`hoba text ${index}-${i}`)
    }
    console.log('sended')
  })

  ws.on('message', (data) => {
    count += 1;
    console.log(`message ${count}:`, data)
    // ws.close();
  })
}
