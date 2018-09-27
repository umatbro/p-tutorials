import React from 'react';
import '../App.css';

class Weather extends React.Component {
    render() {
        if (this.props.error) {
            return <h3 className="error">{this.props.error}</h3>
        }

        const temperature = this.props.temperature;
        const temperatureMessage = 'Temperature: ' + (temperature !== null ? `${temperature} F` : `-`);

        const humidity = this.props.humidity;
        const humidityMessage = `Humidity: ${humidity ? humidity + ' %' : '-'}`

        const pressure = this.props.pressure;
        const pressureMessage = 'Pressure: ' + (pressure !== null ? `${pressure} hPa` : `-`);

        const description = this.props.description ? this.props.description : '';

        return (
            <div>
                <h3>{description}</h3>
                {temperatureMessage}<br />
                {humidityMessage}<br />
                {pressureMessage}<br />
            </div>
        );
    }
};

export default Weather;