const gilad = new Promise(resolve => resolve('Gilad'));
// const error = new Promise((resolve, reject) => reject(new Error('Error')));
async function a() {
    try {
        console.log(await gilad);
    } catch(e) {
        console.log('caught error', e);
    }
};

a()
