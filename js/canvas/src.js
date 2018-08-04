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
        // head
        ctx.arc(75, 75, 50, 0, Math.PI * 2, true);
        
        // smile
        ctx.moveTo(110, 75);
        ctx.arc(75, 75, 35, 0, Math.PI, false);
        
        // eyes
        ctx.moveTo(100, 65);
        ctx.arc(95, 65, 5, 0, Math.PI * 2, true);
        ctx.moveTo(60, 65);
        ctx.arc(55, 65, 5, 0, Math.PI * 2, true);

        ctx.stroke();
    },

    function drawBezierCurves() {
        let canvas = document.querySelector('#bezier-curves');
        let ctx = canvas.getContext('2d');

        ctx.beginPath();
        ctx.moveTo(75, 25);
        ctx.quadraticCurveTo(25, 25, 25, 62.5);
        ctx.quadraticCurveTo(25, 100, 50, 100);
        ctx.quadraticCurveTo(50, 120, 30, 125);
        ctx.quadraticCurveTo(60, 120, 65, 100);
        ctx.quadraticCurveTo(125, 100, 125, 62.5);
        ctx.quadraticCurveTo(125, 25, 75, 25);
        
        ctx.stroke();
    },
]

functions.forEach(func => {
    document.addEventListener('DOMContentLoaded', func);
});
