const axios5 = require('axios').default;

const p3: Promise<string> = axios5.get('https://www.youtube.com/', { timeout: 5 });
const p4: Promise<string> = new Promise((resolve, reject) => {
    // Go's time.After(1 * time.MilliSecond)
    setTimeout(() => {
        resolve('my call back function to be triggered!');
    }, 1 /* delay */)
});

let race: Promise<string> = Promise.race([p3, p4])

race.then((resolve) => console.log(resolve))
    .catch(error => console.log(error))
    .then(() => console.log("anyway"));


// Native JavaScript promises don't have any timeout mechanism.
// https://stackoverflow.com/questions/32461271/nodejs-timeout-a-promise-if-failed-to-complete-in-time