/**
* Activation function.
* If it recieves positive - returns 1, else -1
*/
int sign(float n){
  if (n >= 0) return 1;
  else return -1;
}

class Perceptron {
  float[] weights = new float[2];
  float learningRate = 0.1;

  Perceptron() {
    // initialize weights randomly
    for (int i = 0; i < this.weights.length; i++){
      this.weights[i] = random(-1, 1);
    }
  }

  int guess(float[] inputs) {
    float sum = 0;
    for (int i = 0; i < weights.length; i++){
      sum += inputs[i] * this.weights[i];
    }
    int output = sign(sum);
    return output;
  }
  
  void train(float[] inputs, int target) {
    int guess = this.guess(inputs);
    int error = target - guess;
    for (int i = 0; i < this.weights.length; i++){
      this.weights[i] += error * inputs[i] * this.learningRate; 
    }
  }
  
  void printWeights(){
    println(this.weights[0] + " " + this.weights[1]);
  }
}