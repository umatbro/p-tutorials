functions = [
    function draw() {
        let canvas = document.querySelector('#tut');

        if (!canvas.getContext) return; // no support for canvaas
        
        let ctx = canvas.getContext('2d');
        
        ctx.fillStyle = 'rgb(200, 0, 0)';
        ctx.fillRect(10, 10, 50, 50);
        ctx.fillStyle = 'rgba(0, 0, 200, 0.5)';
        ctx.fillRect(30, 30, 50, 50);

        ctx.fillStyle = 'rgb(0, 0, 0)';
        ctx.fillRect(90, 50, 50, 50);
        ctx.clearRect(100, 60, 30, 30);
        ctx.strokeRect(110, 70, 10, 10);
    },

    function drawPaths() {
        let canvas = document.querySelector('#paths');
        let ctx = canvas.getContext('2d');

        // draw triangle
        ctx.beginPath();
        ctx.moveTo(75, 50);
        ctx.lineTo(100, 75);
        ctx.lineTo(75, 100);
        ctx.fill();
    },

    function drawSmilingFace() {
        let canvas = document.querySelector('#smiling-face');
        let ctx = canvas.getContext('2d');
        let [w, h] = [canvas.width, canvas.height];
        ctx.beginPath();
        console.log(w, h);
        ctx.arc(w/2, h/2, 10, 0, Math.PI * 2, true);
    }
]

functions.forEach(func => {
    document.addEventListener('DOMContentLoaded', func);
});
