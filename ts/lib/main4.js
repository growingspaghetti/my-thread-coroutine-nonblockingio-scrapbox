// https://www.gesource.jp/weblog/?p=7663
const p1 = new Promise((resolve, reject) => {
    setTimeout(() => {
        resolve("OK");
    }, 1000);
}).then((value) => {
    console.log(value);
}).catch((reason) => {
    console.log(reason);
});
console.log(p1);
