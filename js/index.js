const rust = import("../pkg/index.js");
rust.then(m => {
    console.log('hoge');
}).catch(console.error);
