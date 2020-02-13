const wait = (ms) => new Promise(res => setTimeout(res, ms));
const startAsync = async (callback) => {
    await wait(1000);
    callback('Hello');
    await wait(1000);
    callback('And Welcome');
    await wait(1000);
    callback('To Async Await Using TypeScript');
};
startAsync(text => console.log(text));
