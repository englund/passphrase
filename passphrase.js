const readline = require('readline');
const fs = require('fs');

const path = process.argv[2];
const n = process.argv[3] ?? 1;

const validWordLength = [4, 5];
const illegalChars = ['-', '.'];

const rl = readline.createInterface({
    input: fs.createReadStream(path),
    crlfDelay: Infinity
});

const words = [];

rl.on('line', (word) => {
    const startsWithUppercase = /^[A-ZÅÄÖ]/.test(word);
    const hasCorrectLength = validWordLength.includes(word.length);
    const hasIllegalChars = illegalChars.some(char => word.includes(char));

    if (hasCorrectLength && !hasIllegalChars && !startsWithUppercase) {
        words.push(word);
    }
});

rl.on('close', () => {
    console.log('Word count:', words.length);
    for (let i = 0; i < n; i++) {   
        const passphrase = getRandomWords(words, 6).join(' ');
        console.log(passphrase);
    }
});

function getRandomWords(words, count) {
    const randomWords = [];
    while (randomWords.length < count) {
        const randomWord = words[Math.floor(Math.random() * words.length)];
        if (!randomWords.includes(randomWord)) {
            randomWords.push(randomWord);
        }
    }
    return randomWords;
}