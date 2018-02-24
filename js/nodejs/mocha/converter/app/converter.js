function rgbToHex (red, green, blue) {
    let redHex = red.toString(16);
    let greenHex = green.toString(16);
    let blueHex = blue.toString(16);

    return pad(redHex) + pad(greenHex) + pad(blueHex);
}

function pad(hex) {
    return hex.length === 1 ? '0' + hex : hex;
}

exports.rgbToHex = rgbToHex;