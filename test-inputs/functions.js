function isEven(number) {
  return number % 2 === 0;
}

class Car {
    constructor(make, model, year) {
      this.make = make;
      this.model = model;
      this.year = year;
      this.speed = 0;
    }
  
    accelerate() {
      this.speed += 10;
      console.log(`The ${this.make} ${this.model} is accelerating. Current speed: ${this.speed} km/h`);
    }
  
    brake() {
      if (this.speed >= 10) {
        this.speed -= 10;
        console.log(`The ${this.make} ${this.model} is braking. Current speed: ${this.speed} km/h`);
      } else {
        console.log(`The ${this.make} ${this.model} has already stopped.`);
      }
    }
  
    turnOn() {
      console.log(`The ${this.make} ${this.model} is now turned on.`);
    }
  
    turnOff() {
      if (this.speed === 0) {
        console.log(`The ${this.make} ${this.model} is now turned off.`);
      } else {
        console.log(`Please stop the ${this.make} ${this.model} before turning it off.`);
      }
    }
  }


function capitalizeFirstLetter(string) {
  return string.charAt(0).toUpperCase() + string.slice(1);
}