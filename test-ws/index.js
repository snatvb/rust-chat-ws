const WebSocket = require('ws')
const process = require('process')

const stressTest = () => {
  let count = 0;
  for (let index = 0; index < 30; index++) {
    const ws = new WebSocket('ws://localhost:8089')

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
}

const chat = () => {
  const ws = new WebSocket('ws://localhost:8089')

    ws.on('open', () => {
      console.log('open')
      ws.send(JSON.stringify({
        type: 'message',
        payload: {
          receiver_id: 1,
          text: 'Hello my friend!',
        },
      }))
      console.log('sended')
    })

    ws.on('message', (data) => {
      console.log(`message:`, data)
    })
}

const main = () => {
  const type = process.argv[2]
  switch (type) {
    case 'stress':
      stressTest()
      break

    case 'chat':
      chat()
      break

    default:
      console.warn('Unknown type of task', type)
      break
  }

}

main()
