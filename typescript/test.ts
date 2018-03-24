let names: string[] = ['barbra', 'dżon', 'mandy'];

for (var i of names) {
    setTimeout(() => {
        console.log(i);
    });
}