import React, { Component } from 'react';
import Title from './components/Title';
import Form from './components/Form';
import Weather from './components/Weather'

const API_KEY = '8dabfe815233a9733881c391720ab8e3';

class App extends Component {
  state = {
    temperature: null,
    humidity: null,
    pressure: null,
    description: null,
    error: null,
  }

  getWeather = async (event) => {
    event.preventDefault();
    const latitude = event.target.elements.latitude.value;
    const longitude = event.target.elements.longitude.value;
    const api_call = await fetch(`https://cors-anywhere.herokuapp.com/https://api.darksky.net/forecast/${API_KEY}/${latitude},${longitude}`);
    if (!(latitude && longitude)) {
      return;
    }
    const data = await api_call.json();

    if (data.code >= 400) {
      this.setState({error: data.error});
      return;
    }
    this.setState({
      temperature: data.currently.temperature,
      humidity: data.currently.humidity,
      pressure: data.currently.pressure,
      description: data.currently.summary,
    });

    return data;
  }

  render() {
    return(
      <div>
        <Title />
        <Form getWeather={this.getWeather} />
        <Weather 
          temperature={this.state.temperature} 
          humidity={this.state.humidity}
          pressure={this.state.pressure}
          description={this.state.description}
          error={this.state.error}
        />
      </div>
    );
  }
}

export default App;