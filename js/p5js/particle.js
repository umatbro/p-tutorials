const ALABASTER = [244, 243, 238];
const MACARONI_AND_CHEESE = [255, 190, 134]
const MUSTARD = [255, 225, 86]
const BIANCHED_ALMOND = [255, 233, 206]
const CHERRY_BLOSSOM_PINK = [255, 181, 194]
const BLUE_CANYOLA = [55, 119, 255]
const FRENCH_LIME = [150, 245, 80]
const BYZANTIUM = [122, 48, 108]

// cool rainbow
const SIZZLING_RED = [255, 89, 94]
const SUNGLOW  = [255, 202, 58];
const YELLOW_GREEN = [138, 201, 38];
const GREEN_BLUE_CRAYOLA = [25, 130, 196];
const ROYAL_PURPLE = [106, 76, 147];

const RANDOM_COLORS = [
    SIZZLING_RED,
    SUNGLOW,
    YELLOW_GREEN,
    GREEN_BLUE_CRAYOLA,
    ROYAL_PURPLE,
]

class Particle {
    constructor(x, y) {
        this.pos = createVector(x, y);
        this.vel = p5.Vector.random2D();
        this.vel.mult(random(1, 5));
        this.acc = createVector(0, 0);
        this.r = 2;
        this.lifetime = 255;
        this.color = ALABASTER;
    }

    applyForce(force) {
        this.acc.add(force);
        return this
    }

    setRandomColor() {
        this.color = RANDOM_COLORS[Math.floor(Math.random() * RANDOM_COLORS.length)];
        return this
    }

    edges() {
        if (this.pos.y >= height - this.r) {
            this.pos.y = height - this.r;
            this.vel.y *= -1;
        }

        if (this.pos.x >= width - this.r) {
            this.pos.x = width - this.r;
            this.vel.x *= -1;
        } else if (this.pos.x <= this.r) {
            this.pos.x = this.r;
            this.vel.x *= -1;
        }

        return this;
    }

    update() {
        // let mouse = createVector(mouseX, mouseY);
        // this.acc = p5.Vector.sub(mouse, this.pos);
        // this.acc.setMag(0.1);

        this.vel.add(this.acc);
        this.pos.add(this.vel);
        this.acc.set(0, 0);

        this.lifetime -= 5;
        return this;
    }

    finished() {
        return this.lifetime < 0;
    }

    show() {
        stroke(...this.color, this.lifetime);
        strokeWeight(2);
        fill(...this.color, this.lifetime);
        ellipse(this.pos.x, this.pos.y, this.r * 2);
        return this;
    }
}
