let roomCode = 0;
let pot = 0;
let player = { // Player state
    id: null,
    money: 0,
    hand: [],
    currentBet: 0,
    roomCode: 0,
    isMyTurn: false
}; // TODO: make not global variables

document.addEventListener('DOMContentLoaded', function() {
    const join_form = document.getElementById('join-lobby-form');
    const create_form = document.getElementById('create-lobby-form');
    // Update buttons based on player's current bet
    const raiseButton = document.getElementById('raise');

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

    document.getElementById('start').addEventListener('click', function() {
        makeMove('Start');
    });

    document.getElementById('raise').addEventListener('click', () => {
        const raiseAmount = parseInt(document.getElementById('raise-amount').value);
        makeMove('Raise', raiseAmount);
    });
    // Update raise amount display when slider value changes
    document.getElementById('raise-amount').addEventListener('input', function() {
        document.getElementById('raise-value').innerText = this.value;
    });

    create_form.addEventListener('submit', function(event) {
        event.preventDefault();
        roomCode = document.getElementById('create-room-code').value;
        player.roomCode = parseInt(roomCode);

        const data = {
            code: player.roomCode
        };

        fetch('/api/create_lobby', {
            method: 'POST',
            headers: {
                'Content-Type' : 'application/json'
            },
            body: JSON.stringify(data)
        })

        .then(response => response.json())
        .then(lobby => {
            console.log('Joined lobby:', lobby);
            document.getElementById('join-lobby').style.display = 'none';
            document.getElementById('game-container').style.display = 'block';

            player.id = lobby.game.players[lobby.game.players.length - 1].id
            player.money = lobby.game.settings.starting_money;

            updateUI()
        })
        .catch(error => {
            console.error('Error joining lobby:', error);
            // Handle errors, e.g., display an error message to the user
        });
    });

    join_form.addEventListener('submit', function(event) {
        event.preventDefault();

        roomCode = document.getElementById('join-room-code').value;

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
            player.money = lobby.game.settings.starting_money;

            updateUI()
        })
        .catch(error => {
            console.error('Error joining lobby:', error);
            // Handle errors, e.g., display an error message to the user
        });
    });

    // Function to update UI based on player state
    function updateUI() {
        // Update information
        document.getElementById('pot').innerText = `Pot: $${pot}`; // Update with actual pot amount
        document.getElementById('money').innerText = `Money: $${player.money}`;
        document.getElementById('current-bet').innerText = `Bet: $${player.currentBet}`;
        document.getElementById('is-my-turn').innerText = `Is My Turn: ${player.isMyTurn}`;

        // Update player's cards display
        const playerCards = document.getElementById('player-cards');
        playerCards.innerHTML = ""; // Clear previous cards in new round
        playerHandToString(player.hand).forEach(card => {
            const cardDiv = document.createElement('div');
            cardDiv.classList.add('card');
            cardDiv.textContent = card;
            playerCards.appendChild(cardDiv);
        });

    }

    // Function to send move to the server
    function makeMove(action, amount = null) {
        if ((!player.isMyTurn) && player.id !== 0 && player.money === 0 && player.hand === []) return;

        const message = {
            action: action,
            room_code: player.roomCode,
            player_id: player.id
        };

        if (action === 'Raise') {
            message.action = { "Raise": amount };
        }

        fetch('/api/make_move', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify(message)
        })
        .catch(error => {
            console.error('Error making move:', error);
        });

        player.isMyTurn = false;
    }

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

        function setConnectedStatus(status) {

        }

        function updatePot(potAmount) {
            // Update the pot display with the new pot amount
            document.getElementById('pot').innerText = `Pot: $${potAmount}`;
        }

        function updateGameValues(game) {
            // Update player values
            let server_player = game.players[player.id];
            console.log("server player money: " + server_player.money + " server player bet: " + server_player.current_bet);

            player.money = server_player.money;
            player.currentBet = server_player.current_bet;

            // Update game values
            pot = game.pot;
        }

        function handleEvent(eventData) {
            console.log("Received event:", eventData);

            console.log("This message is for player: ", eventData.player_id, " and room: ", eventData.room_code);
            if (eventData.room_code !== roomCode) {
                //return;
            }

                updateGameValues(eventData.game);

            if (player.id === eventData.player_id && eventData.action.DealPlayer !== undefined) {
                player.hand = eventData.action.DealPlayer;
                console.log("Player Hand: ", player.hand);
            } else if (eventData.action === "YourTurn" && player.id === eventData.player_id) {
                console.log("it is now this players turn");
                player.isMyTurn = true;
            }

            updateUI();
        }

    function playerHandToString(hand) {
        stringHand = [];
        hand.forEach(card =>  {
            if (card.value === 1) stringHand.push("A " + card.suit);
            else if (card.value === 11) stringHand.push("J " + card.suit);
            else if (card.value === 12) stringHand.push("Q " + card.suit);
            else if (card.value === 13) stringHand.push("K " + card.suit);
            else stringHand.push(card.value + " " + card.suit);
        });

        return stringHand;
    }

        subscribe("/api/events");
   });

// Connects to the event stream for real time updates
