import React, { Component } from 'react';
import Title from './components/Title';
import Form from './components/Form';
import Weather from './components/Weather'

const API_KEY = '8dabfe815233a9733881c391720ab8e3';

class App extends Component {

  getWeather = async (event) => {
    event.preventDefault();
    const latitude = event.target.elements.latitude.value;
    const longitude = event.target.elements.longitude.value;
    const api_call = await fetch(`https://cors-anywhere.herokuapp.com/https://api.darksky.net/forecast/${API_KEY}/${latitude},${longitude}`);
   
    const data = await api_call.json();
    console.log(data);
    return data;
  }

  render() {
    return(
      <div>
        <Title />
        <Form getWeather={this.getWeather} />
        <Weather />
      </div>
    );
  }
}

export default App;