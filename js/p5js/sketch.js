let particles = [];
function setup() {
    createCanvas(windowWidth, windowHeight);

}

function draw() {
    background(0);
    let spawnIntensity = mouseIsPressed ? 6 : (random() > 0.9);
    for (let i = 0; i < spawnIntensity; i++) {
        let particle = new Particle(mouseX, mouseY);
        if (mouseIsPressed) particle.setRandomColor();
        particles.push(particle);
    }

    for (let particle of particles) {
        let gravity = createVector(0, 0.2);
        particle.applyForce(gravity);

        particle.update();
        // particle.edges();
        particle.show();
    }

    particles = particles.filter((particle) => !particle.finished());
}
