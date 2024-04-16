document.addEventListener('DOMContentLoaded', function() {
    const form = document.getElementById('join-lobby-form');
    const player = { // Player state
        id: null,
        money: 0,
        hand: [],
        currentBet: 0,
        roomCode: 0
    };

    form.addEventListener('submit', function(event) {
        event.preventDefault();

        const roomCode = document.getElementById('room-code').value;

        player.roomCode = parseInt(roomCode);

        // Data to send to the server
        const data = {
            code: parseInt(roomCode)
        };

        // Make a POST request to the server to join the lobby
        fetch('/api/join_lobby', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify(data)
        })
        .then(response => response.json())
        .then(lobby => {
            console.log('Joined lobby:', lobby);
            document.getElementById('join-lobby').style.display = 'none';
            document.getElementById('game-container').style.display = 'block';

            player.id = lobby.game.players[lobby.game.players.length - 1].id

            updateUI()
        })
        .catch(error => {
            console.error('Error joining lobby:', error);
            // Handle errors, e.g., display an error message to the user
        });
    });

    // Function to update UI based on player state
    function updateUI() {
        // Update pot display
        document.getElementById('pot').innerText = `Pot: $0`; // Update with actual pot amount

        // Update player's cards display
        const playerCards = document.getElementById('player-cards');
        playerCards.innerHTML = ''; // Clear previous cards
        player.hand.forEach(card => {
            const cardDiv = document.createElement('div');
            cardDiv.classList.add('card');
            cardDiv.textContent = card; // Assuming card is a string representation
            playerCards.appendChild(cardDiv);
        });

        // Update buttons based on player's current bet
        const raiseButton = document.getElementById('raise');
        raiseButton.innerText = `Raise (${player.currentBet * 2})`; // Example: Update raise button text with double the current bet amount

        // Add event listeners to buttons
        document.getElementById('check').addEventListener('click', function() {
            makeMove('Check');
        });

        document.getElementById('fold').addEventListener('click', function() {
            makeMove('Fold');
        });

        document.getElementById('call').addEventListener('click', function() {
            makeMove('Call');
        });

        document.getElementById('raise').addEventListener('click', () => {
            const raiseAmount = parseInt(document.getElementById('raise-amount').value);
            makeMove('Raise', raiseAmount);
        });
        // Update raise amount display when slider value changes
        document.getElementById('raise-amount').addEventListener('input', function() {
            document.getElementById('raise-value').innerText = this.value;
        });
    }

    // Function to send move to the server
    function makeMove(action, amount = null) {
        const message = {
            action: action,
            room_code: player.roomCode, // Assuming player has roomCode property
            player_id: player.id
        };

        if (action === 'Raise') {
            message.action = { "Raise": amount };
        }

        // Make a POST request to the server to make a move
        fetch('/api/make_move', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify(message)
        })
        .catch(error => {
            console.error('Error making move:', error);
            // Handle errors, e.g., display an error message to the user
        });
    }
   });

// Connects to the event stream for real time updates
function subscribe(uri) {
  var retryTime = 1;

  function connect(uri) {
    const events = new EventSource(uri);

    events.addEventListener("message", (ev) => {
      console.log("raw data", JSON.stringify(ev.data));
      console.log("decoded data", JSON.stringify(JSON.parse(ev.data)));
      const msg = JSON.parse(ev.data);

      // Handle the messages that come from the stream here
      handleEvent(msg);
    });

    events.addEventListener("open", () => {
      setConnectedStatus(true);
      console.log(`connected to event stream at ${uri}`);
      retryTime = 1;
    });

    events.addEventListener("error", () => {
      setConnectedStatus(false);
      events.close();

      let timeout = retryTime;
      retryTime = Math.min(64, retryTime * 2);
      console.log(`connection lost. attempting to reconnect in ${timeout}s`);
      setTimeout(() => connect(uri), (() => timeout * 1000)());
    });
  }

  connect(uri);
}

function updatePot(potAmount) {
    // Update the pot display with the new pot amount
    document.getElementById('pot').innerText = `Pot: $${potAmount}`;
}

function handleEvent(eventData) {
    console.log("Received event:", eventData);

    if (eventData.action.Raise !== null) {
        updatePot(eventData.action.Raise);
    }
}

subscribe("/api/events")
