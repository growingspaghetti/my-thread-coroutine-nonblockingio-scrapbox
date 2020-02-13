async function giladOk() {
    return "Gilad"
}
giladOk().then(value => {
    console.log(value);
});

async function giladErr() {
    throw new Error("Error")
    return ""
}
giladErr().catch(err => {
    console.error(err)
});