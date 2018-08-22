import React from 'react';
import ReactDOM from 'react-dom';
import './index.css';


function Square(props) {
    const cls = `square${props.isWinning ? ' winning' : ''}`;
    return (
        <button className={cls} onClick={props.onClick}>
            {props.value}
        </button>
    );
}

class Board extends React.Component {
    renderSquare(i) {
        return (
            <Square
                value={this.props.squares[i]}
                isWinning={this.props.winningSquares.includes(i)}
                onClick={() => this.props.onClick(i)}
            />
        );
    }

    render() {
        return (
            <div>
                <div className="board-row">
                    {this.renderSquare(0)}
                    {this.renderSquare(1)}
                    {this.renderSquare(2)}
                </div>
                <div className="board-row">
                    {this.renderSquare(3)}
                    {this.renderSquare(4)}
                    {this.renderSquare(5)}
                </div>
                <div className="board-row">
                    {this.renderSquare(6)}
                    {this.renderSquare(7)}
                    {this.renderSquare(8)}
                </div>
            </div>
        );
    }
}

class Game extends React.Component {
    constructor(props) {
        super(props);
        this.state = {
            history: [{
                squares: Array(9).fill(null),
                lastPositionSelected: Array(2).fill(null),
            }],
            xIsNext: true,
            stepNumber: 0,
            currentlyViewedMove: 0,
        };
    }

    handleClick(i) {
        const history = this.state.history.slice(0, this.state.stepNumber + 1);
        const current = history[history.length - 1];
        const squares = current.squares.slice();
        if (calculateWinner(squares) || squares[i]) {
            return;
        }

        let coords = posToCoords(i);

        squares[i] = this.state.xIsNext ? 'X' : 'O';
        this.setState({
            history: history.concat([{
                squares: squares, 
                lastPositionSelected: coords,
            }]),
            stepNumber: history.length,
            xIsNext: !this.state.xIsNext,
            currentlyViewedMove: this.state.currentlyViewedMove + 1,
        });
    }

    jumpTo(moveNr) {
        this.setState({
            stepNumber: moveNr,
            xIsNext: (moveNr % 2) === 0,
            currentlyViewedMove: moveNr,
        });
    }

    render() {
        const history = this.state.history;
        const current = history[this.state.stepNumber];
        const winner = calculateWinner(current.squares);
        const winningSquares = getWinningSquares(current.squares);

        const moves = history.map((step, move) => {
            const desc = move ?
                'Go to move #' + move :
                'Go to game start';
            
            let [col, row] = step.lastPositionSelected;
            const posMsg = col && row ? `(${col}, ${row})` : '';
            const liContent = (
                <li key={move}>
                    <button onClick={() => this.jumpTo(move)}>{desc}</button>
                    {posMsg}
                </li>
            );

            return move === this.state.currentlyViewedMove ? (
                <li key={move}>
                    <b>
                    <button onClick={() => this.jumpTo(move)}>{desc}</button>
                    {posMsg}
                    </b>
                </li>
            ) : liContent;
        });

        let status;

        if (winner) {
            status = `Winner: ${winner}`;
        } else if (areAllSquaresFilled(current.squares)) {
            status = 'Draw!'
        } else {
            status = `Next player: ${this.state.xIsNext ? 'X' : 'O'}`
        }

        return (
            <div className="game">
                <div className="game-board">
                    <Board
                        squares={current.squares}
                        winningSquares={winningSquares}
                        onClick={(i) => this.handleClick(i)}
                    />
                </div>
                <div className="game-info">
                    <div>{status}</div>
                    <ol>{moves}</ol>
                </div>
            </div>
        );
    }
}

// ========================================

ReactDOM.render(
    <Game/>,
    document.getElementById('root')
);


function calculateWinner(squares) {
    const lines = [
        [0, 1, 2],
        [3, 4, 5],
        [6, 7, 8],
        [0, 3, 6],
        [1, 4, 7],
        [2, 5, 8],
        [0, 4, 8],
        [2, 4, 6],
    ];

    for (let [a, b, c] of lines) {
        if (squares[a] && squares[a] === squares[b] && squares[a] === squares[c]) {
            return squares[a];
        }
    }
    return null;
}

function getWinningSquares(squares) {
    const lines = [
        [0, 1, 2],
        [3, 4, 5],
        [6, 7, 8],
        [0, 3, 6],
        [1, 4, 7],
        [2, 5, 8],
        [0, 4, 8],
        [2, 4, 6],
    ];

    for (let [a, b, c] of lines) {
        if (squares[a] && squares[a] === squares[b] && squares[a] === squares[c]) {
            return [a, b, c];
        }
    }
    return [];
}

function areAllSquaresFilled(squares) {
    for (let square of squares) {
        if (!square) return false;
    }
    return true;
}

/**
 * @param {Number} pos position in the array to be translated
 * @returns 2-element array with col, row
 */
function posToCoords(pos) {
    return [(pos) % 3 + 1, Math.floor(pos / 3) + 1];
}
