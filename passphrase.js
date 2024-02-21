const readline = require('readline');
const fs = require('fs');

const path = process.argv[2];
const n = process.argv[3] ?? 1;

const readStream = fs.createReadStream(path);
const rl = readline.createInterface({
    input: readStream,
    crlfDelay: Infinity
});

const wordSizes = {};

const getRandomWords = (words, count) => {
    const randomWords = [];
    const wordCount = Math.min(count, words.length);

    while (randomWords.length < wordCount) {
        const randomIndex = Math.floor(Math.random() * words.length);
        const randomWord = words[randomIndex];

        if (!randomWords.includes(randomWord)) {
            randomWords.push(randomWord);
        }
    }

    return randomWords;
};

rl.on('line', (word) => {
    if (word.includes('-') || /^[A-ZÅÄÖ]/.test(word)) return;
    
    const wordLength = word.length;
    const sizeArr = wordSizes[wordLength];
    if (sizeArr) {
        sizeArr.push(word);
    } else {
        wordSizes[wordLength] = [word];
    }
});

rl.on('close', () => {
    const words = wordSizes[4].concat(wordSizes[5]);

    console.log('Count:', words.length);

    for (let i = 0; i < n; i++) {   
        const passphrase = getRandomWords(words, 6).join(' ');
        console.log(passphrase);
    }
});