import React from 'react';


class Form extends React.Component {
    render() {
        return (
            <form onSubmit={this.props.getWeather}>
                <input type="text" name="latitude" placeholder="Latitude..." />
                <input type="text" name="longitude" placeholder="Longitude..." />
                <button>Get weather</button>
            </form>
        );
    }
}

export default Form;