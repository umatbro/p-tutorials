float f(float x) {
    // y = mx + b
    return 0.3 * x + 0.2;
}

class Point {
    float x;
    float y;
    int bias = 1;
    int label;

    Point(float x, float y) {
        this.x = x;
        this.y = y;
        this.label = -1;
    }

    Point() {
        this.x = random(-1, 1);
        this.y = random(-1, 1);

        float lineY = f(this.x);

        if (this.y > lineY) this.label = 1;
        else this.label = -1;
    }

    float pixelX() {
      return map(this.x, -1, 1, 0, width);
    }
    float pixelY() {
        return map(this.y, -1, 1, height, 0);
    }

    void show() {
        stroke(0);
        if (label == 1){
            fill(255);
        } else {
            fill(0);
        }

        float px = this.pixelX();
        float py = this.pixelY();
        ellipse(px, py, 8, 8);
    }
}
