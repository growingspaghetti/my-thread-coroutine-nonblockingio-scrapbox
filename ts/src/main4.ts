// https://www.gesource.jp/weblog/?p=7663

const p1 = new Promise<string>(
    (resolve: (value?: string) => void, reject: (reason?: any) => void) => {
    setTimeout(() => {
        resolve("OK");
    }, 1000);
}).then(
    (value: string) => {
        console.log(value);
    }
).catch(
    (reason: any) => {
        console.log(reason);
    }
);

console.log(p1)