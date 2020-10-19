exports.main = async (event) => {
    const now = new Date();
    const response = {
        statusCode: 200,
        body: JSON.stringify({
            time: now,
            event,
        }),
    };
    return response;
};
