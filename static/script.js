document.addEventListener('DOMContentLoaded', function() {
    const form = document.getElementById('join-lobby-form');

    form.addEventListener('submit', function(event) {
        event.preventDefault();

        const roomCode = document.getElementById('room-code').value;

        // Data to send to the server
        const data = {
            code: parseInt(roomCode) // Assuming room code is a number
        };

        // Make a POST request to the server to join the lobby
        fetch('/join_lobby', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify(data)
        })
        .then(response => response.json())
        .then(lobby => {
            console.log('Joined lobby:', lobby);
            // Handle the response from the server
            // For example, update the UI based on the lobby data
        })
        .catch(error => {
            console.error('Error joining lobby:', error);
            // Handle errors, e.g., display an error message to the user
        });
    });
});
