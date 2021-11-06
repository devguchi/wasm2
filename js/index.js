const rust = import("../pkg/index.js");
rust.then(m => {
    let counter = m.Counter.new('A');
    console.log(counter);
    console.log(counter.count());
    counter.increment();
    console.log(counter.count());
}).catch(console.error);
