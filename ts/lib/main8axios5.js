const axios5 = require('axios').default;
const p3 = axios5.get('https://www.youtube.com/', { timeout: 5 });
const p4 = new Promise((resolve, reject) => {
    // Go's time.After(1 * time.MilliSecond)
    setTimeout(() => {
        resolve('my call back function to be triggered!');
    }, 100, () => {
    });
});
let race = Promise.race([p3, p4]);
race.then((resolve) => console.log(resolve))
    .catch(error => console.log(error))
    .then(() => console.log("anyway"));
