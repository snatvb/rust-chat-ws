const randInt = require('./randInt')

const CHARS = '0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ'
const DEFAULT_CODE_LENGTH = 9

const randFrom = (alphabet, length) => {
  let uid = '';
  const maxIndex = alphabet.length - 1;
  while (length-- > 0) {
    uid += alphabet[randInt(0, maxIndex)];
  }
  return uid;
}

const randCode = () => randFrom('0123456789', DEFAULT_CODE_LENGTH)

const uCode = (length = DEFAULT_CODE_LENGTH) => randFrom(CHARS, length)

module.exports = {
  uCode,
  randCode,
}